

#![ allow (dead_code) ]




pub mod ascii {
	// NOTE:  python -c 'for c in range (32, 127) : print ("pub const C%02X : char = %r;" % (c, chr(c)))'
	include! ("./_generated/patterns_consts_ascii.in");
}




pub mod mnemonic {
	// NOTE:  => https://github.com/singpolyma/mnemonicode
	// NOTE:  => https://github.com/mbrubeck/rust-mnemonic
	include! ("./_generated/patterns_consts_mnemonic.in");
}




pub mod bip0039 {
	// NOTE:  => https://en.bitcoin.it/wiki/BIP_0039
	// NOTE:  => https://github.com/maciejhirsz/tiny-bip39
	include! ("./_generated/patterns_consts_bip0039.in");
}




pub mod skey {
	// NOTE:  => https://en.wikipedia.org/wiki/S/KEY
	// NOTE:  => https://www.ietf.org/rfc/rfc1760.html
	// NOTE:  => https://www.ietf.org/rfc/rfc2289.html
	include! ("./_generated/patterns_consts_skey.in");
}


