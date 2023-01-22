pub fn rand_string(length: usize) -> String {
	use rand::distributions::{Alphanumeric, DistString};
	Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}


#[cfg(test)]
mod test {
	use crate::net::BASE64_I2P;

use super::*;
	#[test]
	fn test_rand_string() {
		let str = rand_string(32);
		println!("{}", BASE64_I2P.encode(str.as_bytes()).len());
	}
}