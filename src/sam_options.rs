//! objects used for configuration SAM sessions

/// options used when interacting with the SAM bridge
pub struct SAMOptions {
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    /// i2cp control options
    pub i2cp_options: Option<I2CPOptions>
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
        if let Some(i2cp_options) = &self.i2cp_options {
            options.push_str(&format!("{}", i2cp_options.string()));
        }
        options
    }
}

/// I2CP client and router options taken from https://geti2p.net/en/docs/protocol/i2cp
pub struct I2CPOptions {
    pub router_options: Option<I2CPRouterOptions>,
    pub client_options: Option<I2CPClientOptions>,
}



impl I2CPOptions {
    pub fn string(&self) -> String {
        let mut options = String::default();

        options
    }
}


impl I2CPRouterOptions {
    pub fn string(&self) -> String {
        let mut options = String::default();
        if let Some(client_message_timeout) = self.client_message_timeout {
            options.push_str(&format!("clientMessageTimeout={}", client_message_timeout));
        }
        if let Some(crypto_options) = self.crypto_options {
            if let Some(low_tag_threshold) = crypto_options.low_tag_threshold {
                options.push_str(&format!("crypto.lowTagThreshold={}", low_tag_threshold));
            }
            if let Some(inbound_tags) = crypto_options.ratchet_inbound_tags {
                options.push_str(&format!("crypto.ratchet.inboundTags={}", inbound_tags));
            }
            if let Some(outbound_tags) = crypto_options.ratchet_outbound_tags {
                options.push_str(&format!("crypto.ratchet.outboundTags={}", outbound_tags));
            }
            if let Some(tags_to_send) = crypto_options.tags_to_send {
                options.push_str(&format!("crypto.tagsToSend={}", tags_to_send));
            }
        }
        if let Some(dont_publish_lease_set) = self.dont_publish_lease_set {
            options.push_str(&format!("i2cp.dontPublishLeaseSet={}", dont_publish_lease_set));
        }
        if let Some(fast_receive) = self.fast_receive {
            options.push_str(&format!("i2cp.fastReceive={}", fast_receive));
        }
        if let Some(lease_set_auth_type) = self.lease_set_auth_type {
            options.push_str(&format!("i2cp.leaseSetAuthType={}", lease_set_auth_type.to_string()));
        }
        if let Some(lease_set_enc_type) = self.lease_set_enc_type {
            options.push_str(&format!("i2cp.leaseSetEncType={}", lease_set_enc_type.to_string()));
        }
        if let Some(lease_set_offline_expiration) = self.lease_set_offline_expiration {
            options.push_str(&format!("i2cp.leaseSetOfflineExpiration={}", String::from_utf8(lease_set_offline_expiration[..].to_vec()).unwrap()))
        }
        if let Some(lease_set_priv_key) = self.lease_set_priv_key {
            options.push_str(&format!("i2cp.leaseSetPrivKey={}", lease_set_priv_key.to_string()))
        }
        if let Some(lease_set_secret) = self.lease_set_secret {
            options.push_str(&format!("i2cp.leaseSetSecret={}", lease_set_secret.to_string()))
        }
        options
    }
}


pub struct I2CPRouterOptions {
    /// The timeout (ms) for all sent messages. Unused. See the protocol specification for per-message settings.
    pub client_message_timeout: Option<u32>,
    pub crypto_options: Option<I2CPRouterCryptoOptions>,
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
    ///Set to false to disable ever bundling a reply LeaseSet. For clients that do not publish their LeaseSet, this option must be true for any reply to be possible. "true" is also recommended for multihomed servers with long connection times.
    ///
    ///Setting to "false" may save significant outbound bandwidth, especially if the client is configured with a large number of inbound tunnels (Leases). If replies are still required, this may shift the bandwidth burden to the far-end client and the floodfill. There are several cases where "false" may be appropriate:
    ///
    ///    Unidirectional communication, no reply required
    ///    LeaseSet is published and higher reply latency is acceptable
    ///    LeaseSet is published, client is a "server", all connections are inbound so the connecting far-end destination obviously has the leaseset already. Connections are either short, or it is acceptable for latency on a long-lived connection to temporarily increase while the other end re-fetches the LeaseSet after expiration. HTTP servers may fit these requirements.
    ///    
    pub should_bundle_reply_info: Option<bool>,
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
    /// 	The encryption type to be used, as of 0.9.38. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. As of 0.9.39, may be comma-separated values for multiple types. See also i2cp.leaseSetPrivateKey. See PublicKey in common strutures spec for values. See proposals 123, 144, and 145. 
    pub lease_set_enc_type: Option<LeaseSetEncType>,
    /// 	For encrypted leasesets. Base 64 SessionKey (44 characters)
    pub lease_set_key: Option<LeaseSetKey>,
    /// Base 64 private keys for encryption. Optionally preceded by the encryption type name or number and ':'. For LS1, only one key is supported, and only "0:" or "ELGAMAL_2048:" is supported, which is the default. As of 0.9.39, for LS2, multiple keys may be comma-separated, and each key must be a different encryption type. I2CP will generate the public key from the private key. Use for persistent leaseset keys across restarts. See proposals 123, 144, and 145. See also i2cp.leaseSetEncType. Do not confuse with i2cp.leaseSetPrivKey which is for encrypted LS2. 
    pub lease_set_private_key: Option<LeaseSetPrivateKey>,
    /// 	Base 64 encoded UTF-8 secret used to blind the leaseset address. See proposal 123. 
    pub lease_set_secret: Option<LeaseSetSecret>,
    /// 	The type of leaseset to be sent in the CreateLeaseSet2 Message. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. See proposal 123. 
    pub lease_set_signing_private_key: Option<LeaseSetSigningPrivateKey>,
    /// Guaranteed is disabled; None implemented in 0.8.1; None is the default as of 0.9.4
    pub message_reliability: Option<MessageReliability>,
    /// (ms) Idle time required (default 20 minutes, minimum 5 minutes)
    pub reduce_idle_time: Option<u64>,
    /// 	Reduce tunnel quantity when idle
    pub reduce_on_idle: Option<bool>,
    /// Tunnel quantity when reduced (applies to both inbound and outbound)
    pub reduce_quantity: Option<u8>,
    /// Connect to the router using SSL. If the client is running in the same JVM as a router, this option is ignored, and the client connects to that router internally.
    pub ssl: Option<bool>,
    /// Router hostname. If the client is running in the same JVM as a router, this option is ignored, and the client connects to that router internally
    pub tcp_host: Option<String>,
    /// Router I2CP port. If the client is running in the same JVM as a router, this option is ignored, and the client connects to that router internally.
    pub tcp_port: Option<u8>,


}
/// The base 64 of the offline signature. See proposal 123. 
pub struct LeaseSetOfflineSignature(String);

/// The encryption type to be used, as of 0.9.38. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. As of 0.9.39, may be comma-separated values for multiple types. See PublicKey in common strutures spec for values. See proposals 123, 144, and 145. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetEncType(String);

/// A base 64 X25519 private key for the router to use to decrypt the encrypted LS2 locally, only if per-client authentication is enabled. Optionally preceded by the key type and ':'. Only "ECIES_X25519:" is supported, which is the default. See proposal 123. Do not confuse with i2cp.leaseSetPrivateKey which is for the leaseset encryption keys. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetPrivKey(String);

/// Base 64 private keys for encryption. Optionally preceded by the encryption type name or number and ':'. For LS1, only one key is supported, and only "0:" or "ELGAMAL_2048:" is supported, which is the default. As of 0.9.39, for LS2, multiple keys may be comma-separated, and each key must be a different encryption type. I2CP will generate the public key from the private key. Use for persistent leaseset keys across restarts. See proposals 123, 144, and 145. See also i2cp.leaseSetEncType. Do not confuse with i2cp.leaseSetPrivKey which is for encrypted LS2. 
pub struct LeaseSetPrivateKey(String);

/// 	For encrypted leasesets. Base 64 SessionKey (44 characters)
pub struct LeaseSetKey(String);

/// Base 64 encoded UTF-8 secret used to blind the leaseset address. See proposal 123. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetSecret(String);

///  The base 64 of the transient private key, prefixed by an optional sig type number or name, default DSA_SHA1. See proposal 123. 
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct LeaseSetTransientPublicKey(String);

/// Base 64 private key for signatures. Optionally preceded by the key type and ':'. DSA_SHA1 is the default. Key type must match the signature type in the destination. I2CP will generate the public key from the private key. Use for persistent leaseset keys across restarts.
pub struct LeaseSetSigningPrivateKey(String);


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

impl ToString for LeaseSetAuthType {
    fn to_string(&self) -> String {
        match self {
            Self::NoPerClient => String::from("0"),
            Self::DHPerClient => String::from("1"),
            Self::PSKPerClient => String::from("2"),
        }
    }
}

impl ToString for LeaseSetOfflineSignature {
    fn to_string(&self) -> String {
        self.0
    }
}

impl ToString for LeaseSetEncType {
    fn to_string(&self) -> String {
        self.0
    }
}


impl ToString for LeaseSetPrivKey {
    fn to_string(&self) -> String {
        self.0
    }
}




impl ToString for LeaseSetPrivateKey {
    fn to_string(&self) -> String {
        self.0
    }
}
impl ToString for LeaseSetKey {
    fn to_string(&self) -> String {
        self.0
    }
}

impl ToString for LeaseSetSecret {
    fn to_string(&self) -> String {
        self.0
    }
}
impl ToString for LeaseSetTransientPublicKey {
    fn to_string(&self) -> String {
        self.0
    }
}
impl ToString for LeaseSetSigningPrivateKey {
    fn to_string(&self) -> String {
        self.0
    }
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