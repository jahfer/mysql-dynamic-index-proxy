use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
//use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
	.nth(1)
	.unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let mut initial_db = HashMap::new();
    initial_db.insert("foo".to_string(), "bar".to_string());

    loop {
	match listener.accept().await {
	    Ok((socket, _)) => {
		tokio::spawn(async move {
		    let mut lines = Framed::new(socket, LinesCodec::new());

		    while let Some(result) = lines.next().await {
			match result {
			    Ok(line) => {
				let response = handle_request(&line);
				if let Err(e) = lines.send(response).await {
				    println!("error on sending response; error = {:?}", e);
				}
			    }
			    Err(e) => {
				println!("error on decoding from socket; error = {:?}", e);
			    }
			}
		    }
		});
	    }
	    Err(e) => {
		println!("error accepting socket; error = {:?}", e);
	    }
	}
    }
}

fn handle_request(line: &str) -> String {
    format!("Received line: {:?}", line).to_string()
}
