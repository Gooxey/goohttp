//! This module provides an [`HttpServer`] that is compatible with embedded systems like the ESP32, but also supports many of the popular HttpServer features.

use std::{
    io::{
        self,
        BufRead,
        BufReader,
        ErrorKind,
        Write,
    },
    net::{
        SocketAddr,
        TcpListener,
        TcpStream,
        ToSocketAddrs,
    },
};

use axum::Router;
use goolog::*;
use http::{
    Method,
    Uri,
};
use hyper::{
    body::HttpBody,
    service::Service,
    Body,
    Request,
    Response,
};
use tokio::{
    spawn,
    task::JoinHandle,
};

/// When developing for embedded systems, you cannot, as of now, use asynchronous TcpListeners and thus
/// [one of the most popular HttpServers](https://docs.rs/hyper/0.14.26/hyper/server/struct.Server.html). But this does not immediately mean that you have to miss out on all
/// of the features provided by [`axum`]. The solution is to do everything with a synchronous TcpListener.
///
/// # Requirements
///
/// Because this HttpServer uses async functions and the [`spawn`] function from tokio, you may get this error:
/// ```bash
/// ***ERROR*** A stack overflow in task pthread has been detected.
/// ```
/// Fortunately, all you have to do is adjust the following value in your `sdkconfig.defaults` which should have been generated when you used
/// [this ESP32 template](https://github.com/esp-rs/esp-idf-template):
/// ```text
/// CONFIG_PTHREAD_TASK_STACK_SIZE_DEFAULT=10000    # 10000 has worked for my project so far but you can probably set it far lower
/// ```
///
/// # Reducing binary size
///
/// When using this library and other libraries, you may encounter another problem: you run out of memory. To fix this, you need to change some compiler settings. For that, I
/// would suggest to have a look at [`this`](https://github.com/johnthagen/min-sized-rust) and [`this guide`](https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html).
///
/// # How to use this HttpServer
///
/// First, you will need a [`Router`]. You can use the macros from this library:
/// ```
/// // In this example, we will create a simple router with one route
/// impl_routes! {
///     router {
///         get_list, get;  // The route will be called get_list and will only accept get requests.
///                         // For more details on the implementation of this route handler, see this macro documentation.
///     }
/// }
/// ```
/// After creating a router, we can bind and serve our HttpServer:
/// ```
/// let router = router(); // The macro above has only generated a function.
///                        // Only after calling it, we can get our router.
///
/// let http_server = HttpServer::bind("0.0.0.0:80");
/// http_server.serve(router);
/// ```
pub struct HttpServer {
    /// The address that the internal TcpListener will use.
    addr: SocketAddr,
    /// The main task of this HttpServer.
    main_task: Option<JoinHandle<()>>,
    /// The name of this HttpServer, which gets used in log messages.
    name: String,
}
impl HttpServer {
    /// Create and set an address for a new HttpServer.
    pub fn bind<A: ToSocketAddrs>(addr: A, name: Option<&str>) -> Self {
        let final_name;
        if let Some(name) = name {
            final_name = name.to_string();
        } else {
            final_name = "HttpServer".to_string();
        }

        Self {
            addr: addr
                .to_socket_addrs()
                .unwrap_or_else(|_| {
                    fatal!(
                        final_name,
                        "The specified address could not be converted to `std::net::SocketAddr`."
                    );
                })
                .next()
                .unwrap_or_else(|| {
                    fatal!(final_name, "Could not find an address.");
                }),
            main_task: None,
            name: final_name,
        }
    }
    /// This method will close the internal TCPListener and all of its connections by killing the task they are running on. \
    /// If this HttpServer was already offline, this method will do nothing.
    pub async fn shutdown(&self) {
        if let Some(main_task) = &self.main_task {
            main_task.abort();

            info!(self.name, "Stopped.");
        }
    }

    /// Serve the given [`HttpServer`] with the given [`Router`]. \
    /// This function is non-blocking.
    ///
    /// # Errors
    ///
    /// An error is returned if the TcpListener failed to bind to the given address.
    pub fn serve(&mut self, router: Router) -> io::Result<()> {
        info!(self.name, "Starting...");

        let tcp_listener = TcpListener::bind(self.addr)?;

        info!(self.name, "Started! Now listening for clients...");

        let name = self.name.clone();
        let main_task = spawn(async move {
            for connection in tcp_listener.incoming() {
                match connection {
                    Ok(client) => {
                        spawn(Self::handler(client, router.clone()));
                    }
                    Err(error) => {
                        error!(name, "Could not accept an incoming connection. It will be ignored. Error: {error}");
                        continue;
                    }
                }
            }
        });

        self.main_task = Some(main_task);

        Ok(())
    }
    /// The handler of each client.
    async fn handler(mut client: TcpStream, mut router: Router) -> io::Result<()> {
        /// Get a [`Response`] from the given [`Router`] based on the given [`Request`].
        async fn request_to_response(
            req: Request<Body>,
            router: &mut Router,
        ) -> Result<Response<Vec<u8>>, axum::http::Error> {
            Response::builder().body({
                let result = router
                    .call(req)
                    .await
                    .expect("This should not fail since the error is of kind `Infallible`.")
                    .data()
                    .await;

                let mut data = vec![];
                if let Some(Ok(val)) = result {
                    data = val.to_vec();
                }

                data
            })
        }
        /// Convert a [`Response`] to a vec of bytes.
        fn response_to_bytes(response: Response<Vec<u8>>) -> Vec<u8> {
            let (parts, mut body) = response.into_parts();
            let mut http_response = vec![];

            // status line
            http_response.append(
                &mut format!(
                    "{:?} {} {}\r\n",
                    parts.version,
                    parts.status.as_u16(),
                    parts
                        .status
                        .canonical_reason()
                        .expect("Every status code should have a canonical_reason!")
                )
                .as_bytes()
                .to_vec(),
            );

            // headers
            for (header_name, header_value) in parts.headers {
                http_response.append(
                    &mut format!(
                        "{}: ",
                        header_name.expect("Every header should have a name!")
                    )
                    .as_bytes()
                    .to_vec(),
                );
                http_response.append(&mut header_value.as_bytes().to_vec());
                http_response.append(&mut b"\r\n".to_vec());
            }

            // body
            http_response.append(&mut b"\r\n".to_vec());
            http_response.append(&mut body);

            http_response
        }

        let buf_reader = BufReader::new(&mut client);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.expect("Each request should be convertible to a String.")) // Maybe this should just cancel the connection
            .take_while(|line| !line.is_empty())
            .collect();

        if http_request.is_empty() {
            return Err(ErrorKind::InvalidData.into());
        }

        let mut head_line = http_request[0].split(' ');
        let method;
        let uri;
        if let Some(val) = head_line.next() {
            if let Ok(val) = Method::from_bytes(val.as_bytes()) {
                method = val;
            } else {
                return Err(ErrorKind::InvalidData.into());
            }
        } else {
            return Err(ErrorKind::InvalidData.into());
        }
        if let Some(val) = head_line.next() {
            if let Ok(val) = val.parse::<Uri>() {
                uri = val;
            } else {
                return Err(ErrorKind::InvalidData.into());
            }
        } else {
            return Err(ErrorKind::InvalidData.into());
        }

        let request;
        if let Ok(val) = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
        {
            request = val;
        } else {
            return Err(ErrorKind::InvalidData.into());
        }

        let response;
        if let Ok(val) = request_to_response(request, &mut router).await {
            response = val;
        } else {
            return Err(ErrorKind::InvalidData.into());
        }

        if client.write_all(&response_to_bytes(response)).is_err() {}

        Ok(())
    }
}
