

package main




import (
		"bytes"
		"encoding/binary"
		"fmt"
		"io"
		"os"
	)


import (
		
		"golang.org/x/crypto/curve25519"
		"golang.org/x/crypto/chacha20"
		"golang.org/x/crypto/argon2"
		
		"github.com/zeebo/blake3"
		"github.com/andybalholm/brotli"
		
		"github.com/btcsuite/btcd/btcutil/bech32"
	)








const BECH32_RECIPIENT_PRIVATE_PREFIX = "ztxrk"
const BECH32_RECIPIENT_PUBLIC_PREFIX = "ztxrp"

const BECH32_SENDER_PRIVATE_PREFIX = "ztxsk"
const BECH32_SENDER_PUBLIC_PREFIX = "ztxsp"

const BECH32_SECRET_PREFIX = "ztxcs"




const PACKET_PADDING_SIZE = 256
const PACKET_MAC_SIZE = 32
const PACKET_SALT_SIZE = 32




const ARGON_SECRET_M_COST = 512 * 1024
const ARGON_SECRET_T_COST = 8

const ARGON_PIN_M_COST = 32 * 1024
const ARGON_PIN_T_COST = 4

const ARGON_ANY_P_COST = 1




var CRYPTO_DHE_KEY_CONTEXT = cryptographic_context ("encryption", "dhe_key")
var CRYPTO_NAIVE_KEY_CONTEXT = cryptographic_context ("encryption", "naive_key")
var CRYPTO_AONT_KEY_CONTEXT = cryptographic_context ("encryption", "aont_key")

var CRYPTO_PACKET_SALT_CONTEXT = cryptographic_context ("encryption", "packet_salt")
var CRYPTO_PACKET_KEY_CONTEXT = cryptographic_context ("encryption", "packet_key")
var CRYPTO_ENCRYPTION_KEY_CONTEXT = cryptographic_context ("encryption", "encryption_key")
var CRYPTO_AUTHENTICATION_KEY_CONTEXT = cryptographic_context ("encryption", "authentication_key")

var CRYPTO_SECRET_HASH_CONTEXT = cryptographic_context ("encryption", "secret_hash")
var CRYPTO_SECRET_SALT_CONTEXT = cryptographic_context ("encryption", "secret_salt")
var CRYPTO_SECRET_KEY_CONTEXT = cryptographic_context ("encryption", "secret_key")

var CRYPTO_PIN_HASH_CONTEXT = cryptographic_context ("encryption", "pin_hash")
var CRYPTO_PIN_SALT_CONTEXT = cryptographic_context ("encryption", "pin_salt")
var CRYPTO_PIN_KEY_CONTEXT = cryptographic_context ("encryption", "pin_key")

var CRYPTO_SSH_WRAP_INPUT_CONTEXT = cryptographic_context ("encryption", "ssh_wrap_input")
var CRYPTO_SSH_WRAP_OUTPUT_CONTEXT = cryptographic_context ("encryption", "ssh_wrap_output")

var CRYPTO_DERIVE_SALT_CONTEXT = cryptographic_context ("password", "salt")
var CRYPTO_DERIVE_OUTPUT_CONTEXT = cryptographic_context ("password", "output")








var TEST_RECIPIENT_PRIVATE_BECH32 = "ztxrk1rp0qkrrrht77nh42pkzcf70uy3yrs5uxpq6uvql55h3jsgsxm9fqn6lxmk"
var TEST_SENDER_PUBLIC_BECH32 = "ztxsp1m6f9fwz0ukd7agd3udrlqsu8j0ltc8evfpzw7n040yjatkr4u4nq38he88"

var TEST_SECRET_BECH32 = "ztxcs1qvjhy8ftc7fjajtky3mcrgxdlacer2m6sj8hyxcaa2segdcnhjnqj7ylhm"

var TEST_PIN_STRING = "1234"

var TEST_ENCRYPTED_BOTH = []byte {
		0x89, 0xBB, 0x2C, 0x5C, 0xBB, 0x28, 0xAD, 0x49, 0x22, 0x32, 0xA5, 0x99, 0x48, 0x4C, 0x84, 0x44, 0x56, 0xB3, 0xFD, 0x3A, 0xEF, 0x2A, 0x93, 0xB5, 0x0F, 0x4F, 0x16, 0xDA, 0x8C, 0x8B, 0x25, 0x7A, 0x00, 0x88, 0x6F, 0x18, 0x0B, 0x91,
		0x23, 0x8C, 0x21, 0x8C, 0x0F, 0x1F, 0x76, 0xD0, 0xF4, 0xEB, 0xD2, 0xB0, 0x14, 0xCD, 0xDF, 0x43, 0x24, 0x55, 0x5C, 0x98, 0x92, 0x73, 0xA0, 0xA0, 0xBD, 0xF5, 0x05, 0x81, 0x3A, 0x8F, 0x02, 0x22, 0x40, 0x49, 0x18, 0xA6, 0xCB, 0x3C,
		0xCB, 0x49, 0x6E, 0xF1, 0xEA, 0x8D, 0x95, 0x12, 0xB8, 0x8F, 0x5C, 0xC0, 0x0E, 0x80, 0xE5, 0x16, 0x9B, 0xFB, 0x72, 0x81, 0xF1, 0x59, 0x48, 0x4E, 0x28, 0xCF, 0xA6, 0x02, 0xD0, 0xBE, 0x2A, 0x53, 0xF4, 0xCA, 0x7C, 0x94, 0x43, 0x78,
		0x27, 0x6E, 0x1E, 0x3E, 0xB6, 0x27, 0xCF, 0xA8, 0x65, 0xF3, 0x08, 0xA1, 0x62, 0x9D, 0x8C, 0xE4, 0x29, 0x6F, 0xDB, 0x19, 0x02, 0x44, 0x92, 0x22, 0x19, 0xF1, 0xCA, 0x49, 0x62, 0x5C, 0x63, 0xAE, 0xBE, 0x28, 0x60, 0xDF, 0xAB, 0xC3,
		0xCE, 0xAC, 0x9E, 0x58, 0x81, 0xB4, 0xF1, 0x1E, 0xB9, 0xF1, 0x9E, 0xF6, 0xDE, 0xDC, 0xC6, 0x3A, 0x9A, 0xFA, 0x72, 0xD7, 0x4B, 0x03, 0x61, 0x55, 0x18, 0x0D, 0x69, 0x4B, 0x1A, 0xA7, 0xB0, 0xEB, 0x80, 0x57, 0x66, 0x3E, 0x8D, 0xC3,
		0x7D, 0x18, 0x50, 0x6D, 0x87, 0x66, 0xC8, 0xD3, 0xA3, 0x28, 0xD7, 0x97, 0x93, 0xCD, 0x6B, 0xB9, 0x78, 0xDC, 0x37, 0xB8, 0x3D, 0x9E, 0x23, 0x47, 0xE2, 0x4F, 0x31, 0xC4, 0x65, 0x3A, 0x43, 0x1F, 0x81, 0x52, 0x8A, 0x34, 0x79, 0x95,
		0xCE, 0x60, 0xF1, 0xCA, 0x9B, 0xBB, 0x6A, 0xC5, 0x1B, 0x3F, 0xD9, 0x93, 0x6D, 0x31, 0x40, 0xC6, 0x9A, 0x53, 0x9B, 0xD1, 0xFC, 0xB4, 0x63, 0xB3, 0x9E, 0x8A, 0x14, 0x4E, 0xA9, 0x18, 0xBB, 0x94, 0x40, 0xA2, 0x67, 0xDE, 0x2A, 0xDA,
		0x0A, 0x3E, 0x4A, 0x47, 0x27, 0xD4, 0x4D, 0x85, 0x1E, 0x53, 0xFD, 0x1A, 0xD9, 0x00, 0x76, 0x16, 0x98, 0x8A, 0x25, 0xC9, 0x8F, 0x93, 0x8F, 0x02, 0xB2, 0x92, 0x22, 0x31, 0x1A, 0xC9, 0x1A, 0x91, 0x5D, 0xC7, 0xF8, 0x5E, 0xDF, 0x25,
		0x0D, 0x25, 0x85, 0xF9, 0xDE, 0xD1, 0x2B, 0x29, 0xD3, 0xF9, 0x54, 0x5C, 0xE2, 0xD5, 0xD5, 0xF6,
	}

var TEST_ENCRYPTED_BOTH_WITH_PIN = []byte {
		0xCE, 0x56, 0xED, 0xE0, 0x06, 0x8D, 0xA3, 0x5C, 0xC1, 0x9F, 0x3E, 0xA5, 0x31, 0xDF, 0x41, 0xE7, 0xCC, 0x98, 0x26, 0x04, 0x6E, 0x9D, 0x01, 0x3A, 0x7D, 0xEA, 0xB3, 0x3D, 0xEA, 0x7A, 0x87, 0xD1, 0x5B, 0xBA, 0xCB, 0x60, 0x0C, 0x32,
		0x09, 0x0A, 0x46, 0x1C, 0xB7, 0xFF, 0x7E, 0x27, 0x65, 0x77, 0x87, 0x81, 0x49, 0x15, 0xB9, 0x4E, 0x35, 0xFE, 0xA1, 0x94, 0xC9, 0xD3, 0xAE, 0x45, 0x10, 0xD0, 0x96, 0x04, 0x52, 0x21, 0x72, 0xF8, 0x24, 0x9F, 0x2C, 0x21, 0x4A, 0x3B,
		0x74, 0x12, 0x84, 0x6D, 0x3E, 0x04, 0x29, 0xA1, 0x22, 0x53, 0x4B, 0x22, 0xC2, 0x97, 0x02, 0xA4, 0x7D, 0x62, 0x0E, 0x31, 0x50, 0xCF, 0xC2, 0xED, 0xFE, 0x4E, 0x16, 0x97, 0xA6, 0xA6, 0x00, 0xB9, 0x44, 0xAB, 0x80, 0x59, 0x6C, 0xE1,
		0x04, 0x50, 0xC5, 0x26, 0x66, 0xAF, 0xB9, 0x1D, 0x22, 0xC0, 0x6E, 0x40, 0xA8, 0x7F, 0x88, 0xE1, 0xCD, 0xC4, 0x72, 0x16, 0x07, 0xBC, 0xD5, 0x0D, 0x24, 0x3B, 0x65, 0xBD, 0x3A, 0xA6, 0xAF, 0x03, 0x27, 0xBA, 0x68, 0xDE, 0x28, 0xB5,
		0xB9, 0xB6, 0x81, 0x27, 0x7C, 0x05, 0x1F, 0xFE, 0x5A, 0x70, 0x4D, 0xA0, 0xE7, 0x05, 0x50, 0xA3, 0xCB, 0x84, 0xAF, 0x62, 0x44, 0xFD, 0xCE, 0xF6, 0x46, 0x4F, 0x8B, 0xAE, 0x00, 0x1F, 0x6B, 0x3F, 0x93, 0x4C, 0x90, 0xB6, 0x0F, 0xE2,
		0x31, 0xF7, 0x26, 0x59, 0x61, 0x08, 0xB6, 0x25, 0x75, 0x2A, 0xF5, 0x7C, 0x22, 0x65, 0x04, 0x24, 0x16, 0x77, 0xF2, 0xA4, 0x18, 0x21, 0x99, 0xC1, 0xE4, 0xBD, 0xEB, 0xD3, 0xE0, 0x58, 0xBD, 0x1C, 0x87, 0xB5, 0x67, 0x6F, 0xCA, 0xF3,
		0xFA, 0xD5, 0xD4, 0xD7, 0xED, 0xF1, 0x49, 0xAB, 0x5C, 0xA9, 0x75, 0x85, 0xDD, 0x85, 0xBF, 0xA0, 0xA9, 0x97, 0xFD, 0x11, 0x55, 0xDD, 0xA7, 0x02, 0x12, 0xB3, 0x88, 0x57, 0x21, 0x70, 0xC2, 0x9E, 0x04, 0x66, 0x62, 0x62, 0x86, 0x8E,
		0xAB, 0x06, 0xDF, 0x47, 0x89, 0x52, 0xE0, 0xF3, 0x89, 0xFC, 0xCD, 0xFD, 0xDF, 0x44, 0xBE, 0x7B, 0xFC, 0x20, 0x5F, 0x88, 0x16, 0x50, 0x07, 0x7B, 0xDA, 0x26, 0x49, 0xD8, 0xB4, 0x93, 0xD4, 0xD3, 0x8B, 0x06, 0x28, 0x01, 0x84, 0x8E,
		0x3A, 0x08, 0x8A, 0x60, 0xA1, 0x15, 0x6B, 0x8B, 0xA5, 0x52, 0xC2, 0x9C, 0x3A, 0x58, 0xA3, 0xC3,
	}

var TEST_ENCRYPTED_BOTH_WITH_PIN_AND_SECRET = []byte {
		0x4D, 0xD4, 0x6B, 0x90, 0xCE, 0x93, 0x19, 0xA0, 0x08, 0xB6, 0xB5, 0xFB, 0x47, 0xD9, 0x6A, 0x79, 0x80, 0xFC, 0x32, 0xC2, 0x11, 0x1A, 0xA6, 0xDC, 0xD0, 0xF5, 0x5A, 0x31, 0xF7, 0x5E, 0x7C, 0x26, 0x4D, 0xA6, 0x3D, 0x0E, 0x47, 0xE7,
		0xE9, 0xE6, 0xE1, 0x5D, 0x85, 0x07, 0xBE, 0xD6, 0xDF, 0xDF, 0xCD, 0x5E, 0x52, 0xC9, 0x2F, 0x8E, 0x2B, 0x2C, 0xC4, 0x2F, 0xED, 0x33, 0x24, 0xAD, 0x5D, 0x33, 0x4B, 0xC9, 0x6C, 0x6D, 0x92, 0xAC, 0xDD, 0xB4, 0x40, 0x5A, 0xFC, 0x86,
		0x5B, 0x35, 0xA4, 0x45, 0x23, 0x1F, 0x3A, 0xA3, 0x3C, 0xC9, 0x75, 0xBC, 0x43, 0xAC, 0x12, 0x87, 0x42, 0x5D, 0x95, 0x4F, 0xD0, 0xBE, 0x2A, 0xD7, 0xC1, 0x1B, 0xCD, 0xAB, 0x77, 0x07, 0xDF, 0x66, 0xB6, 0xF6, 0x77, 0x51, 0xEC, 0xDE,
		0xAF, 0xBA, 0xBC, 0x79, 0x6C, 0x76, 0xD4, 0xD9, 0x3A, 0x9B, 0xCF, 0x8D, 0x76, 0x9F, 0xD5, 0x2D, 0x81, 0x32, 0x80, 0xC0, 0xDC, 0xA6, 0xB3, 0x41, 0xD1, 0x60, 0x88, 0x15, 0x4A, 0x14, 0x5D, 0x2F, 0x46, 0xD5, 0x9F, 0xDC, 0x32, 0xEF,
		0xE9, 0x2F, 0xDD, 0x98, 0xA8, 0x41, 0x51, 0x87, 0x74, 0x9D, 0x7A, 0xC7, 0xAF, 0x08, 0x77, 0x8A, 0x77, 0xA4, 0x02, 0x62, 0x32, 0x0A, 0x0A, 0x4F, 0xE5, 0xF7, 0x61, 0xE5, 0x71, 0xF3, 0x72, 0x31, 0xE7, 0x79, 0x6F, 0xCD, 0x26, 0x5D,
		0xE1, 0x27, 0x98, 0x5A, 0x7E, 0x9E, 0x90, 0xC6, 0x0A, 0xD4, 0xAB, 0xD3, 0x9D, 0x10, 0x99, 0x3B, 0x12, 0x6B, 0xC6, 0x6A, 0x4A, 0xC0, 0xE0, 0x48, 0xC1, 0x04, 0x10, 0x5E, 0x6C, 0x20, 0x06, 0x7C, 0x28, 0xDF, 0xAD, 0xB3, 0xD1, 0x2E,
		0x3C, 0xBB, 0xA4, 0xA0, 0xC3, 0xC3, 0x72, 0x00, 0xD4, 0xA9, 0xD1, 0xC5, 0xA1, 0xE8, 0x8F, 0x2F, 0x8E, 0x93, 0xE1, 0x19, 0x03, 0xD2, 0xCE, 0xBB, 0xA2, 0xC5, 0x8C, 0x87, 0x6F, 0x58, 0x55, 0xA7, 0xED, 0x3E, 0xB9, 0x5D, 0xFE, 0x91,
		0x22, 0x2F, 0x19, 0x14, 0x56, 0x64, 0x1E, 0x9E, 0xEC, 0x07, 0x8B, 0x76, 0xCC, 0xC3, 0x7B, 0xD3, 0xF9, 0xA6, 0xC8, 0x4D, 0x01, 0x7A, 0x3A, 0xCF, 0x75, 0x3E, 0x96, 0xFA, 0x8D, 0x91, 0xFD, 0xC6, 0x72, 0xF2, 0xC3, 0x33, 0x82, 0x86,
		0x5C, 0x79, 0xB0, 0xED, 0x18, 0xFF, 0xC9, 0x85, 0x73, 0x20, 0x0F, 0xFA, 0x50, 0x5E, 0x2D, 0xFB,
	}








func main () () {
	
	_recipient := TEST_RECIPIENT_PRIVATE_BECH32
	_sender := TEST_SENDER_PUBLIC_BECH32
	
	decrypt_with_inputs (_recipient, _sender, "", "", TEST_ENCRYPTED_BOTH)
	decrypt_with_inputs (_recipient, _sender, "", TEST_PIN_STRING, TEST_ENCRYPTED_BOTH_WITH_PIN)
	decrypt_with_inputs (_recipient, _sender, TEST_SECRET_BECH32, TEST_PIN_STRING, TEST_ENCRYPTED_BOTH_WITH_PIN_AND_SECRET)
}








func decrypt_with_inputs (
		_recipient_private_bech32 string,
		_sender_public_bech32 string,
		_secret_input_bech32 string,
		_pin_input_string string,
		_encrypted []byte,
) ([]byte) {
	
	_recipient_private_key := bech32_decode_key (BECH32_RECIPIENT_PRIVATE_PREFIX, _recipient_private_bech32)
	_sender_public_key := bech32_decode_key (BECH32_SENDER_PUBLIC_PREFIX, _sender_public_bech32)
	
	_secret_inputs_bech32 := []string {}
	if _secret_input_bech32 != "" {
		_secret_inputs_bech32 = append (_secret_inputs_bech32, _secret_input_bech32)
	}
	
	_pin_inputs_string := []string {}
	if _pin_input_string != "" {
		_pin_inputs_string = append (_pin_inputs_string, _pin_input_string)
	}
	
	var _secret_inputs = make ([][]byte, len (_secret_inputs_bech32))
	for _index, _secret_input_bech32 := range _secret_inputs_bech32 {
		_secret_input := bech32_decode_key (BECH32_SECRET_PREFIX, _secret_input_bech32)
		_secret_inputs[_index] = _secret_input[:]
	}
	
	var _pin_inputs = make ([][]byte, len (_pin_inputs_string))
	for _index, _pin_input_string := range _pin_inputs_string {
		_pin_inputs[_index] = []byte (_pin_input_string)
	}
	
	return decrypt (_recipient_private_key, _sender_public_key, _secret_inputs, _pin_inputs, _encrypted)
}








func decrypt (
			_recipient_private_key [32]byte,
			_sender_public_key [32]byte,
			_secret_inputs [][]byte,
			_pin_inputs [][]byte,
			_encrypted []byte,
) ([]byte) {
	
	
	// --------------------------------------------------------------------------------
	
	
	_buffer := _encrypted
	_buffer_len := len (_buffer)
	
	
	// --------------------------------------------------------------------------------
	
	
	abort_if (0x685eff56, _buffer_len < PACKET_PADDING_SIZE + PACKET_MAC_SIZE + PACKET_SALT_SIZE)
	
	var _packet_salt [32]byte
	_buffer_len -= PACKET_SALT_SIZE
	copy (_packet_salt[:], _buffer[_buffer_len:])
	_buffer = _buffer[:_buffer_len]
	
	
	debug_key ("packet_salt_aont", _packet_salt)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  X25519 shared secret.
	
	
	var _recipient_public_key [32]byte
	curve25519.ScalarBaseMult (&_recipient_public_key, &_recipient_private_key)
	
	var _dhe_shared [32]byte
	curve25519.ScalarMult (&_dhe_shared, &_recipient_private_key, &_sender_public_key)
	// FIXME:  Check if `_dhe_key` is all zeroes!
	
	_dhe_key := blake3_derive_key (CRYPTO_DHE_KEY_CONTEXT, [][32]byte {
			_dhe_shared,
			_sender_public_key,
			_recipient_public_key,
		}, nil)
	
	
	debug_key ("sender_public_key", _sender_public_key)
	debug_key ("recipient_public_key", _recipient_public_key)
	debug_key ("private_key", _recipient_private_key)
	debug_key ("dhe_shared", _dhe_shared)
	debug_key ("dhe_key", _dhe_key)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Initial secrets and pins hashing.
	
	
	_secrets_hash, _secret_hashes := hash_tokens (CRYPTO_SECRET_HASH_CONTEXT, _secret_inputs)
	_pins_hash, _pin_hashes := hash_tokens (CRYPTO_PIN_HASH_CONTEXT, _pin_inputs)
	
	
	debug_key ("secrets_hash", _secrets_hash)
	debug_key ("pins_hash", _pins_hash)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive "naive" key.
	
	
	_naive_key := blake3_derive_key (CRYPTO_NAIVE_KEY_CONTEXT, [][32]byte {
			_secrets_hash,
			_pins_hash,
			_dhe_key,
		}, nil)
	
	
	debug_key ("naive_key", _naive_key)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive AONT key and apply to salt.
	
	
	_aont_key := blake3_derive_key (CRYPTO_AONT_KEY_CONTEXT, [][32]byte {
			_naive_key,
		}, nil)
	
	_aont_hash := blake3_keyed_hash (_aont_key, nil, [][]byte { _buffer })
	
	for i := 0; i < 32; i += 1 {
		_packet_salt[i] ^= _aont_hash[i]
	}
	
	
	debug_key ("aont_key", _aont_key)
	debug_key ("aont_hash", _aont_hash)
	debug_key ("packet_salt", _packet_salt)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive SSH wrapping...
	
	
	var _ssh_wrap_key [32]byte
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive secrets Argon2.
	
	
	_secret_key := _secrets_hash
	for _, _secret_hash := range _secret_hashes {
		
		_secret_salt := blake3_derive_key (CRYPTO_SECRET_SALT_CONTEXT, [][32]byte {
				_secret_key,
				_ssh_wrap_key,
				_packet_salt,
				_naive_key,
			}, nil)
		
		_secret_argon := argon2_derive (_secret_hash, _secret_salt, ARGON_SECRET_M_COST, ARGON_SECRET_T_COST)
		
		_secret_key = blake3_derive_key (CRYPTO_SECRET_KEY_CONTEXT, [][32]byte {
				_secret_salt,
				_secret_argon,
			}, nil)
	}
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive pins Argon2.
	
	
	_pin_key := _pins_hash
	for _, _pin_hash := range _pin_hashes {
		
		_pin_salt := blake3_derive_key (CRYPTO_PIN_SALT_CONTEXT, [][32]byte {
				_pin_key,
				_ssh_wrap_key,
				_packet_salt,
				_naive_key,
			}, nil)
		
		_pin_argon := argon2_derive (_pin_hash, _pin_salt, ARGON_PIN_M_COST, ARGON_PIN_T_COST)
		
		_pin_key = blake3_derive_key (CRYPTO_PIN_KEY_CONTEXT, [][32]byte {
				_pin_salt,
				_pin_argon,
			}, nil)
	}
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive packet keys.
	
	
	_packet_key := blake3_derive_key (CRYPTO_PACKET_KEY_CONTEXT, [][32]byte {
			_ssh_wrap_key,
			_secret_key,
			_pin_key,
			_packet_salt,
			_naive_key,
		}, nil)
	
	_encryption_key := blake3_derive_key (CRYPTO_ENCRYPTION_KEY_CONTEXT, [][32]byte {
			_packet_key,
		}, nil)
	
	_authentication_key := blake3_derive_key (CRYPTO_AUTHENTICATION_KEY_CONTEXT, [][32]byte {
			_packet_key,
		}, nil)
	
	
	debug_key ("ssh_wrap_key", _ssh_wrap_key)
	debug_key ("secret_key", _secret_key)
	debug_key ("pin_key", _pin_key)
	
	debug_key ("packet_key", _packet_key)
	debug_key ("encryption_key", _encryption_key)
	debug_key ("authentication_key", _authentication_key)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Authenticate packet.
	
	
	var _authentication_tag_actual [32]byte
	_buffer_len -= PACKET_MAC_SIZE
	copy (_authentication_tag_actual[:], _buffer[_buffer_len:])
	_buffer = _buffer[:_buffer_len]
	
	_authentication_tag_expected := blake3_keyed_hash (_authentication_key, nil, [][]byte { _buffer })
	
	abort_if_not_equals (0x35cfc683, _authentication_tag_actual, _authentication_tag_expected)
	
	
	debug_key ("authentication_tag_actual", _authentication_tag_actual)
	debug_key ("authentication_tag_expected", _authentication_tag_expected)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Decrypt packet.
	
	
	var _encryption_nonce [12]byte
	
	_chacha20, _error := chacha20.NewUnauthenticatedCipher (_encryption_key[:], _encryption_nonce[:])
	abort_on_error (0x2dc52f2c, _error)
	
	_chacha20.XORKeyStream (_buffer, _buffer)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Unpadding.
	
	
	abort_if (0x8d92763f, _buffer_len < 1)
	
	_padding := int (_buffer[_buffer_len - 1]) + 1
	abort_if (0x46083b69, _buffer_len < _padding)
	_buffer_len -= _padding
	
	for i := 0; i < _padding; i += 1 {
		abort_if_not_equals (0x3813d80b, int (_buffer[_buffer_len + i]), i)
	}
	_buffer = _buffer[:_buffer_len]
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Decompressing.
	
	
	abort_if (0x9c880e37, _buffer_len < 4)
	
	_buffer_len -= 4
	_decrypted_len := int (binary.BigEndian.Uint32 (_buffer[_buffer_len:]))
	_buffer = _buffer[:_buffer_len]
	
	abort_if (0xe57a14f8, _decrypted_len < _buffer_len)
	
	var _decrypted []byte
	if _decrypted_len == _buffer_len {
		_decrypted = _buffer
	} else {
		_decompressor := brotli.NewReader (bytes.NewBuffer (_buffer))
		_decompressed, _error := io.ReadAll (_decompressor)
		abort_on_error (0x60664b36, _error)
		_decrypted = _decompressed
	}
	
	abort_if_not_equals (0x5e7e9e54, len (_decrypted), _decrypted_len)
	
	
	debug_slice ("decrypted", _decrypted)
	
	
	// --------------------------------------------------------------------------------
	
	
	return _decrypted
}








func hash_tokens (_context string, _tokens [][]byte) ([32]byte, [][32]byte) {
	
	_hashes := make ([][32]byte, len (_tokens))
	for _index, _token := range _tokens {
		_hashes[_index] = blake3_derive_key (_context, nil, [][]byte { []byte (_token) })
	}
	
	// FIXME:  Sort and deduplicate!
	
	_hash := blake3_derive_key (_context, _hashes, nil)
	
	return _hash, _hashes
}








func argon2_derive (_hash [32]byte, _salt [32]byte, _m_cost uint, _t_cost uint) ([32]byte) {
	
	var _output [32]byte
	
	_output_0 := argon2.IDKey (_hash[:], _salt[:], uint32 (_t_cost), uint32 (_m_cost), ARGON_ANY_P_COST, 32)
	
	copy (_output[:], _output_0)
	return _output
}








func blake3_derive_key (_context string, _fixed_elements [][32]byte, _variable_elements [][]byte) ([32]byte) {
	_hasher := blake3.NewDeriveKey (_context)
	return blake3_continue (_hasher, _fixed_elements, _variable_elements)
}


func blake3_keyed_hash (_key [32]byte, _fixed_elements [][32]byte, _variable_elements [][]byte) ([32]byte) {
	_hasher, _error := blake3.NewKeyed (_key[:])
	abort_on_error (0x16f3b835, _error)
	return blake3_continue (_hasher, _fixed_elements, _variable_elements)
}


func blake3_continue (_hasher *blake3.Hasher, _fixed_elements [][32]byte, _variable_elements [][]byte) ([32]byte) {
	
	for _, _fixed_element := range _fixed_elements {
		_hasher.Write (_fixed_element[:])
	}
	
	for _, _variable_element := range _variable_elements {
		binary.Write (_hasher, binary.BigEndian, uint32 (len (_variable_element)))
		_hasher.Write (_variable_element)
	}
	
	var _hash [32]byte
	_hasher.Digest () .Read (_hash[:])
	
	return _hash
}








func bech32_decode_key (_prefix string, _encoded string) ([32]byte) {
	
	_prefix_actual, _u5_slice, _error := bech32.Decode (_encoded)
	abort_on_error (0x01dfd25c, _error)
	
	abort_if_not_equals (0x22a377c0, _prefix_actual, _prefix)
	
	_u8_slice, _error := bech32.ConvertBits (_u5_slice, 5, 8, false)
	abort_on_error (0x43c3f46a, _error)
	
	abort_if_not_equals (0xe071efb5, len (_u8_slice), 32)
	
	var _decoded [32]byte
	copy (_decoded[:], _u8_slice)
	
	return _decoded
}








func cryptographic_context (_namespace string, _purpose string) (string) {
	return "z-tokens / exchange / " + _namespace + " / " + _purpose + " / " + "(2023a)"
}








func debug_key (_identifier string, _key [32]byte) () {
	fmt.Fprintf (os.Stderr, "[>>] [88709639]  >>  %-40s  >>  %x\n", _identifier, _key)
}


func debug_slice (_identifier string, _slice []byte) () {
	fmt.Fprintf (os.Stderr, "[>>] [d7291bc3]  vv  %-40s  vv  (%04d)\n", _identifier, len (_slice))
	for len (_slice) > 0 {
		var _display []byte
		if len (_slice) > 32 {
			_display = _slice[:32]
			_slice = _slice[32:]
		} else {
			_display = _slice
			_slice = _slice[:0]
		}
		fmt.Fprintf (os.Stderr, "[>>] [574d4f8a]  --  %-40s  >>  %x\n", "", _display)
	}
		fmt.Fprintf (os.Stderr, "[>>] [8a96325e]  ^^  %-40s  ^^\n", _identifier)
}








func abort_on_error (_code uint32, _error error) () {
	if _error != nil {
		abort (_code, _error)
	}
}


func abort_if_not_equals [V comparable] (_code uint32, _actual V, _expected V) () {
	if _actual != _expected {
		abort (_code, fmt.Errorf ("expected: `%v`  //  actual: `%v`", _expected, _actual))
	}
}


func abort_if (_code uint32, _condition bool) () {
	if _condition {
		abort (_code, nil)
	}
}


func abort (_code uint32, _error error) (error) {
	if _error != nil {
		fmt.Fprintf (os.Stderr, "[!!] [%08x]  %s\n", _code, _error)
	} else {
		fmt.Fprintf (os.Stderr, "[!!] [%08x]\n", _code)
	}
	os.Exit (10)
	panic (0xf5a21b11)
}


