

use crate::prelude::*;




pub struct EntropyEstimates {
	
	pub entropy_bits : f64,
	
	pub for_cryptography : bool,
	pub for_cryptography_margin_bits : f64,
	
	pub for_authentication : bool,
	pub for_authentication_margin_bits : f64,
	
	pub for_archival : bool,
	pub for_archival_margin_bits : f64,
	
	pub for_long_term : bool,
	pub for_long_term_margin_bits : f64,
	
	pub for_short_term : bool,
	pub for_short_term_margin_bits : f64,
	
	pub bruteforce_hours : Vec<(Cow<'static, str>, Option<f64>)>,
}








// NOTE:  =>  https://hashcat.net/wiki/doku.php?id=hashcat#supported_attack_modes
// NOTE:  =>  https://gist.github.com/Chick3nman/32e662a5bb63bc4f51b847bb422222fd
/*
      # | Name                                                       | Category
  ======+============================================================+======================================
    900 | MD4                                                        | Raw Hash
      0 | MD5                                                        | Raw Hash
    100 | SHA1                                                       | Raw Hash
   1400 | SHA2-256                                                   | Raw Hash
  17400 | SHA3-256                                                   | Raw Hash
  11900 | PBKDF2-HMAC-MD5                                            | Generic KDF (iterations 999)
  12000 | PBKDF2-HMAC-SHA1                                           | Generic KDF (iterations 999)
  10900 | PBKDF2-HMAC-SHA256                                         | Generic KDF (iterations 999)
  12100 | PBKDF2-HMAC-SHA512                                         | Generic KDF (iterations 999)
   8900 | scrypt                                                     | Generic KDF (iterations 16384)
  17010 | GPG (AES-128/AES-256 (SHA-1($pass)))                       | Raw Hash    (iterations 65536)
  26401 | AES-128-ECB NOKDF (PT = $salt, key = $pass)                | Raw Cipher, Known-plaintext attack
   2000 | STDOUT                                                     | Plaintext
*/


pub const BRUTEFORCE_UNITS_HPS : &[(&str, u64)] = &[
		("MD4", 291 * 1_000_000_000),
		("MD5", 165 * 1_000_000_000),
		("SHA1", 51 * 1_000_000_000),
		("SHA2-256", 22 * 1_000_000_000),
		("SHA3-256", 5059 * 1_000_000),
		("PBKDF2-HMAC-MD5", 46170 * 1_000),
		("PBKDF2-HMAC-SHA1", 19125 * 1_000),
		("PBKDF2-HMAC-SHA256", 8866 * 1_000),
		("PBKDF2-HMAC-SHA512", 3121 * 1_000),
		("scrypt", 7126),
		("GPG", 26835 * 1_000),
		("AES-128", 21901 * 1_000_000),
	];

pub const BRUTEFORCE_UNITS_COUNT : u64 = 1000;
pub const BRUTEFORCE_LUCK_FACTOR : u64 = 10;
pub const BRUTEFORCE_SECURITY_FACTOR : u64 = 10;




pub const FOR_STORAGE_UNIT_HPS : u64 = 46170 * 1_000;
pub const FOR_STORAGE_UNIT_COUNT : u64 = 1000;
pub const FOR_STORAGE_LUCK_FACTOR : u64 = 10;
pub const FOR_STORAGE_SECURITY_FACTOR : u64 = 10;

// NOTE:  Let's assume performance doubles each 5 years:  `2**(1/5)`
pub const FOR_STORAGE_PERF_FACTOR : f64 = 1.15;

// NOTE:  Let's assume cost halves each 10 years:  `2**(1/10)`
pub const FOR_STORAGE_UNITS_FACTOR : f64 = 1.08;

pub const FOR_ARCHIVAL_YEARS : u64 = 100;
pub const FOR_LONG_TERM_YEARS : u64 = 25;
pub const FOR_SHORT_TERM_YEARS : u64 = 5;


pub const FOR_CRYPTOGRAPHY_MIN_BITS : u64 = 128;
pub const FOR_AUTHENTICATION_MIN_BITS : u64 = 32;








pub fn entropy_estimates (_entropy : &Entropy) -> EntropyResult<EntropyEstimates> {
	
	
	let _entropy_bits = _entropy.bits ();
	
	
	
	
	fn _bruteforce_bits_per_hour (_unit_hps : f64, _unit_count : f64) -> f64 {
		
		let _bits =
				f64::log2 (_unit_hps) +
				f64::log2 (_unit_count as f64) +
				f64::log2 (3600.0);
		
		debug_assert! (! _bits.is_nan (), "[732c4ada]");
		debug_assert! (_bits.is_finite (), "[5ab30220]");
		
		_bits
	}
	
	
	
	
	fn _bruteforce_bits_in_years (_base_unit_hps : f64, _base_unit_count : f64, _yearly_hps_factor : f64, _yearly_count_factor : f64, _years_count : u64) -> f64 {
		
		let mut _unit_hps = _base_unit_hps;
		let mut _unit_count = _base_unit_count;
		
		let mut _total_tera_hashes = 0.0;
		
		for _ in 0 .. _years_count {
			
			let _tera_hashes_per_year_bits =
					_bruteforce_bits_per_hour (_unit_hps, _unit_count)
					+ f64::log2 (24.0 * 365.25)
					- f64::log2 (1_000_000_000_000.0);
			
			_total_tera_hashes += f64::powf (2.0, _tera_hashes_per_year_bits);
			
			_unit_hps *= _yearly_hps_factor;
			_unit_count *= _yearly_count_factor;
			
			debug_assert! (! _total_tera_hashes.is_nan (), "[6e3898bc]");
			debug_assert! (_total_tera_hashes.is_finite (), "[900c14b8]");
		}
		
		let _total_bits = f64::log2 (_total_tera_hashes) + f64::log2 (1_000_000_000_000.0);
		
		debug_assert! (! _total_bits.is_nan (), "[d7bd0c84]");
		debug_assert! (_total_bits.is_finite (), "[bdf77e79]");
		
		_total_bits
	}
	
	
	
	
	let mut _bruteforce_hours = Vec::with_capacity (BRUTEFORCE_UNITS_HPS.len ());
	for (_identifier, _unit_hps) in BRUTEFORCE_UNITS_HPS {
		
		let _hours_bits =
				_entropy_bits
				- _bruteforce_bits_per_hour (*_unit_hps as f64, BRUTEFORCE_UNITS_COUNT as f64)
				- f64::log2 (BRUTEFORCE_SECURITY_FACTOR as f64)
				- f64::log2 (BRUTEFORCE_LUCK_FACTOR as f64);
		
		let _hours_count = f64::powf (2.0, _hours_bits);
		
		debug_assert! (! _hours_count.is_nan (), "[360d61a8]");
		
		let _hours_count = if _hours_count.is_finite () {
				debug_assert! (_hours_count.is_finite (), "[bd1b1cbc]");
				Some (_hours_count)
			} else {
				None
			};
		
		_bruteforce_hours.push ((Cow::Borrowed (*_identifier), _hours_count));
	}
	
	
	
	
	let _for_cryptography_margin_bits = _entropy_bits - (FOR_CRYPTOGRAPHY_MIN_BITS as f64);
	let _for_authentication_margin_bits = _entropy_bits - (FOR_AUTHENTICATION_MIN_BITS as f64);
	
	fn _for_storage_margin_bits (_years : u64, _entropy_bits : f64) -> f64 {
		
		let _margin_bits =
				_entropy_bits
				- _bruteforce_bits_in_years (FOR_STORAGE_UNIT_HPS as f64, BRUTEFORCE_UNITS_COUNT as f64, FOR_STORAGE_PERF_FACTOR, FOR_STORAGE_UNITS_FACTOR, _years)
				- f64::log2 (FOR_STORAGE_SECURITY_FACTOR as f64)
				- f64::log2 (FOR_STORAGE_LUCK_FACTOR as f64);
		
		_margin_bits
	}
	
	let _for_archival_margin_bits = _for_storage_margin_bits (FOR_ARCHIVAL_YEARS, _entropy_bits);
	let _for_long_term_margin_bits = _for_storage_margin_bits (FOR_LONG_TERM_YEARS, _entropy_bits);
	let _for_short_term_margin_bits = _for_storage_margin_bits (FOR_SHORT_TERM_YEARS, _entropy_bits);
	
	
	
	let _estimates = EntropyEstimates {
			
			entropy_bits : _entropy_bits,
			
			for_cryptography : _for_cryptography_margin_bits >= 0.0,
			for_cryptography_margin_bits : _for_cryptography_margin_bits,
			
			for_authentication : _for_authentication_margin_bits >= 0.0,
			for_authentication_margin_bits : _for_authentication_margin_bits,
			
			for_archival : _for_archival_margin_bits >= 0.0,
			for_archival_margin_bits : _for_archival_margin_bits,
			
			for_long_term : _for_long_term_margin_bits >= 0.0,
			for_long_term_margin_bits : _for_long_term_margin_bits,
			
			for_short_term : _for_short_term_margin_bits >= 0.0,
			for_short_term_margin_bits : _for_short_term_margin_bits,
			
			bruteforce_hours : _bruteforce_hours,
		};
	
	
	Ok (_estimates)
}


