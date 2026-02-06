use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let address = std::env::var("PORT")
        .ok()
        .map(|port| format!("127.0.0.1:{port}"))
        .unwrap_or_else(|| String::from("127.0.0.1:8080"));

    let listener = TcpListener::bind(&address)?;
    println!("web_std_http listening on http://{address}");

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0_u8; 1024];
        let _ = stream.read(&mut buffer)?;

        let body = "ok";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );

        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}
