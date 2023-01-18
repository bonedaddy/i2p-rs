use rand::{distributions::Alphanumeric, Rng};
use std::str::FromStr;

use super::sam::Session;

use crate::{
	parsers::{sam_session_status, sam_stream_status},
	sam::SessionStyle,
	sam_options::SAMOptions,
	I2PError, SamConnection,
};
use anyhow::Result;

pub struct SessionManager {
	/// the primary session which is created
	pub primary_session: Session,
	/// maps session_key -> in_use
	pub subsessions: dashmap::DashMap<String, SubSession>,
}

pub struct SubSession {
	pub nickname: String,
	pub listen_port: u16,
}

impl SessionManager {
	pub fn new(session: Session) -> SessionManager {
		SessionManager {
			primary_session: session,
			subsessions: dashmap::DashMap::new(),
		}
	}
	pub fn add_subsession(
		&mut self,
		session_key: &str,
		listen_port: &str,
		style: SessionStyle,
		options: SAMOptions,
	) -> Result<()> {
		let nickname = self.rand_session_id();
		let add_session_msg = format!(
			// values for SIGNATURE_TYPE and leaseSetEncType taken from
			// https://github.com/eyedeekay/goSam/blob/62cade9ebc26e48ff32a517ef94212fc90aa92cd/client.go#L169
			// https://github.com/eyedeekay/goSam/blob/62cade9ebc26e48ff32a517ef94212fc90aa92cd/client.go#L166
			"SESSION ADD STYLE={style} ID={nickname} LISTEN_PORT={listen_port} {options}\n",
			style = style.string(),
			nickname = nickname,
			listen_port = listen_port,
			options = options.options(),
		);
		self.primary_session
			.sam
			.send(add_session_msg, sam_session_status)?;
		let _ = self.subsessions.insert(
			session_key.to_string(),
			SubSession {
				nickname,
				listen_port: u16::from_str(listen_port).unwrap(),
			},
		);
		Ok(())
	}
	pub fn accept(&self, session_key: &str) -> Result<Session> {
		let mut sam_conn = SamConnection::connect(self.primary_session.sam_api()?).unwrap();
		let subsession_info = match self.subsessions.get(&session_key.to_string()) {
			Some(sess_info) => sess_info,
			None => return Err(I2PError::Io("invalid_session_key".to_string()).into()),
		};
		let accept_stream_msg = format!(
			"STREAM ACCEPT ID={nickname} SILENT=false\n",
			nickname = subsession_info.nickname,
		);
		sam_conn.send(accept_stream_msg, sam_stream_status)?;
		let local_dest = sam_conn.naming_lookup("ME")?;
		Ok(Session {
			sam: sam_conn,
			local_dest,
			nickname: subsession_info.nickname.clone(),
		})
	}
	fn rand_session_id(&self) -> String {
		let suffix: String = rand::thread_rng()
			.sample_iter(&Alphanumeric)
			.take(16)
			.collect();
		format!("sessid-{suffix}")
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::sam::DEFAULT_API;

	#[test]
	fn test_session_manager() {
		let sam_sess = Session::create(
			DEFAULT_API,
			"TRANSIENT",
			"mainsess",
			SessionStyle::Primary,
			SAMOptions::default(),
		)
		.unwrap();
		let mut sess_man = SessionManager::new(sam_sess);
		sess_man
			.add_subsession(
				"test_session",
				"8696",
				SessionStyle::Stream,
				Default::default(),
			)
			.unwrap();
		println!("session added");
	}
}
