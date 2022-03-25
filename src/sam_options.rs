//! objects used for configuration SAM sessions

//! options used when interacting with the SAM bridge
pub struct SAMOptions {
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub i2cp: Option<I2CP>
}

impl SAMOptions {
    pub fn string(&self) -> String { 
        let mut options = String::default();
        if let Some(from_port) = self.from_port {
            options.push_str(&format!("FROM_PORT={} ", from_port));
        }
        if let Some(to_port) = self.to_port {
            options.push_str(&format!("TO_PORT={} ", to_port));
        }
        if let Some(i2cp_options) = &self.i2cp {
            options.push_str(&format!("{}", i2cp_options.string()));
        }
        options
    }
}

/// I2CP options taken from https://geti2p.net/en/docs/protocol/i2cp
pub struct I2CP {
    pub router_options: Option<I2CPRouterOptions>,
}



impl I2CP {
    pub fn string(&self) -> String {
        let mut options = String::default();
        options
    }
}



pub struct I2CPRouterOptions {
    /// The timeout (ms) for all sent messages. Unused. See the protocol specification for per-message settings.
    pub client_message_timeout: Option<u32>,
    /// Should generally be set to true for clients and false for servers
    pub dont_publish_lease_set: Option<bool>,
    /// If true, the router just sends the MessagePayload instead of sending a MessageStatus and awaiting a ReceiveMessageBegin.
    pub fast_receive: Option<bool>,
    /// The type of authentication for encrypted LS2. 0 for no per-client authentication (the default); 1 for DH per-client authentication; 2 for PSK per-client authentication. See proposal 123. 
    pub lease_set_auth_type: Option<LeaseSetAuthType>,
    /// 	The encryption type to be used, as of 0.9.38. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. As of 0.9.39, may be comma-separated values for multiple types. See PublicKey in common strutures spec for values. See proposals 123, 144, and 145. 
    pub lease_set_enc_type: Option<LeaseSetEncType>,
    /// The expiration of the offline signature, 4 bytes, seconds since the epoch. See proposal 123. 
    pub lease_set_offline_expiration: Option<LeaseSetOfflineExpiration>,
    /// The base 64 of the offline signature. See proposal 123. 
    pub lease_set_offline_signature: Option<LeaseSetOfflineSignature>,
    /// A base 64 X25519 private key for the router to use to decrypt the encrypted LS2 locally, only if per-client authentication is enabled. Optionally preceded by the key type and ':'. Only "ECIES_X25519:" is supported, which is the default. See proposal 123. Do not confuse with i2cp.leaseSetPrivateKey which is for the leaseset encryption keys. 
    pub lease_set_priv_key: Option<LeaseSetPrivKey>,
    /// 	Base 64 encoded UTF-8 secret used to blind the leaseset address. See proposal 123. 
    pub lease_set_secret: Option<LeaseSetSecret>,
    ///  The base 64 of the transient private key, prefixed by an optional sig type number or name, default DSA_SHA1. See proposal 123. 
    pub lease_set_transient_public_key: Option<LeaseSetTransientPublicKey>,
    /// The type of leaseset to be sent in the CreateLeaseSet2 Message. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. See proposal 123. 
    pub lease_set_type: Option<LeaseSetType>,
    /// Guaranteed is disabled; None implemented in 0.8.1; the streaming lib default is None as of 0.8.1, the client side default is None as of 0.9.4
    pub message_reliability: Option<MessageReliability>,
    pub username: Option<String>,
    pub password: Option<String>,
    /// inbound tunnel optoins
    pub inbound: Option<I2CPTunnelInboundOptions>,
    pub output: Option<I2CPTunnelOutboundOptions>,

}

pub struct I2CPRouterCryptoOptions {
    /// Minimum number of ElGamal/AES Session Tags before we send more. Recommended: approximately tagsToSend * 2/3
    pub low_tag_threshold: Option<u8>,
    /// Inbound tag window for ECIES-X25519-AEAD-Ratchet. Local inbound tagset size. See proposal 144. 
    pub ratchet_inbound_tags: Option<u64>,
    /// Outbound tag window for ECIES-X25519-AEAD-Ratchet. Advisory to send to the far-end in the options block. See proposal 144. 
    pub ratchet_outbound_tags: Option<u64>,
    /// Number of ElGamal/AES Session Tags to send at a time. For clients with relatively low bandwidth per-client-pair (IRC, some UDP apps), this may be set lower.
    pub tags_to_send: Option<u8>,
}


#[derive(Default)]
pub struct I2CPTunnelInboundOptions {
    /// 	If incoming zero hop tunnel is allowed
    pub allow_zero_hop: Option<bool>,
    /// 	Number of redundant fail-over for tunnels in
    pub backup_quantity: Option<u8>,
    /// 	Number of IP bytes to match to determine if two routers should not be in the same tunnel. 0 to disable.
    pub ip_restriction: Option<u8>,
    /// Length of tunnels in
    pub length: Option<u8>,
    /// Random amount to add or subtract to the length of tunnels in. A positive number x means add a random amount from 0 to x inclusive. A negative number -x means add a random amount from -x to x inclusive. The router will limit the total length of the tunnel to 0 to 7 inclusive. The default variance was 1 prior to release 0.7.6.
    pub length_variance: Option<i8>,
    ///  	Number of tunnels in. Limit was increased from 6 to 16 in release 0.9; however, numbers higher than 6 are incompatible with older releases.
    pub quantity: Option<u8>,
    ///  	Used for consistent peer ordering across restarts.
    pub random_key: Option<String>,
}

#[derive(Default)]
pub struct I2CPTunnelOutboundOptions {
    /// 	If outgoing zero hop tunnel is allowed
    pub allow_zero_hop: Option<bool>,
    /// 	Number of redundant fail-over for tunnels out
    pub backup_quantity: Option<u8>,
    /// 	Number of IP bytes to match to determine if two routers should not be in the same tunnel. 0 to disable.
    pub ip_restriction: Option<u8>,
    /// Length of tunnels out
    pub length: Option<u8>,
    /// Random amount to add or subtract to the length of tunnels in. A positive number x means add a random amount from 0 to x inclusive. A negative number -x means add a random amount from -x to x inclusive. The router will limit the total length of the tunnel to 0 to 7 inclusive. The default variance was 1 prior to release 0.7.6.
    pub length_variance: Option<i8>,
    /// 	Priority adjustment for outbound messages. Higher is higher priority.
    pub priority: Option<i8>,
    ///  	Number of tunnels in. Limit was increased from 6 to 16 in release 0.9; however, numbers higher than 6 are incompatible with older releases.
    pub quantity: Option<u8>,
    ///  	Used for consistent peer ordering across restarts.
    pub random_key: Option<String>,
}



pub struct I2CPClientOptions {
    /// 	(ms) Idle time required (default 30 minutes)
    pub close_idle_time: Option<u64>,
    /// 	Close I2P session when idle
    pub close_on_idle: Option<bool>,
    /// Encrypt the lease
    pub encrypt_lease_set: Option<bool>,
    /// 	If true, the router just sends the MessagePayload instead of sending a MessageStatus and awaiting a ReceiveMessageBegin.
    pub fast_receive: Option<bool>,
    /// Gzip outbound data
    pub gzip: Option<bool>,
    /// The type of authentication for encrypted LS2. 0 for no per-client authentication (the default); 1 for DH per-client authentication; 2 for PSK per-client authentication. See proposal 123. 
    pub lease_set_auth_type: Option<LeaseSetAuthType>,
    /// The sig type of the blinded key for encrypted LS2. Default depends on the destination sig type. See proposal 123. 
    pub lease_set_blinded_type: Option<LeaseSetBlindedType>,
    pub lease_set_enc_type: Option<LeaseSetEncType>,
}
/// The base 64 of the offline signature. See proposal 123. 
pub struct LeaseSetOfflineSignature(String);

/// The encryption type to be used, as of 0.9.38. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. As of 0.9.39, may be comma-separated values for multiple types. See PublicKey in common strutures spec for values. See proposals 123, 144, and 145. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetEncType(String);

/// A base 64 X25519 private key for the router to use to decrypt the encrypted LS2 locally, only if per-client authentication is enabled. Optionally preceded by the key type and ':'. Only "ECIES_X25519:" is supported, which is the default. See proposal 123. Do not confuse with i2cp.leaseSetPrivateKey which is for the leaseset encryption keys. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetPrivKey(String);

/// Base 64 encoded UTF-8 secret used to blind the leaseset address. See proposal 123. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetSecret(String);

///  The base 64 of the transient private key, prefixed by an optional sig type number or name, default DSA_SHA1. See proposal 123. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetTransientPublicKey(String);

/// The expiration of the offline signature, 4 bytes, seconds since the epoch. See proposal 123. 
pub type LeaseSetOfflineExpiration = [u8; 4];

/// The type of leaseset to be sent in the CreateLeaseSet2 Message. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. See proposal 123.
pub struct LeaseSetType(u8);

/// The sig type of the blinded key for encrypted LS2. Default depends on the destination sig type. See proposal 123. 
pub type LeaseSetBlindedType = u16;

/// The type of authentication for encrypted LS2. 0 for no per-client authentication (the default); 1 for DH per-client authentication; 2 for PSK per-client authentication. See proposal 123. 
#[derive(Debug)]
#[repr(u64)]
pub enum LeaseSetAuthType {
    NoPerClient = 0_u64,
    DHPerClient = 1_u64,
    PSKPerClient = 2_u64,
}

impl Default for LeaseSetEncType {
    fn default() -> LeaseSetEncType { LeaseSetEncType::from("4,0") }
}

impl From<String> for LeaseSetEncType {
    fn from(val: String) -> LeaseSetEncType { LeaseSetEncType(val) }
}

impl From<&str> for LeaseSetEncType { 
    fn from(val: &str) -> LeaseSetEncType { LeaseSetEncType(val.to_string()) }
}

impl Default for LeaseSetAuthType {
    fn default() -> Self { Self::NoPerClient }
}

impl Default for LeaseSetType {
    fn default() -> Self { LeaseSetType(1) }
}
/// Guaranteed is disabled; None implemented in 0.8.1; the streaming lib default is None as of 0.8.1, the client side default is None as of 0.9.4
pub enum MessageReliability {
    BestEffort,
    None,
}

impl Default for MessageReliability {
    fn default() -> Self { Self::None }
}

impl MessageReliability {
    pub fn string(&self) -> String {
        match self {
            Self::BestEffort => String::from("BestEffort"),
            Self::None => String::from("None"),
        }
    }
}


impl Default for SAMOptions {
    fn default() -> SAMOptions {
        SAMOptions { 
            from_port: None,
            to_port: None,
            i2cp: Some(I2CP::default())
        }
    }
}

impl Default for I2CP {
    fn default() -> I2CP  {
        I2CP {
            router_options: None,
        }
    }
}