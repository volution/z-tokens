

#![ allow (unused_macros) ]
#![ allow (unused_imports) ]




use crate::prelude::*;


use crate::patterns_glyphs as glyphs;
use crate::patterns_separators as separators;
use crate::patterns_macros as macros;


include! ("./patterns_tokens_macros.in");








#[ cfg (feature = "zt-patterns-digits-base10") ]
define_repeat! (
		pub DIGITS_BASE10,
		("digits-base10", "encoding", "digits", "password", "pronounceable", "memorable"),
		("digits-base10", "d"),
		{ glyphs::DIGIT_BASE10_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base2") ]
define_repeat! (
		pub DIGITS_BASE2,
		("digits-base2", "encoding", "digits"),
		("digits-base2"),
		{ glyphs::DIGIT_BASE2_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN },
		(32 : 4));

#[ cfg (feature = "zt-patterns-digits-base8") ]
define_repeat! (
		pub DIGITS_BASE8,
		("digits-base8", "encoding", "digits"),
		("digits-base8"),
		{ glyphs::DIGIT_BASE8_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base16") ]
define_repeat! (
		pub DIGITS_BASE16,
		("digits-base16", "encoding", "password"),
		("digits-base16", "x"),
		{ glyphs::DIGIT_BASE16_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base32") ]
define_repeat! (
		pub DIGITS_BASE32_HEX,
		("digits-base32-hex", "digits-base32", "encoding", "password"),
		("digits-base32-hex"),
		{ glyphs::DIGIT_BASE32_HEX_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base32") ]
define_repeat! (
		pub DIGITS_BASE32_RFC,
		("digits-base32-rfc", "digits-base32", "encoding", "password"),
		("digits-base32-rfc"),
		{ glyphs::DIGIT_BASE32_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base64") ]
define_repeat! (
		pub DIGITS_BASE64_URL,
		("digits-base64-url", "digits-base64", "encoding", "password"),
		("digits-base64-url"),
		{ glyphs::DIGIT_BASE64_URL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base64") ]
define_repeat! (
		pub DIGITS_BASE64_RFC,
		("digits-base64-rfc", "digits-base64", "encoding", "password"),
		("digits-base64-rfc"),
		{ glyphs::DIGIT_BASE64_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base58") ]
define_repeat! (
		pub DIGITS_BASE58,
		("digits-base58", "encoding", "password"),
		("digits-base58"),
		{ glyphs::DIGIT_BASE58_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base62") ]
define_repeat! (
		pub DIGITS_BASE62,
		("digits-base62", "encoding", "password"),
		("digits-base62"),
		{ glyphs::DIGIT_BASE62_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-bech32") ]
define_repeat! (
		pub DIGITS_BECH32,
		("digits-bech32", "encoding", "password"),
		("digits-bech32"),
		{ glyphs::DIGIT_BECH32_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-z85") ]
define_repeat! (
		pub DIGITS_Z85,
		("digits-z85", "encoding", "password"),
		("digits-z85"),
		{ glyphs::DIGIT_Z85_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN },
		(35 : 5));








#[ cfg (feature = "zt-patterns-bytes") ]
define_bytes! (
		pub BYTES_HEX,
		("bytes-hex", "encoding", "password"),
		("bytes-hex", "b"),
		BYTES_HEX,
		(512 : 1));








#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_LOWER,
		("ascii-lower", "letters", "password"),
		("ascii-lower"),
		{ glyphs::ASCII_LETTER_LOWER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_UPPER,
		("ascii-upper", "letters"),
		("ascii-upper"),
		{ glyphs::ASCII_LETTER_UPPER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_MIXED,
		("ascii-mixed", "letters", "password"),
		("ascii-mixed"),
		{ glyphs::ASCII_LETTER_MIXED_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));


#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_SYMBOLS,
		("ascii-symbols"),
		("ascii-symbols"),
		{ glyphs::ASCII_SYMBOL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_PRINTABLE,
		("ascii-any", "password"),
		("ascii-any", "r"),
		{ glyphs::ASCII_PRINTABLE_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));








#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_LOWER_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_UPPER_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_MIXED_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_LOWER,
		("cva-lower", "cva", "cv", "letters", "password", "pronounceable", "memorable"),
		("cva-lower", "cva"),
		{ ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_UPPER,
		("cva-upper", "cva", "cv", "letters"),
		("cva-upper"),
		{ ASCII_CONSONANT_VOWEL_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_MIXED,
		("cva-mixed", "cva", "cv", "letters", "password"),
		("cva-mixed"),
		{ ASCII_CONSONANT_VOWEL_MIXED_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_A_WORD,
		(),
		(),
		[
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_B_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_C_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::ASCII_SYMBOL_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_A,
		("cva-plus-a", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-a", "cvapa"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_A_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_B,
		("cva-plus-b", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-b", "cvapb"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_B_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_C,
		("cva-plus-c", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-c", "cvapc"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_C_WORD, )
		},
		(15 : 1, number_plus_one));








#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_LOWER_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_LOWER_TOKEN,
			glyphs::SIMPLE_VOWEL_LOWER_TOKEN,
			glyphs::SIMPLE_CONSONANT_LOWER_TOKEN,
			glyphs::SIMPLE_VOWEL_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_UPPER_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_MIXED_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_MIXED_TOKEN,
			glyphs::SIMPLE_VOWEL_MIXED_TOKEN,
			glyphs::SIMPLE_CONSONANT_MIXED_TOKEN,
			glyphs::SIMPLE_VOWEL_MIXED_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_LOWER,
		("cvs-lower", "cvs", "cv", "letters", "password", "pronounceable", "memorable"),
		("cvs-lower", "cvs"),
		{ SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_UPPER,
		("cvs-upper", "cvs", "cv", "letters"),
		("cvs-upper"),
		{ SIMPLE_CONSONANT_VOWEL_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_MIXED,
		("cvs-mixed", "cvs", "cv", "letters", "password"),
		("cvs-mixed"),
		{ SIMPLE_CONSONANT_VOWEL_MIXED_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_A_WORD,
		(),
		(),
		[
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_B_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_C_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::ASCII_SYMBOL_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_A,
		("cvs-plus-a", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-a", "cvspa"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_A_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_B,
		("cvs-plus-b", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-b", "cvspb"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_B_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_C,
		("cvs-plus-c", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-c", "cvspc"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_C_WORD, )
		},
		(15 : 1, number_plus_one));








#[ cfg (feature = "zt-patterns-proquint") ]
define_sequence! (
		pub PROQUINT_LOWER_WORD,
		(),
		(),
		[
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-proquint") ]
define_sequence! (
		pub PROQUINT_UPPER_WORD,
		(),
		(),
		[
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-proquint") ]
define_repeat! (
		pub PROQUINT_LOWER,
		("proquint-lower", "proquint", "letters", "password", "pronounceable", "memorable", "encoding"),
		("proquint-lower", "proquint"),
		{ PROQUINT_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-proquint") ]
define_repeat! (
		pub PROQUINT_UPPER,
		("proquint-upper", "proquint", "letters"),
		("proquint-upper"),
		{ PROQUINT_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));








#[ cfg (feature = "zt-patterns-koremutake") ]
define_sequence! (
		pub KOREMUTAKE_WORD_A,
		(),
		(),
		[
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-koremutake") ]
define_sequence! (
		pub KOREMUTAKE_WORD_B,
		(),
		(),
		[
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-koremutake") ]
define_repeat! (
		pub KOREMUTAKE_A,
		("koremutake-a", "koremutake", "letters", "password", "pronounceable", "memorable", "encoding"),
		("koremutake-a"),
		{ KOREMUTAKE_WORD_A => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-koremutake") ]
define_repeat! (
		pub KOREMUTAKE_B,
		("koremutake-b", "koremutake", "letters", "password", "pronounceable", "memorable", "encoding"),
		("koremutake-b"),
		{ KOREMUTAKE_WORD_B => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));








#[ cfg (feature = "zt-patterns-mnemonic") ]
define_repeat! (
		pub MNEMONIC,
		("mnemonic", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("mnemonic"),
		{ glyphs::MNEMONIC_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-bip0039") ]
define_repeat! (
		pub BIP0039,
		("bip0039", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("bip0039"),
		{ glyphs::BIP0039_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-skey") ]
define_repeat! (
		pub SKEY,
		("skey", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("skey"),
		{ glyphs::SKEY_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-pgp") ]
define_sequence! (
		pub PGP_TUPLE,
		(),
		(),
		[
			glyphs::PGP_EVEN_WORD_TOKEN,
			glyphs::PGP_ODD_WORD_TOKEN,
		], separators::SPACE_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-pgp") ]
define_repeat! (
		pub PGP,
		("pgp", "dictionary", "pronounceable", "encoding"),
		("pgp"),
		{ PGP_TUPLE => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-eff-large") ]
define_repeat! (
		pub EFF_LARGE,
		("eff-large", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-large"),
		{ glyphs::EFF_LARGE_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));

#[ cfg (feature = "zt-patterns-eff-short") ]
define_repeat! (
		pub EFF_SHORT,
		("eff-short", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-short"),
		{ glyphs::EFF_SHORT_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));

#[ cfg (feature = "zt-patterns-eff-unique") ]
define_repeat! (
		pub EFF_UNIQUE,
		("eff-unique", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-unique"),
		{ glyphs::EFF_UNIQUE_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));








#[ cfg (feature = "zt-patterns-nato") ]
define_repeat! (
		pub NATO,
		("nato", "dictionary", "pronounceable"),
		("nato"),
		{ glyphs::NATO_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(66 : 1));








#[ cfg (feature = "zt-patterns-uuid") ]
define_sequence! (
		pub UUID_V4,
		("uuid-v4", "uuid", "password"),
		("uuid-v4"), [
			glyphs::UUID_ANY_FIELD_1_TOKEN,
			glyphs::UUID_ANY_FIELD_2_TOKEN,
			glyphs::UUID_V4_FIELD_3_TOKEN,
			glyphs::UUID_V4_FIELD_4_TOKEN,
			glyphs::UUID_ANY_FIELD_5_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-uuid") ]
define_all! (pub UUID_ALL, [
		UUID_V4,
	]);








#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_127_PREFIX, Str, "127");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_10_PREFIX, Str, "10");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_172_PREFIX, Str, "172");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_192_A_PREFIX, Str, "192");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_192_B_PREFIX, Str, "168");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_MAC_PREFIX, Str, "02");

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_127,
		("ip-127", "ip", "networking"),
		("ip-127"),
		[
			IP_127_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_10,
		("ip-10", "ip", "networking"),
		("ip-10"),
		[
			IP_10_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_172,
		("ip-172", "ip", "networking"),
		("ip-172"),
		[
			IP_172_PREFIX_TOKEN,
			glyphs::INTEGER_1_30_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_192,
		("ip-192", "ip", "networking"),
		("ip-192"),
		[
			IP_192_A_PREFIX_TOKEN,
			IP_192_B_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_MAC,
		("ip-mac", "networking"),
		("ip-mac"),
		[
			IP_MAC_PREFIX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
		], separators::COLON_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_all! (pub IP_ALL, [
		IP_127,
		IP_10,
		IP_172,
		IP_192,
		IP_MAC,
	]);








#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_DATETIME,
		("timestamp-date-time", "timestamp"),
		("timestamp-date-time", "date-time"),
		glyphs::TIMESTAMP_STRING_DATETIME_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_DATE,
		("timestamp-date", "timestamp"),
		("timestamp-date", "date"),
		glyphs::TIMESTAMP_STRING_DATE_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_TIME,
		("timestamp-time", "timestamp"),
		("timestamp-time", "time"),
		glyphs::TIMESTAMP_STRING_TIME_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_SECONDS_DEC,
		("timestamp-sec", "timestamp"),
		("timestamp-sec", "timestamp"),
		glyphs::TIMESTAMP_SECONDS_DEC_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_SECONDS_HEX,
		("timestamp-sec-hex", "timestamp"),
		("timestamp-sec-hex"),
		glyphs::TIMESTAMP_SECONDS_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_NANOSECONDS_DEC,
		("timestamp-nano", "timestamp"),
		("timestamp-nano"),
		glyphs::TIMESTAMP_NANOSECONDS_DEC_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_NANOSECONDS_HEX,
		("timestamp-nano-hex", "timestamp"),
		("timestamp-nano-hex"),
		glyphs::TIMESTAMP_NANOSECONDS_HEX_TOKEN);


#[ cfg (any (feature = "zt-patterns-timestamp", feature = "zt-patterns-flake")) ]
define_named! (
		pub TIMESTAMP_FLAKE_SECONDS_DEC,
		("timestamp-flake", "timestamp"),
		("timestamp-flake"),
		glyphs::TIMESTAMP_FLAKE_SECONDS_DEC_TOKEN);

#[ cfg (any (feature = "zt-patterns-timestamp", feature = "zt-patterns-flake")) ]
define_named! (
		pub TIMESTAMP_FLAKE_SECONDS_HEX,
		("timestamp-flake-hex", "timestamp"),
		("timestamp-flake-hex"),
		glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN);


#[ cfg (feature = "zt-patterns-timestamp") ]
define_all! (pub TIMESTAMP_ALL, [
		TIMESTAMP_STRING_DATETIME,
		TIMESTAMP_STRING_DATE,
		TIMESTAMP_STRING_TIME,
		TIMESTAMP_SECONDS_DEC,
		TIMESTAMP_SECONDS_HEX,
		TIMESTAMP_NANOSECONDS_DEC,
		TIMESTAMP_NANOSECONDS_HEX,
		TIMESTAMP_FLAKE_SECONDS_DEC,
		TIMESTAMP_FLAKE_SECONDS_HEX,
	]);


#[ cfg (feature = "zt-patterns-flake") ]
define_repeat! (
		pub FLAKE_SECONDS,
		("flake", "timestamp"),
		("flake"),
		{
			( glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN, separators::HYPHEN_OPTIONAL_TOKEN, ),
			( glyphs::BYTES_HEX_4_TOKEN => separators::HYPHEN_OPTIONAL_INFIX_PATTERN ),
			()
		},
		(16 : 1, number_times_four_in_bits));








pub static ALL : &[&[Rb<TokenPattern>]] = &[
		
		#[ cfg (feature = "zt-patterns-digits-base10") ]
		DIGITS_BASE10_ALL,
		#[ cfg (feature = "zt-patterns-digits-base2") ]
		DIGITS_BASE2_ALL,
		#[ cfg (feature = "zt-patterns-digits-base8") ]
		DIGITS_BASE8_ALL,
		#[ cfg (feature = "zt-patterns-digits-base16") ]
		DIGITS_BASE16_ALL,
		#[ cfg (feature = "zt-patterns-digits-base32") ]
		DIGITS_BASE32_HEX_ALL,
		#[ cfg (feature = "zt-patterns-digits-base32") ]
		DIGITS_BASE32_RFC_ALL,
		#[ cfg (feature = "zt-patterns-digits-base64") ]
		DIGITS_BASE64_URL_ALL,
		#[ cfg (feature = "zt-patterns-digits-base64") ]
		DIGITS_BASE64_RFC_ALL,
		#[ cfg (feature = "zt-patterns-digits-base58") ]
		DIGITS_BASE58_ALL,
		#[ cfg (feature = "zt-patterns-digits-base62") ]
		DIGITS_BASE62_ALL,
		#[ cfg (feature = "zt-patterns-digits-bech32") ]
		DIGITS_BECH32_ALL,
		#[ cfg (feature = "zt-patterns-digits-z85") ]
		DIGITS_Z85_ALL,
		
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_SYMBOLS_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_PRINTABLE_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_A_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_B_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_C_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_A_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_B_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_C_ALL,
		
		#[ cfg (feature = "zt-patterns-proquint") ]
		PROQUINT_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-proquint") ]
		PROQUINT_UPPER_ALL,
		
		#[ cfg (feature = "zt-patterns-koremutake") ]
		KOREMUTAKE_A_ALL,
		#[ cfg (feature = "zt-patterns-koremutake") ]
		KOREMUTAKE_B_ALL,
		
		#[ cfg (feature = "zt-patterns-mnemonic") ]
		MNEMONIC_ALL,
		#[ cfg (feature = "zt-patterns-bip0039") ]
		BIP0039_ALL,
		#[ cfg (feature = "zt-patterns-skey") ]
		SKEY_ALL,
		#[ cfg (feature = "zt-patterns-pgp") ]
		PGP_ALL,
		#[ cfg (feature = "zt-patterns-eff-large") ]
		EFF_LARGE_ALL,
		#[ cfg (feature = "zt-patterns-eff-short") ]
		EFF_SHORT_ALL,
		#[ cfg (feature = "zt-patterns-eff-unique") ]
		EFF_UNIQUE_ALL,
		
		#[ cfg (feature = "zt-patterns-nato") ]
		NATO_ALL,
		
		#[ cfg (feature = "zt-patterns-bytes") ]
		BYTES_HEX_ALL,
		
		#[ cfg (feature = "zt-patterns-uuid") ]
		UUID_ALL,
		
		#[ cfg (feature = "zt-patterns-ip") ]
		IP_ALL,
		
		#[ cfg (feature = "zt-patterns-timestamp") ]
		TIMESTAMP_ALL,
		#[ cfg (feature = "zt-patterns-flake") ]
		FLAKE_SECONDS_ALL,
		
	];


