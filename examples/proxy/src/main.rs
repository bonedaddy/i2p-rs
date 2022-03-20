use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use clap::{ArgMatches, App, Arg, SubCommand};
use anyhow::{anyhow, Result};
use i2p::net::{I2pListener, I2pStream, I2pAddr};
use std::io::{Write, Read};
use log::*;
use std::{thread, time};
use std::str::from_utf8;
#[tokio::main]
async fn main() -> Result<()> {
	env_logger::init();

	let matches = App::new("proxy")
	.subcommands(vec![
		SubCommand::with_name("forwarder")
		.arg(Arg::with_name("ip").long("ip").takes_value(true).required(true))
		.arg(Arg::with_name("destination").long("destination").takes_value(true).required(true)),
		SubCommand::with_name("server")
	]).get_matches();
	process_matches(&matches).await?;
	Ok(())
}

async fn process_matches(matches: &ArgMatches<'_>) -> Result<()> {
	match matches.subcommand() {
		("forwarder", Some(forwarder)) => {
			let ip = forwarder.value_of("ip").unwrap();
			let destination = forwarder.value_of("destination").unwrap();
			let mut stream = I2pStream::connect(destination).unwrap();
			let listener = match TcpListener::bind(ip.to_string()).await {
                Ok(listener) => listener,
                Err(err) => return Err(anyhow!("failed to create listener {:#?}", err)),
            };
            loop {
                // Asynchronously wait for an inbound socket.
                let (mut socket, _) = listener.accept().await?;
        
                // And this is where much of the magic of this server happens. We
                // crucially want all clients to make progress concurrently, rather than
                // blocking one on completion of another. To achieve this we use the
                // `tokio::spawn` function to execute the work in the background.
                //
                // Essentially here we're executing a new task to run concurrently,
                // which will allow all of our clients to be processed concurrently.
        
                    let mut buf = vec![0; 1024];
                    // In a loop, read data from the socket and write the data back.
                    loop {
                        let n = socket
                            .read(&mut buf)
                            .await
                            .expect("failed to read data from socket");
        
                        if n == 0 {
                            return Ok(());
                        }
						stream.write(&buf[0..n]);
                        println!("proxy received data");
                    }
            }
		}
		("server", Some(_)) => {
			let server = I2pListener::bind().unwrap();
			let our_dest = server.local_addr().unwrap();
			let base64_addr = format!("{}", our_dest.dest());
			let our_dest = I2pAddr::from_b64(&base64_addr).unwrap();
			println!("our address {}", our_dest);
			for stream in server.incoming() {
				match stream {
					Ok(mut stream) => {
						thread::spawn(move || {
							let mut buffer = [0; 100];
							loop {
								let n = match stream.read(&mut buffer) {
									Ok(n) => n,
									Err(err) => {
										error!("failed to read from stream {:#?}", err);
										continue;
									}
								};
								let data = match from_utf8(&buffer[0..n]) {
									Ok(data) => data,
									Err(err) => {
										error!("failed to convert buffer to utf8 {:#?}", err);
										continue;
									}
								};
								info!("< {:?}", data);
								println!("< {:?}", data);
								match stream.write("pong".as_bytes()) {
									Ok(_) => (),
									Err(err) => error!("failed to write response {:#?}", err),
								};
							}
						});
					}
					Err(e) => error!("Error on incoming connection: {:?}", e),
				}
			}
			Ok(())
		}
		_ => return Err(anyhow!("invalid subcommand")),
	}
}
