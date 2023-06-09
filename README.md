# goohttp

This library provides macros for easy [router](https://docs.rs/axum/latest/axum/routing/struct.Router.html) definition and, if enabled, an embedded compatible synchronous HttpServer, that uses the [axum router](https://docs.rs/axum/latest/axum/routing/struct.Router.html) for route management.

## Features

By default this library only provides two macros for more convenient router creation.

- `esp` -> This feature enables the embedded compatible [HttpServer](./src/http_server.rs).
