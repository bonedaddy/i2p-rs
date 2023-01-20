use crossbeam::sync::WaitGroup;
use env_logger;
use i2p;

use crossbeam_channel::select;
use i2p::net::{I2pListener, I2pStream, BASE64_I2P};
use i2p::sam_options::{
	I2CPClientOptions, I2CPOptions, I2CPRouterOptions, I2CPTunnelInboundOptions,
	I2CPTunnelOutboundOptions, SAMOptions, SignatureType, I2CPRouterCryptoOptions, LeaseSetType, LeaseSetAuthType, LeaseSetClientEncryption, LeaseSetClientPSK, LeaseSetPrivKey, LeaseSetSecret, LeaseSetPrivateKey,
};
use log::*;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::str::from_utf8;
use std::{thread, time};

use i2p::sam::{SamConnection, SessionStyle, DEFAULT_API};

// Run with RUST_LOG=debug to see the action
#[tokio::main]
async fn main() {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	let (pubkey, seckey) = {
		let mut sam_conn = SamConnection::connect(DEFAULT_API).unwrap();
		sam_conn
			.generate_destination(SignatureType::RedDsaSha512Ed25519)
			.unwrap()
	};
	let (pubkey2, seckey2) = {
		let mut sam_conn = SamConnection::connect(DEFAULT_API).unwrap();
		sam_conn
			.generate_destination(SignatureType::RedDsaSha512Ed25519)
			.unwrap()
	};
	let decoded = BASE64_I2P.decode(seckey2.as_bytes()).unwrap();
	let pk = BASE64_I2P.encode(&decoded[0..32]);
	println!("pk {}", pk);
	info!("New public key: {}", pubkey);
	info!("New secret key: {}", seckey);
	let mut watcher = i2p::session_watcher::SamSessionWatcher::new(
		DEFAULT_API,
		&seckey,
		SessionStyle::Stream,
		SAMOptions {
			from_port: None,
			to_port: None,
			signature_type: SignatureType::RedDsaSha512Ed25519,
			i2cp_options: Some(I2CPOptions { 
				router_options: Some(I2CPRouterOptions {
					lease_set_type: Some(LeaseSetType(5)),
					lease_set_auth_type: Some(LeaseSetAuthType::PSKPerClient),
					lease_set_secret: Some(LeaseSetSecret(BASE64_I2P.encode("foobarbaz".as_bytes()))),
					lease_set_priv_key: Some(LeaseSetPrivKey(pk)),
					..Default::default()
				}), 
				client_options: Some(I2CPClientOptions {
					lease_set_auth_type: Some(LeaseSetAuthType::PSKPerClient),
					lease_set_secret: Some(LeaseSetSecret(BASE64_I2P.encode("foobarbaz".as_bytes()))),
					lease_set_private_key: Some(LeaseSetPrivateKey("ECIES_X25519:OFgx84wKG6YUkXwk9oDbVciOVOrGCU98o6YNzi3ibGw=".to_string())),
					leaset_set_client_encryption: Some(vec![
						LeaseSetClientEncryption::PSK(LeaseSetClientPSK {
							nickname: BASE64_I2P.encode("foobarbaz".as_bytes()),
							nnn: 0,
							psk: i2p::utils::rand_string(32),
						})
					]),
					..Default::default()
				})
			})
		},
	)
	.unwrap();

	loop {
		match watcher.accept() {
			Ok((conn, addr)) => {
				info!("receiving incoming connection {}", addr);
				let _ = conn.shutdown(Shutdown::Both).unwrap();
			}
			Err(err) => {
				error!("failed to accept connection {:#?}", err);
				return;
			}
		}
	}
}
