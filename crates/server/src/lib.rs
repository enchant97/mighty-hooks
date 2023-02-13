use actix_web::{middleware, HttpResponse};
use actix_web::{middleware::Logger, post, web, App, HttpRequest, HttpServer};
use mighty_hooks_config::Config;
use mighty_hooks_core::{signing::verify_hmac_sha256, tls::load_rustls_config};

/// Make domain + path from request data
fn get_in_path(path: String, request: &HttpRequest) -> Option<String> {
    let host = match request.headers().get("Host") {
        Some(host) => match host.is_empty() {
            true => return None,
            false => host.to_str().unwrap(),
        },
        None => {
            return None;
        }
    };
    Some(format!("{}/{}", host, path))
}

/// Get the real client ip from the request
fn get_client_ip(behind_proxy: bool, request: &HttpRequest) -> Option<String> {
    match behind_proxy {
        true => match request.connection_info().realip_remote_addr() {
            Some(ip) => Some(ip.to_owned()),
            None => None,
        },
        false => match request.connection_info().peer_addr() {
            Some(ip) => Some(ip.to_owned()),
            None => None,
        },
    }
}

/// Get the content type from the request validating it is not empty
fn get_content_type(request: &HttpRequest) -> Option<String> {
    match request.headers().get("Content-Type") {
        Some(content_type) => match content_type.is_empty() {
            true => None,
            false => Some(content_type.to_str().unwrap().to_string()),
        },
        None => None,
    }
}

fn get_signature_256(request: &HttpRequest) -> Option<String> {
    match request.headers().get("X-Hub-Signature-256") {
        Some(signature) => match signature.is_empty() {
            true => None,
            false => Some(signature.to_str().unwrap().to_string()),
        },
        None => None,
    }
}

#[post("/{path:.*}")]
async fn post_webhook(
    config: web::Data<Config>,
    path: web::Path<String>,
    request: HttpRequest,
    body: web::Bytes,
) -> HttpResponse {
    // Get the path from the request data ensuring it is valid
    let in_path = match get_in_path(path.into_inner(), &request) {
        Some(in_path) => in_path,
        None => {
            return HttpResponse::BadRequest().finish();
        }
    };
    // Get the real client ip
    let client_ip = match get_client_ip(config.behind_proxy, &request) {
        Some(client_id) => client_id,
        None => {
            log::error!("failed to get client ip");
            return HttpResponse::InternalServerError().finish();
        }
    };
    // Try and find hook for path
    let hook = match config.hooks.get(&in_path) {
        Some(hook) => hook,
        None => {
            // No hook found for path
            log::info!("{} trigged nonexistent hook \"{}\"", client_ip, in_path);
            return HttpResponse::NotFound().finish();
        }
    };
    // Validate content type
    match get_content_type(&request) {
        Some(content_type) => {
            if content_type != hook.r#in.content_type {
                log::info!(
                    "{} trigged hook \"{}\" with unexpected content type: {}",
                    client_ip,
                    in_path,
                    content_type
                );
                return HttpResponse::BadRequest().finish();
            }
        }
        None => {
            log::info!(
                "{} trigged hook \"{}\" without content type",
                client_ip,
                in_path
            );
            return HttpResponse::BadRequest().finish();
        }
    };
    // Validate signature-256 if enabled
    if let Some(secret_256) = &hook.r#in.secret_256 {
        match get_signature_256(&request) {
            Some(signature) => {
                if !verify_hmac_sha256(secret_256, &body, &signature) {
                    log::info!(
                        "{} trigged hook \"{}\" with invalid signature",
                        client_ip,
                        in_path
                    );
                    return HttpResponse::BadRequest().finish();
                }
            }
            None => {
                log::info!(
                    "{} trigged hook \"{}\" without signature",
                    client_ip,
                    in_path
                );
                return HttpResponse::BadRequest().finish();
            }
        };
    }

    log::info!("{} trigged hook successfully \"{}\"", client_ip, in_path);
    HttpResponse::NoContent().finish()
}

pub async fn run_server(config: &Config) {
    let config = config.clone();
    let https_config = config.https.clone();
    let bind = (config.host.to_owned(), config.port);
    // Create server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("Server", "Mighty Hooks")))
            .app_data(web::Data::new(config.clone()))
            .service(post_webhook)
    });
    // Bind to address & port using either http or https
    let bound_server = match https_config {
        Some(https_config) => {
            let cert_config = load_rustls_config(&https_config.cert, &https_config.key);
            server.bind_rustls(bind, cert_config)
        }
        None => server.bind(bind),
    };
    // Run server
    bound_server
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Failed to run server");
}
