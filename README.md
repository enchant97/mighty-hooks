# Mighty Hooks
A lightweight & fast stateless webhook relay server.

> Currently in early stages of development. **NOT** suitable for production.


## Features
- Relay/Resend Webhooks
- Multi domain & route support
- Stateless (no database or temporary files)
- Configured via a single yaml file
- Supports HMAC-256 validating and signing
- Header extraction
- Webhook rewording (receive one hook and send a different)
  - Templating via [Tera](https://tera.netlify.app/)
  - JSON body access in template
  - Add headers
- HTTPS support


## Future
- Authentication Bearer token support (both send & receive)


## License
This project is Copyright (c) 2023 Leo Spratt, licences shown below:

Code

    AGPL-3 or any later version. Full license found in `LICENSE.txt`

Documentation

    FDLv1.3 or any later version. Full license found in `docs/LICENSE.txt`
