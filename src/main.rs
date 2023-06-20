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
            .with_ansi(true);

    tracing_subscriber::fmt()
        .event_format(format)
        .init();
    
    let routes = warp::any()
        .and(warp::path("body"))
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            let body = String::from_utf8(body.to_vec()).unwrap();
            let response = Response::builder()
                .status(200)
                .body(body)
                .unwrap();
            tracing::info!("response: {:#?}", response);
            response
        })
        .or(warp::path("path")
            .and(warp::path::full())
            .map(|path: FullPath| {
                let path = path.as_str().to_string();
                let response = Response::builder()
                    .status(200)
                    .body(path)
                    .unwrap();
                tracing::info!("response: {:#?}", response);
                response
            })
        )
        .or(warp::path("headers")
            .and(warp::header::headers_cloned())
            .map(|headers: HeaderMap| {
                let headers = headers.iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect::<Vec<(String, String)>>();
                let response = Response::builder()
                    .status(200)
                    .body(format!("{:?}", headers))
                    .unwrap();
                tracing::info!("response: {:#?}", response);
                response
            })
        )
        .or(warp::any()
            .and(warp::body::bytes())
            .and(warp::path::full())
            .and(warp::header::headers_cloned())
            .map(|body: warp::hyper::body::Bytes, path: FullPath, headers: HeaderMap| {
                let body = String::from_utf8(body.to_vec()).unwrap();
                let path = path.as_str().to_string();
                let headers = headers.iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect::<Vec<(String, String)>>();
                let response = Response::builder()
                    .status(200)
                    .body(format!("{{\"path\": \"{}\",\n\r \"headers\": {:?},\n\r \"body\": \"{}\"}}", path, headers, body))
                    .unwrap();
                tracing::info!("response: {:?}", response);
                response
            })
        );
        tracing::info!("Send a request to http://localhost:3031");
        tracing::info!("Try http://localhost:3031/body to see the body");
        tracing::info!("Try http://localhost:3031/path to see the path");
        tracing::info!("Try http://localhost:3031/headers to see the headers");
        tracing::info!("Try http://localhost:3031/ or any other path, to see the path, headers and body");
        tracing::info!("Starting main thread");
        tracing::info!("Press Enter to exit");
        tokio::spawn(async move {
        // watch for enter key, confirm stop and continue if not
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "" {
                tracing::info!("Confirm Exit (y?)");
                // read key
                let input = console::Term::stdout().read_key().unwrap();
                if input == console::Key::Char('y') {
                    tracing::info!("Exiting");
                    std::process::exit(0);
                }
                tracing::info!("Continuing");
            }
        }
    });
        
        warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, 3031)).await;
}