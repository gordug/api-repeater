use std::net::Ipv4Addr;
use warp::{http::Response, Filter, path::FullPath, hyper::HeaderMap};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    let format =
        tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_ansi(true)
            .compact();

    tracing_subscriber::fmt()
        .event_format(format)
        .init();
    
    let routes = warp::any()
        .and(warp::body::bytes())
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .map(|body: warp::hyper::body::Bytes, path: FullPath, headers: HeaderMap| {
            let body = String::from_utf8(body.to_vec()).unwrap();
            let path = path.as_str().to_string();
            let headers = headers.iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect::<Vec<(String, String)>>();
            let response = Response::builder()
                .status(200)
                .body(format!("{{\"body\": \"{}\", \"path\": \"{}\", \"headers\": {:?}}}", body, path, headers))
                .unwrap();
            tracing::info!("response: {:?}", response);
            response
        });

        warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, 3031)).await;
}