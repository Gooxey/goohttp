//! This module exports two macros used to simplify the route setup of [`axum Routers`](axum::Router):
//! - [`impl_route_group`](crate::impl_route_group) -> Create a new group of routes.
//! - [`impl_routes`](crate::impl_routes) -> Create new routes.

// TODO add index to impl_routes!

/// Use this macro to create a new group of routes. \
/// To create new routes use the [`impl_routes`](crate::impl_routes) macro.
///
/// # Example
///
/// This macro can only be used to nest routes created by the [`impl_routes`](crate::impl_routes) macro.
///
/// .../mcserver/info/mod.rs
/// ```
/// impl_routes! {
///     info {
///         get_list, get;
///         get_log, get
///     }
/// }
/// ```
/// .../mcserver/actions/mod.rs
/// ```
/// impl_routes! {
///     actions {
///         start, get;
///         stop, get
///     }
/// }
/// ```
/// .../mcserver/mod.rs
/// ```
/// impl_route_groups! {
///     mcserver {  // This must be the name of the module. Otherwise, this module will be useless to other modules that use this macro.
///         info;
///         actions;
///     }
/// }
/// ```
///
/// Once this macro has been expanded, you will get the following code.
///
/// ```
/// mod info;
/// mod actions;
///
/// pub fn mcserver() -> goohttp::axum::Router {
///     goohttp::axum::Router::new()
///         .nest("/info", info::info())
///         .nest("/actions", actions::actions())
/// }
/// ```
#[macro_export]
macro_rules! impl_route_group {
    {
        $group_id:tt {
            $ ($group:tt); *;
        }
    } => {
        $ (
            mod $group;
        ) *

        pub fn $group_id() -> $crate::axum::Router {
            $crate::axum::Router::new()
                $ (
                    .nest(
                        &format!("/{}", std::stringify!($group)),
                        $group::$group()
                    )
                ) *
        }
    };
}
/// Use this macro to create new routes. \
/// To create a new group of routes, use the [`impl_route_group`](crate::impl_route_group) macro.
///
/// # Example
///
/// Each route requires an associated function, which must be declared in its own module.
///
/// .../info/index.rs
/// ```
/// pub async fn index() -> impl IntoResponse {
///     // Implementation of this function
/// }
/// ```
///
/// .../info/get_log.rs
/// ```
/// pub async fn get_log(Path(mcserver): Path<String>) -> impl IntoResponse {
///     // Implementation of this function
/// }
/// ```
///
/// The parent module defines a router that can be nested by other routers. This router will then implement all specified routes and their associated functions.
///
/// .../info/mod.rs
/// ```
/// impl_routes! {
///     info {
///         index, get;     // Any function called indexed will be interpreted as the root route `/`.
///         index, get, ":username/:password";
///         get_log, get, ":mcserver";    // The second argument `get` can also be replaced by any other function from `axum::routing::*`.
///     }
/// }
/// ```
///
/// This macro allows you to skip writing the code below:
///
/// ```
/// use goohttp::axum::*;
///
/// mod index;
/// mod get_log;
///
/// pub fn info() -> Router {
///     Router::new()
///         .route("/", get(index::index))
///         .route("/get_log/:mcserver", get(get_log::get_log))
/// }
/// ```
#[macro_export]
macro_rules! impl_routes {
    {
        $group_id:tt {
            $ (
                $route:tt,
                $request_type:tt
                $(, $parameters:expr)?
            ); *
            ;
        }
    } => {
        use $crate::axum::*;
        $ ( mod $route; ) *

        pub fn $group_id() -> Router {
            Router::new()
                $ (
                    .route(
                        & {
                            let route;
                            if std::stringify!($route) == "index" {
                                route = "/".to_string();
                            } else {
                                route = format!("/{}", std::stringify!($route));
                            }

                            $ (
                                let mut route = route;
                                route.push_str(&format!("/{}", $parameters));
                            ) ?

                            route
                        },
                        $request_type($route::$route)
                    )
                ) *
        }
    };
}
