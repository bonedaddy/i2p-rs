//! objects used for configuration SAM sessions

//! options used when interacting with the SAM bridge
pub struct SAMOptions<'a> {
    pub i2cp: Option<I2CP<'a>>
}

impl<'a> SAMOptions<'a> {
    pub fn string(&self) -> String { 
        let mut options = String::default();
        if let Some(i2cp_options) = &self.i2cp {
            options.push_str(&format!("{}", i2cp_options.string()));
        }
        options
    }
}

/// I2CP options taken from https://geti2p.net/en/docs/protocol/i2cp
pub struct I2CP<'a> {
    /// encrypt the lease set
    pub encrypt_lease_set: Option<bool>,
    /// true, the router just sends the MessagePayload instead of sending a MessageStatus and awaiting a ReceiveMessageBegin.
    pub fast_receive: Option<bool>,
    /// gzip outbound data
    pub gzip: Option<bool>,
    /// The type of authentication for encrypted LS2. 0 for no per-client authentication (the default); 1 for DH per-client authentication; 2 for PSK per-client authentication. See proposal 123. 
    pub lease_set_auth_type: Option<u8>,
    /// The sig type of the blinded key for encrypted LS2. Default depends on the destination sig type. See proposal 123. 
    pub lease_set_blinded_type: Option<u16>,
    /// The encryption type to be used, as of 0.9.38. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. As of 0.9.39, may be comma-separated values for multiple types. See also i2cp.leaseSetPrivateKey. See PublicKey in common strutures spec for values. See proposals 123, 144, and 145. 
    pub lease_set_enc_type: Option<&'a str>,
    /// For encrypted leasesets. Base 64 SessionKey (44 characters)
    pub lease_set_key: Option<&'a str>,
    /// Base 64 private keys for encryption. Optionally preceded by the encryption type name or number and ':'. For LS1, only one key is supported, and only "0:" or "ELGAMAL_2048:" is supported, which is the default. As of 0.9.39, for LS2, multiple keys may be comma-separated, and each key must be a different encryption type. I2CP will generate the public key from the private key. Use for persistent leaseset keys across restarts. See proposals 123, 144, and 145. See also i2cp.leaseSetEncType. Do not confuse with i2cp.leaseSetPrivKey which is for encrypted LS2. 
    pub lease_set_private_key: Option<&'a str>,
    /// Base 64 encoded UTF-8 secret used to blind the leaseset address. See proposal 123. 
    pub lease_set_secret: Option<&'a str>,
    /// Base 64 private key for signatures. Optionally preceded by the key type and ':'. DSA_SHA1 is the default. Key type must match the signature type in the destination. I2CP will generate the public key from the private key. Use for persistent leaseset keys across restarts.
    pub lease_set_signing_private_key: Option<&'a str>,
    /// The type of leaseset to be sent in the CreateLeaseSet2 Message. Interpreted client-side, but also passed to the router in the SessionConfig, to declare intent and check support. See proposal 123. 
    pub lease_set_type: Option<u8>,
    /// 	Guaranteed is disabled; None implemented in 0.8.1; None is the default as of 0.9.4
    pub message_reliability: Option<I2CPMessageReliability>,
    /// 	(ms) Idle time required (default 20 minutes, minimum 5 minutes)
    pub reduce_idle_time: Option<u64>,
    /// 	Reduce tunnel quantity when idle
    pub reduce_on_idle: Option<bool>,
    /// Tunnel quantity when reduced (applies to both inbound and outbound)
    pub reduce_quantity: Option<u8>,
}

impl<'a> I2CP<'a> {
    pub fn string(&self) -> String {
        let mut options = String::default();
        if let Some(encrypt_lease_set) = self.encrypt_lease_set {
            options.push_str(&format!("i2cp.encryptLeaseSet={} ", encrypt_lease_set));
        }
        if let Some(fast_receive) = self.fast_receive {
            options.push_str(&format!("i2cp.fastReceive={} ", fast_receive));
        }
        if let Some(gzip) = self.gzip {
            options.push_str(&format!("i2cp.gzip={} ", gzip));
        }
        if let Some(lease_set_auth_type) = self.lease_set_auth_type {
            options.push_str(&format!("i2cp.leaseSetAuthType={} ", lease_set_auth_type));
        }
        if let Some(lease_set_blinded_type) = self.lease_set_blinded_type {
            options.push_str(&format!("i2cp.leaseSetBlindedType={} ", lease_set_blinded_type));
        }
        if let Some(lease_set_enc_type) = self.lease_set_enc_type {
            options.push_str(&format!("i2cp.leaseSetEncType={} ", lease_set_enc_type));
        }
        if let Some(lease_set_key) = self.lease_set_key {
            options.push_str(&format!("i2cp.leaseSetKey={} ", lease_set_key));
        }
        if let Some(lease_set_private_key) = self.lease_set_private_key {
            options.push_str(&format!("i2cp.leaseSetPrivateKey={} ", lease_set_private_key));
        }
        if let Some(lease_set_secret) = self.lease_set_secret {
            options.push_str(&format!("i2cp.leaseSetSecret={} ", lease_set_secret));
        }
        if let Some(lease_set_signing_private_key) = self.lease_set_signing_private_key {
            options.push_str(&format!("i2cp.leaseSetSigningPrivateKey={} ", lease_set_signing_private_key));
        }
        if let Some(lease_set_type) = self.lease_set_type {
            options.push_str(&format!("i2cp.leaseSetType={} ", lease_set_type));
        }
        if let Some(message_reliability) = &self.message_reliability {
            options.push_str(&format!("i2cp.messageReliability={} ", message_reliability.string()));
        }
        if let Some(reduce_idle_time) = self.reduce_idle_time {
            options.push_str(&format!("i2cp.reduceIdleTime={} ", reduce_idle_time));
        }
        if let Some(reduce_on_idle) = self.reduce_on_idle {
            options.push_str(&format!("i2cp.reduceOnIdle={} ", reduce_on_idle));
        }
        if let Some(reduce_quantity) = self.reduce_quantity {
            options.push_str(&format!("i2cp.reduceQuantity={} ", reduce_quantity));
        }
        options
    }
}


pub enum I2CPMessageReliability {
    BestEffort,
    None,
}

impl Default for I2CPMessageReliability {
    fn default() -> Self { Self::None }
}

impl I2CPMessageReliability {
    pub fn string(&self) -> String {
        match self {
            Self::BestEffort => String::from("BestEffort"),
            Self::None => String::from("None"),
        }
    }
}


impl Default for SAMOptions<'_> {
    fn default() -> SAMOptions<'static> {
        SAMOptions { 
            i2cp: Some(I2CP {
                encrypt_lease_set: None,
                fast_receive: None,
                gzip: None,
                lease_set_auth_type: None,
                lease_set_blinded_type: None,
                lease_set_enc_type: Some("4,0"),
                lease_set_key: None,
                lease_set_private_key: None,
                lease_set_secret: None,
                lease_set_signing_private_key: None,
                lease_set_type: None,
                message_reliability: None,
                reduce_idle_time: None,
                reduce_on_idle: None,
                reduce_quantity: None,
            })
        }
    }
}