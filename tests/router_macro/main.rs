use goohttp::router;
use hyper::{
    body::HttpBody,
    service::Service,
    Body,
    Request,
};

#[tokio::test]
async fn main() {
    let mut website = website();

    let index_response = website
        .call(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap()
        .data()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        std::str::from_utf8(&index_response.to_vec()).unwrap(),
        "index"
    );

    let remaining_response = website
        .call(
            Request::get("/this_route_does_not_exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .data()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        std::str::from_utf8(&remaining_response.to_vec()).unwrap(),
        "called remaining with the route `this_route_does_not_exist`"
    );

    let say_hello_response = website
        .call(
            Request::get("/api/say_hello/MySuperAwesomeMCManageClient")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .data()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        std::str::from_utf8(&say_hello_response.to_vec()).unwrap(),
        "said hello from MySuperAwesomeMCManageClient"
    );

    let say_hello_caller_sender_response = website.call(Request::get("/api/say_hello_caller_sender/MySuperAwesomeMCManageClient/MyMoreAwesomeMCManageClient").body(Body::empty()).unwrap()).await.unwrap().data().await.unwrap().unwrap();
    assert_eq!(
        std::str::from_utf8(&say_hello_caller_sender_response.to_vec()).unwrap(),
        "said hello from MySuperAwesomeMCManageClient to MyMoreAwesomeMCManageClient"
    );
}

router! {
    website {
        index, get;
        remaining, get;
        api
    }
}
