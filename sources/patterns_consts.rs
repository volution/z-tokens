

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




pub mod pgp {
	// NOTE:  => https://en.wikipedia.org/wiki/PGP_word_list
	include! ("./_generated/patterns_consts_pgp_even.in");
	include! ("./_generated/patterns_consts_pgp_odd.in");
}




pub mod eff_large {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt
	include! ("./_generated/patterns_consts_eff_large.in");
}


pub mod eff_short {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt
	include! ("./_generated/patterns_consts_eff_short.in");
}


pub mod eff_unique {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt
	include! ("./_generated/patterns_consts_eff_unique.in");
}

