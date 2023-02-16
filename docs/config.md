# Configuration

## Environment Variables

> Names must be prefixed with: "MIGHTY_HOOKS_"

| Name        | Description                   | Default       |
| :---------- | :---------------------------- | :------------ |
| LOG_LEVEL   | The logging level             | INFO          |
| CONFIG_PATH | Where to load the YAML config | ./config.yaml |

## YAML File

> Fields commented with a `~` are optional

```yaml
# file: config.yml

# ip that server will bind on
host: 0.0.0.0
# Port that server will bind on
port: 8000
# ~ Set to "true" when behind a trusted reverse-proxy
behind_proxy: false
# ~ Run the server with https instead of http
https:
  # public certificate
  cert: /certs/example.com.crt
  # private key (in PKCS8 format)
  key: /certs/example.com.key
# The configured hooks
hooks:
  # A hook definition, given as the full
  # url (without scheme) to match against
  hooks.example.com/hello:
    # Define settings for receiving
    in:
      # Expected content type to receive,
      # also controls available content values in reword template
      content_type: "application/json"
      # ~ Validate a `x-hub-signature-256` signed webhook
      secret_256: "my_secret"
    # Define settings for sending/relaying
    out:
      # YAML array so one webhook can be send to multiple places
      -
        # Where to send webhook (with scheme)
        href: "http://internal.example.com/hello"
        # ~ Sign the body using `x-hub-signature-256`
        secret_256: "my_secret"
        # ~ Pass through specific errors
        # ~ Will never keep `x-hub-signature-256` or `x-hub-signature`
        keep_headers: ["x-example-header"]
        # ~ Set new body of webhook
        reword:
          # Content type of output
          content_type: "application/json"
          # The new content to set for body, supporting tera templating
          content: |
            {
                "message": "Hello World!",
                "secret-stat": "{{ content.json["stat"] }}",
                "user-agent": "{{ content.headers["user-agent"] }}"
            }
```
