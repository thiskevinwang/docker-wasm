use bytecodec::DecodeExt;
use httpcodec::{
    HeaderField, HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode,
};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

// import ./infer.rs
// mod infer;
// use infer::infer;

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    println!(
        "[{}], {}",
        req.method().as_str(),
        req.request_target().as_str()
    );
    let res: Response<String> = match req.method().as_str() {
        "GET" => Response::new(
            HttpVersion::V1_0,
            StatusCode::new(200)?,
            ReasonPhrase::new("OK")?,
            format!("Hello, world! Try POST /"),
        ),
        "POST" => {
            let body = req.body().as_str();
            let res_body = body;
            // let res_body = infer(&body);

            let mut res = Response::new(
                HttpVersion::V1_0,
                StatusCode::new(200)?,
                ReasonPhrase::new("OK")?,
                // format!("echo: {}", res_body),
                res_body.to_owned(),
            );

            res.header_mut()
                .add_field(HeaderField::new("Content-Type", "image/png")?);

            res
        }
        // this match case is just for the compiler
        _ => Response::new(
            HttpVersion::V1_0,
            StatusCode::new(500)?,
            ReasonPhrase::new("Internal server error")?,
            format!("Something went wrong"),
        ),
    };

    Ok(res)
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 100_000];
    let mut data = Vec::new();
    // let mut vec = Vec::new();

    println!("READING STREAM");

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 100_000 {
            break;
        }
    }
    // loop {
    //     let n = stream.read_to_end(&mut vec)?;
    //     data.extend_from_slice(&buff[0..n]);
    //     if n < 500_000 {
    //         break;
    //     }
    // }
    println!("DONE READING STREAM");

    // println!("data: {}", data.len());

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::BytesDecoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => {
            println!("req: {:?}", req);
            handle_http(req)
        }
        Err(e) => {
            println!("err: {:?}", e);
            Err(e)
        }
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("8888".to_string());
    println!("Listening at http://localhost:{}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
