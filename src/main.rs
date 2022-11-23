use std::net::SocketAddr;

use hyper::http::HeaderValue;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, HeaderMap, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            // utf8mb4
            let msg = "👋 Try POSTing data to /classify such as: `curl http://localhost:3000/classify -X POST --data-binary '@grace_hopper.jpg'`".to_string();
            *response.body_mut() = Body::from(msg);

            // Return Content-Type: 'text/plain; charset=utf-8' to ensure emoji are displayed correctly
            // https://docs.rs/hyper/latest/hyper/struct.Response.html#method.headers_mut
            let mut headers = HeaderMap::new();
            headers.insert(
                hyper::header::CONTENT_TYPE,
                HeaderValue::from_str("text/plain; charset=utf-8").unwrap(),
            );

            *response.headers_mut() = headers;
        }

        (&Method::POST, "/classify") => {
            let model_data: &[u8] =
                include_bytes!("models/mobilenet_v1_1.0_224/mobilenet_v1_1.0_224_quant.tflite");
            let labels =
                include_str!("models/mobilenet_v1_1.0_224/labels_mobilenet_quant_v1_224.txt");

            // buffering the request body...
            let buf = hyper::body::to_bytes(req.into_body()).await?;
            let flat_img = wasmedge_tensorflow_interface::load_jpg_image_to_rgb8(&buf, 224, 224);

            let mut session = wasmedge_tensorflow_interface::Session::new(
                &model_data,
                wasmedge_tensorflow_interface::ModelType::TensorFlowLite,
            );
            session
                .add_input("input", &flat_img, &[1, 224, 224, 3])
                .run();
            let res_vec: Vec<u8> = session.get_output("MobilenetV1/Predictions/Reshape_1");

            let mut i = 0;
            let mut max_index: i32 = -1;
            let mut max_value: u8 = 0;
            while i < res_vec.len() {
                let cur = res_vec[i];
                if cur > max_value {
                    max_value = cur;
                    max_index = i as i32;
                }
                i += 1;
            }

            let mut label_lines = labels.lines();
            for _i in 0..max_index {
                label_lines.next();
            }
            let class_name = label_lines.next().unwrap().to_string();

            *response.body_mut() = Body::from(format!(
                "{} is detected with {}/255 confidence",
                class_name, max_value
            ))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    // return the response
    Ok(response)
}

static DEFAULT_PORT: &str = "8888";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // get PORT env var
    let port = std::env::var("PORT")
        .unwrap_or(DEFAULT_PORT.to_owned())
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(router))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
