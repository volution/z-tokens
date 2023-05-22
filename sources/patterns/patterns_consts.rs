

#![ allow (dead_code) ]




pub mod ascii {
	// NOTE:  python -c 'for c in range (32, 127) : print ("pub const C%02X : char = %r;" % (c, chr(c)))'
	include! ("./_generated/patterns_consts_ascii.in");
}




#[ cfg (feature = "zt-patterns-mnemonic") ]
pub mod mnemonic {
	// NOTE:  => https://github.com/singpolyma/mnemonicode
	// NOTE:  => https://github.com/mbrubeck/rust-mnemonic
	include! ("./_generated/patterns_consts_mnemonic.in");
}




#[ cfg (feature = "zt-patterns-bip0039") ]
pub mod bip0039 {
	// NOTE:  => https://en.bitcoin.it/wiki/BIP_0039
	// NOTE:  => https://github.com/maciejhirsz/tiny-bip39
	include! ("./_generated/patterns_consts_bip0039.in");
}




#[ cfg (feature = "zt-patterns-skey") ]
pub mod skey {
	// NOTE:  => https://en.wikipedia.org/wiki/S/KEY
	// NOTE:  => https://www.ietf.org/rfc/rfc1760.html
	// NOTE:  => https://www.ietf.org/rfc/rfc2289.html
	include! ("./_generated/patterns_consts_skey.in");
}




#[ cfg (feature = "zt-patterns-pgp") ]
pub mod pgp {
	// NOTE:  => https://en.wikipedia.org/wiki/PGP_word_list
	include! ("./_generated/patterns_consts_pgp_even.in");
	include! ("./_generated/patterns_consts_pgp_odd.in");
}




#[ cfg (feature = "zt-patterns-eff-large") ]
pub mod eff_large {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt
	include! ("./_generated/patterns_consts_eff_large.in");
}


#[ cfg (feature = "zt-patterns-eff-short") ]
pub mod eff_short {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt
	include! ("./_generated/patterns_consts_eff_short.in");
}


#[ cfg (feature = "zt-patterns-eff-unique") ]
pub mod eff_unique {
	// NOTE:  => https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
	// NOTE:  => https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt
	include! ("./_generated/patterns_consts_eff_unique.in");
}




#[ cfg (feature = "zt-patterns-pets-medium") ]
pub mod pets_medium {
	// NOTE:  => https://github.com/dustinkirkland/petname
	// NOTE:  => https://github.com/dustinkirkland/petname/tree/master/usr/share/petname/medium
	include! ("./_generated/patterns_consts_pets_medium.in");
}


#[ cfg (feature = "zt-patterns-pets-small") ]
pub mod pets_small {
	// NOTE:  => https://github.com/dustinkirkland/petname
	// NOTE:  => https://github.com/dustinkirkland/petname/tree/master/usr/share/petname/small
	include! ("./_generated/patterns_consts_pets_small.in");
}


#[ cfg (any (feature = "zt-patterns-pets-medium", feature = "zt-patterns-pets-small")) ]
pub mod pets_common {
	// NOTE:  => https://raw.githubusercontent.com/johnd/server-name
	// NOTE:  => https://raw.githubusercontent.com/johnd/server-name/master/lib/rgb.txt
	include! ("./_generated/patterns_consts_pets_common.in");
}


