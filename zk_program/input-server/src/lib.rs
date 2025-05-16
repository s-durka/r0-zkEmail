use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct ServerArg {
    pub bytes: Vec<u8>,
    pub url: String,
}

fn prefix_bytes_with_len(bytes: Vec<u8>) -> Vec<u8> {
    let data_len = bytes.len();
    let mut buffer = Vec::with_capacity(4 + data_len);
    let mut cursor = Cursor::new(&mut buffer);
    cursor.write_u32::<LittleEndian>(data_len as u32).unwrap();

    buffer.extend_from_slice(bytes.as_slice());
    buffer
}

pub fn run_server(args: Vec<ServerArg>) {
    println!("Server started on http://127.0.0.1:8000");
    rouille::start_server("127.0.0.1:8000", move |request| {
        for arg in &args {
            if request.url() == arg.url {
                println!("Received request for {}. Responding with length-prefixed bytes.", arg.url);
                let response = prefix_bytes_with_len(arg.bytes.clone());
                return rouille::Response::from_data("application/octet-stream", response)
                    .with_unique_header("Connection", "close");
            }
        }
        println!("Received request for {}. Responding with 404.", request.url());
        rouille::Response::empty_404()
    });
}