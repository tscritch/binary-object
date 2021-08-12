use hyper::{Body, Method, StatusCode, Request, Response};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use serde_json::json;

use super::movie::{Movie};

static INDEX: &str = "tester.html";
static NOTFOUND: &[u8] = b"Not Found";

// static MOVIES: [&'static Movie; 999] = Movies::create_random();

pub async fn service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
    (&Method::GET, "/bincode_request") => bincode_request(req).await,
    (&Method::GET, "/json_request") => json_request(req).await,
    (&Method::GET, "/bincode_movies") => bincode_movies(req).await,
    (&Method::GET, "/json_movies") => json_movies(req).await,

    // Return the 404 Not Found for other routes.
    _ => {
        let mut not_found = Response::default();
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    }
  }
}

async fn bincode_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let movie = Movie {
    title: String::from("Toy Story"),
    year: 1996,
    description: String::from("a story about toys"),
    poster: String::from("https://google.com")
  };

  let encoded: Vec<u8> = bincode::serialize(&movie).unwrap();

  Ok(Response::new(encoded.into()))
}

async fn json_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let movie = Movie {
    title: String::from("Toy Story"),
    year: 1996,
    description: String::from("a story about toys"),
    poster: String::from("https://google.com")
  };

  let encoded = json!(movie).to_string();

  Ok(Response::new(encoded.into()))
}

async fn bincode_movies(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let movies = Movie::create_random();

  let encoded: Vec<u8> = bincode::serialize(&movies).unwrap();

  Ok(Response::new(encoded.into()))
}

async fn json_movies(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let movies = Movie::create_random();

  let encoded = json!(movies).to_string();

  Ok(Response::new(encoded.into()))
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>, hyper::Error> {
  // Serve a file by asynchronously reading it by chunks using tokio-util crate.

  if let Ok(file) = File::open(filename).await {
      let stream = FramedRead::new(file, BytesCodec::new());
      let body = Body::wrap_stream(stream);
      return Ok(Response::new(body));
  }

  Ok(Response::builder()
  .status(StatusCode::NOT_FOUND)
  .body(NOTFOUND.into())
  .unwrap())
}