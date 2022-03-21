use std::io::prelude::*;

use std::clone::Clone;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};

use log::debug;
use nom::IResult;
use rand::distributions::Alphanumeric;
use rand::{self, Rng};


use crate::error::{Error, ErrorKind};
use crate::net::{I2pAddr, I2pSocketAddr};
use crate::parsers::{sam_hello, sam_naming_reply, sam_session_status, sam_stream_status, sam_dest_reply};
use crate::sam_options::SAMOptions;

pub static DEFAULT_API: &'static str = "127.0.0.1:7656";

static SAM_MIN: &'static str = "3.1";
static SAM_MAX: &'static str = "3.2";

pub enum SignatureType {
	DsaSha1,
	EcdsaSha256P256,
	EcdsaSha384P384,
	EcdsaSha512P21,
	EdDsaSha512Ed25519,
}

impl SignatureType {
	fn string(&self) -> &str {
		match self {
			Self::DsaSha1 => "DSA_SHA1",
			Self::EcdsaSha256P256 => "ECDSA_SHA256_P256",
			Self::EcdsaSha384P384 => "ECDSA_SHA384_P384",
			Self::EcdsaSha512P21 => "ECDSA_SHA512_P521",
			Self::EdDsaSha512Ed25519 => "EdDSA_SHA512_Ed25519",
		}
	}
}

impl Default for SignatureType {
	fn default() -> Self { Self::EdDsaSha512Ed25519 }
}

pub enum SessionStyle {
	Datagram,
	Raw,
	Stream,
}

pub struct SamConnection {
	conn: TcpStream,
}

pub struct Session {
	sam: SamConnection,
	local_dest: String,
	nickname: String,
}

pub struct StreamConnect {
	sam: SamConnection,
	session: Session,
	peer_dest: String,
	peer_port: u16,
	local_port: u16,
}

impl SessionStyle {
	fn string(&self) -> &str {
		match *self {
			SessionStyle::Datagram => "DATAGRAM",
			SessionStyle::Raw => "RAW",
			SessionStyle::Stream => "STREAM",
		}
	}
}

fn verify_response<'a>(vec: &'a [(&str, &str)]) -> Result<HashMap<&'a str, &'a str>, Error> {
	let new_vec = vec.clone();
	let map: HashMap<&str, &str> = new_vec.iter().map(|&(k, v)| (k, v)).collect();
	let res = map.get("RESULT").unwrap_or(&"OK").clone();
	let msg = map.get("MESSAGE").unwrap_or(&"").clone();
	match res {
		"OK" => Ok(map),
		"CANT_REACH_PEER" => Err(ErrorKind::SAMCantReachPeer(msg.to_string()).into()),
		"KEY_NOT_FOUND" => Err(ErrorKind::SAMKeyNotFound(msg.to_string()).into()),
		"PEER_NOT_FOUND" => Err(ErrorKind::SAMPeerNotFound(msg.to_string()).into()),
		"DUPLICATED_DEST" => Err(ErrorKind::SAMDuplicatedDest(msg.to_string()).into()),
		"INVALID_KEY" => Err(ErrorKind::SAMInvalidKey(msg.to_string()).into()),
		"INVALID_ID" => Err(ErrorKind::SAMInvalidId(msg.to_string()).into()),
		"TIMEOUT" => Err(ErrorKind::SAMTimeout(msg.to_string()).into()),
		"I2P_ERROR" => Err(ErrorKind::SAMI2PError(msg.to_string()).into()),
		_ => Err(ErrorKind::SAMInvalidMessage(msg.to_string()).into()),
	}
}

impl SamConnection {
	fn send<F>(&mut self, msg: String, reply_parser: F) -> Result<HashMap<String, String>, Error>
	where
		F: Fn(&str) -> IResult<&str, Vec<(&str, &str)>>,
	{
		debug!("-> {}", &msg);
		self.conn.write_all(&msg.into_bytes())?;

		let mut reader = BufReader::new(&self.conn);
		let mut buffer = String::new();
		reader.read_line(&mut buffer)?;
		debug!("<- {}", &buffer);

		let vec_opts = reply_parser(&buffer)?.1;
		verify_response(&vec_opts).map(|m| {
			m.iter()
				.map(|(k, v)| (k.to_string(), v.to_string()))
				.collect()
		})
	}

	fn handshake(&mut self) -> Result<HashMap<String, String>, Error> {
		let hello_msg = format!(
			"HELLO VERSION MIN={min} MAX={max} \n",
			min = SAM_MIN,
			max = SAM_MAX
		);
		self.send(hello_msg, sam_hello)
	}

	pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<SamConnection, Error> {
		let tcp_stream = TcpStream::connect(addr)?;

		let mut socket = SamConnection { conn: tcp_stream };
		socket.handshake()?;

		Ok(socket)
	}

	// TODO: Implement a lookup table
	pub fn naming_lookup(&mut self, name: &str) -> Result<String, Error> {
		let naming_lookup_msg = format!("NAMING LOOKUP NAME={name} \n", name = name);
		let ret = self.send(naming_lookup_msg, sam_naming_reply)?;
		Ok(ret["VALUE"].clone())
	}

	pub fn generate_destination(
		&mut self,
		signature_type: SignatureType,
	) -> Result<(String, String), Error> {
		let dest_gen_msg = format!(
			"DEST GENERATE SIGNATURE_TYPE={signature_type} \n",
			signature_type = signature_type.string(),
		);
		let ret = self.send(dest_gen_msg, sam_dest_reply)?;
		Ok((ret["PUB"].clone(), ret["PRIV"].clone()))
	}

	pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), Error> {
		self.conn.set_nonblocking(nonblocking).map_err(|e| e.into())
	}

	pub fn duplicate(&self) -> Result<SamConnection, Error> {
		self.conn.try_clone().map(|s| SamConnection { conn: s }).map_err(|e| e.into())
	}
}

impl Session {
	/// Create a new session using all provided parameters
	pub fn create<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		nickname: &str,
		style: SessionStyle,
		signature_type: SignatureType,
		options: SAMOptions,
	) -> Result<Session, Error> {
		let mut sam = SamConnection::connect(sam_addr)?;
		let create_session_msg = format!(
			// values for SIGNATURE_TYPE and leaseSetEncType taken from
			// https://github.com/eyedeekay/goSam/blob/62cade9ebc26e48ff32a517ef94212fc90aa92cd/client.go#L169
			// https://github.com/eyedeekay/goSam/blob/62cade9ebc26e48ff32a517ef94212fc90aa92cd/client.go#L166
			"SESSION CREATE STYLE={style} ID={nickname} DESTINATION={destination} SIGNATURE_TYPE={signature_type} {options}\n",
			style = style.string(),
			nickname = nickname,
			destination = destination,
			signature_type = signature_type.string(),
			options = options.string()
		);

		sam.send(create_session_msg, sam_session_status)?;

		let local_dest = sam.naming_lookup("ME")?;

		Ok(Session {
			sam: sam,
			local_dest: local_dest,
			nickname: nickname.to_string(),
		})
	}

	/// Create a new session identified by the provided destination. Auto-generates
	/// a nickname uniquely associated with the new session.
	pub fn from_destination<A: ToSocketAddrs>(sam_addr: A, destination: &str) -> Result<Session, Error> {
		Self::create(sam_addr, destination, &nickname(), SessionStyle::Stream, SignatureType::EdDsaSha512Ed25519, SAMOptions::default())
	}

	/// Convenience constructor to create a new transient session with an
	/// auto-generated nickname.
	pub fn transient<A: ToSocketAddrs>(sam_addr: A) -> Result<Session, Error> {
		Self::create(sam_addr, "TRANSIENT", &nickname(), SessionStyle::Stream, SignatureType::EdDsaSha512Ed25519, SAMOptions::default())
	}
	pub fn persistent<A: ToSocketAddrs>(sam_addr: A, destination: &str) -> Result<Session, Error> {
		Self::create(sam_addr, destination, &nickname(), SessionStyle::Stream, SignatureType::EdDsaSha512Ed25519, SAMOptions::default())
	}
	pub fn sam_api(&self) -> Result<SocketAddr, Error> {
		self.sam.conn.peer_addr().map_err(|e| e.into())
	}

	pub fn naming_lookup(&mut self, name: &str) -> Result<String, Error> {
		self.sam.naming_lookup(name)
	}

	pub fn duplicate(&self) -> Result<Session, Error> {
		self.sam.duplicate().map(|s| Session {
			sam: s,
			local_dest: self.local_dest.clone(),
			nickname: self.nickname.clone(),
		}).map_err(|e| e.into())
	}
}

impl StreamConnect {

	/// Create a new SAM client connection to the provided destination and port.
	/// Also creates a new transient session to support the connection.
	pub fn new<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
		port: u16,
	) -> Result<StreamConnect, Error> {
		let session = Session::transient(sam_addr)?;
		Self::with_session(&session, destination, port)
	}

	/// Create a new SAM client connection to the provided destination and port
	/// using the provided session.
	pub fn with_session(session: &Session, dest: &str, port: u16) -> Result<StreamConnect, Error> {
		let mut sam = SamConnection::connect(session.sam_api()?).unwrap();
		let dest = sam.naming_lookup(dest)?;

		let mut stream_msg = format!(
			"STREAM CONNECT ID={nickname} DESTINATION={destination} SILENT=false\n",
			nickname = session.nickname,
			destination = dest,
		);
		if port > 0 {
			stream_msg.push_str(&format!(" TO_PORT={port}\n", port = port));
		} else {
			stream_msg.push_str("\n");
		}

		sam.send(stream_msg, sam_stream_status)?;

		Ok(StreamConnect {
			sam: sam,
			session: session.duplicate()?,
			peer_dest: dest,
			peer_port: port,
			local_port: 0,
		})
	}

	pub fn peer_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.peer_dest.clone(), self.peer_port))
	}

	pub fn local_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.session.local_dest.clone(), self.local_port))
	}

	pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), Error> {
		self.sam.set_nonblocking(nonblocking)
	}

	pub fn shutdown(&self, how: Shutdown) -> Result<(), Error> {
		self.sam.conn.shutdown(how).map_err(|e| e.into())
	}

	pub fn duplicate(&self) -> Result<StreamConnect, Error> {
		Ok(StreamConnect {
			sam: self.sam.duplicate()?,
			session: self.session.duplicate()?,
			peer_dest: self.peer_dest.clone(),
			peer_port: self.peer_port,
			local_port: self.local_port,
		})
	}
	pub fn to_tokio_stream(&self) -> Result<tokio::net::TcpStream, Error> {
		Ok(tokio::net::TcpStream::from_std(self.sam.duplicate()?.conn)?)
	}
}

impl Read for StreamConnect {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.sam.conn.read(buf)
	}
}

impl Write for StreamConnect {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.sam.conn.write(buf)
	}
	fn flush(&mut self) -> io::Result<()> {
		self.sam.conn.flush()
	}
}

pub struct StreamForward {
	session: Session,
}

impl StreamForward {
	pub fn new_transient<A: ToSocketAddrs>(
		sam_addr: A,
	) -> Result<StreamForward, Error> {
		Ok(StreamForward {session: Session::transient(sam_addr)?})
	}
	pub fn new_persistent<A: ToSocketAddrs>(
		sam_addr: A,
		destination: &str,
	) -> Result<StreamForward, Error> {
		Ok(StreamForward {session: Session::persistent(sam_addr, destination)?})
	}

	/// Create a new SAM client connection to the provided destination and port
	/// using the provided session.
	pub fn with_session(session: &Session) -> Result<StreamForward, Error> {
		Ok(StreamForward {session: session.duplicate()?})
	}

	pub fn accept(&self) -> Result<(StreamConnect, I2pSocketAddr), Error> {
		let mut sam_conn = SamConnection::connect(self.session.sam_api()?).unwrap();

		let accept_stream_msg = format!(
			"STREAM ACCEPT ID={nickname} SILENT=false\n",
			nickname = self.session.nickname,
		);
		sam_conn.send(accept_stream_msg, sam_stream_status)?;

		let mut stream = StreamConnect {
			sam: sam_conn,
			session: self.session.duplicate()?,
			peer_dest: "".to_string(),
			// port only provided with SAM v3.2+ (not on i2pd)
			peer_port: 0,
			local_port: 0,
		};

		// TODO use a parser combinator, perhaps move down to sam.rs
		let destination: String = {
			let mut buf_read = io::BufReader::new(stream.duplicate()?);
			let mut dest_line = String::new();
			buf_read.read_line(&mut dest_line)?;
			dest_line.split(" ").next().unwrap_or("").trim().to_string()
		};
		if destination.is_empty() {
			return Err(ErrorKind::SAMKeyNotFound("No b64 destination in accept".to_string()).into());
		}

		let addr = I2pSocketAddr::new(I2pAddr::from_b64(&destination)?, 0);
		stream.peer_dest = destination;
	
		Ok((stream, addr))
	}

	pub fn local_addr(&self) -> Result<(String, u16), Error> {
		Ok((self.session.local_dest.clone(), 0))
	}

	pub fn duplicate(&self) -> Result<StreamForward, Error> {
		Ok(StreamForward {session: self.session.duplicate()?})
	}
}

fn nickname() -> String {
	let suffix: String = rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(8)
		.collect();
	format!("i2prs-{}", suffix)
}
