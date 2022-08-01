use hyper::server::conn::Http;
use tokio::net::TcpListener;
use std::fs;
use std::sync::{Arc};
use std::sync::atomic::AtomicUsize;

use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Result, StatusCode};

#[cfg(not(target_os = "wasi"))]
async fn get_tcplistener() -> TcpListener {
    println!("Listening on: 127.0.0.1:4000");
    TcpListener::bind("127.0.0.1:4000").await.unwrap()
}

#[cfg(target_os = "wasi")]
async fn get_tcplistener() -> TcpListener {
    use std::os::wasi::io::FromRawFd;
    let stdlistener = unsafe { std::net::TcpListener::from_raw_fd(4) };
    stdlistener.set_nonblocking(true).unwrap();
    TcpListener::from_std(stdlistener).unwrap()
}

#[derive(Debug)]
struct State {
    visitors: AtomicUsize,
    likes: AtomicUsize,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let listener = get_tcplistener().await;
    let state = Arc::new(State{visitors: AtomicUsize::new(0), likes: AtomicUsize::new(0)});

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(move |req| {
                    router(req, state.clone())
                }))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}


async fn router(req: Request<Body>, state: Arc<State>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/like") => {
            state.likes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            Ok(inc_like())
        }

        (&Method::GET, "/") | (&Method::GET, "/index.html") => {
            state.visitors.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            return_index(state.clone()).await
        }
        (&Method::GET, anything) => {
            state.visitors.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
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

/// count likes
fn inc_like() -> Response<Body> {
    Response::builder()
        .status(301)
        .header("Location", "/")
        .body("Back home!".into()).unwrap_or(not_found())
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
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

async fn return_index(state: Arc<State>) -> Result<Response<Body>> {
    if let Ok(contents) = fs::read_to_string("public/index.html") {
        let contents = contents.replace("{{visitors}}", format!("{:?}", state.visitors).as_str());
        let contents = contents.replace("{{likes}}", format!("{:?}", state.likes).as_str());
        let body = contents.into();
        return Ok(Response::new(body));
    }
    Ok(not_found())
}
