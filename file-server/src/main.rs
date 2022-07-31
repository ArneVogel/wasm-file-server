#![deny(warnings)]

use hyper::server::conn::Http;
use tokio::net::TcpListener;
use std::fs;

use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Result, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";


#[cfg(not(target_os = "wasi"))]
async fn get_tcplistener() -> TcpListener {
    println!("Listening on: 127.0.0.1:4000");
    TcpListener::bind("127.0.0.1:4000").await.unwrap()
}

#[cfg(target_os = "wasi")]
async fn get_tcplistener() -> TcpListener {
    use std::os::wasi::io::FromRawFd;
    let stdlistener = unsafe { std::net::TcpListener::from_raw_fd(3) };
    stdlistener.set_nonblocking(true).unwrap();
    TcpListener::from_std(stdlistener).unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let listener = get_tcplistener().await;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(response_examples))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, anything) => {
            let mut s = String::from(anything);
            match anything.starts_with("/") {
                true => s.insert_str(0, "public"),
                _ => return Ok(not_found()),
            }
            simple_file_send(s.as_str()).await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    if let Ok(contents) = fs::read_to_string(filename) {
        let body = contents.into();
        return Ok(Response::new(body));
    }
    println!("could not open \"{}\": {:?}", filename, fs::read_to_string(filename).err());
    Ok(not_found())
}
