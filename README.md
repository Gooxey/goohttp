# goohttp

This library provides macros for easy [router](https://docs.rs/axum/latest/axum/routing/struct.Router.html) definition and, if enabled, an embedded compatible synchronous HttpServer, that uses the [axum router](https://docs.rs/axum/latest/axum/routing/struct.Router.html) for route management. This crate is **NOT** no_std compatible. 

![Crates.io](https://img.shields.io/crates/v/goohttp) ![Crates.io](https://img.shields.io/crates/l/goohttp)

## Features

By default this library only provides two macros for more convenient router creation.

- `esp` -> This feature enables the embedded compatible [HttpServer](./src/http_server.rs).

## Additional info for use in embedded development

### stack overflow in pthread

Because this HttpServer uses async functions and the [spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) function from tokio, you may get this error:

```text
***ERROR*** A stack overflow in task pthread has been detected.
```

Fortunately, all you have to do is adjust the following value in your `sdkconfig.defaults` which should have been generated when you used
[this ESP32 template](https://github.com/esp-rs/esp-idf-template):

```text
# 10000 has worked for my project so far but you can probably set it far lower

CONFIG_PTHREAD_TASK_STACK_SIZE_DEFAULT=10000
```

### Reducing binary size

When using this library and other libraries, you may encounter another problem: you run out of memory. To fix this, you need to change some compiler settings. For that, I
would suggest to have a look at [this](https://github.com/johnthagen/min-sized-rust) and [this guide](https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html).
