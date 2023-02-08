

package exchange




import (
		"bytes"
		"encoding/binary"
		"io"
	)


import (
		
		"golang.org/x/crypto/curve25519"
		"golang.org/x/crypto/chacha20"
		"golang.org/x/crypto/argon2"
		
		"github.com/zeebo/blake3"
		"github.com/andybalholm/brotli"
	)








func Decrypt (
		_recipient_private_bech32 string,
		_sender_public_bech32 string,
		_secret_input_bech32 string,
		_pin_input_string string,
		_encrypted []byte,
		_ssh_wrap_handle_bech32 string,
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
	
	var _ssh_wrappers = make ([]SshWrapper, 0)
	if _ssh_wrap_handle_bech32 != "" {
		_ssh_wrapper := ssh_wrapper_decode (_ssh_wrap_handle_bech32)
		_ssh_wrappers = append (_ssh_wrappers, _ssh_wrapper)
	}
	
	return decrypt (
			_recipient_private_key,
			_sender_public_key,
			_secret_inputs,
			_pin_inputs,
			_encrypted,
			_ssh_wrappers,
		)
}








func decrypt (
			_recipient_private_key Key,
			_sender_public_key Key,
			_secret_inputs [][]byte,
			_pin_inputs [][]byte,
			_encrypted []byte,
			_ssh_wrappers []SshWrapper,
) ([]byte) {
	
	
	debug_slice ("encrypted", _encrypted)
	
	
	// --------------------------------------------------------------------------------
	
	
	_buffer := _encrypted
	_buffer_len := len (_buffer)
	
	
	abort_if (0x685eff56, _buffer_len < PACKET_PADDING_SIZE + PACKET_MAC_SIZE + PACKET_SALT_SIZE)
	abort_if_not_equals (0x5ac8df83, (_buffer_len - PACKET_MAC_SIZE - PACKET_SALT_SIZE) % PACKET_PADDING_SIZE, 0)
	
	
	// --------------------------------------------------------------------------------
	
	
	_buffer_len -= PACKET_SALT_SIZE
	_packet_salt := key_from_slice (_buffer[_buffer_len:])
	_buffer = _buffer[:_buffer_len]
	
	
	debug_key ("packet_salt_aont", _packet_salt)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  X25519 shared secret.
	
	
	_recipient_public_key := key_zero ()
	curve25519.ScalarBaseMult (_recipient_public_key, _recipient_private_key)
	
	_dhe_shared := key_zero ()
	curve25519.ScalarMult (_dhe_shared, _recipient_private_key, _sender_public_key)
	
	// FIXME:  Check if `_sender_public_key` was not contributory!
	
	_dhe_key := blake3_derive_key (CRYPTO_DHE_KEY_CONTEXT, []Key {
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
	
	
	_naive_key := blake3_derive_key (CRYPTO_NAIVE_KEY_CONTEXT, []Key {
			_secrets_hash,
			_pins_hash,
			_dhe_key,
		}, nil)
	
	
	debug_key ("naive_key", _naive_key)
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive AONT key and apply to salt.
	
	
	_aont_key := blake3_derive_key (CRYPTO_AONT_KEY_CONTEXT, []Key {
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
	
	
	_ssh_wrap_key := key_zero ()
	for _, _ssh_wrapper := range _ssh_wrappers {
		
		_ssh_wrap_input := blake3_derive_key (CRYPTO_SSH_WRAP_INPUT_CONTEXT, []Key {
				_ssh_wrap_key,
				_packet_salt,
				_naive_key,
			}, nil)
		
		_ssh_wrap_output := ssh_wrap (_ssh_wrapper, _ssh_wrap_input[:])
		
		_ssh_wrap_key = blake3_derive_key (CRYPTO_SSH_WRAP_OUTPUT_CONTEXT, []Key {
				_ssh_wrap_input,
				_ssh_wrap_output,
			}, nil)
	}
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive secrets Argon2.
	
	
	_secret_key := _secrets_hash
	for _, _secret_hash := range _secret_hashes {
		
		_secret_salt := blake3_derive_key (CRYPTO_SECRET_SALT_CONTEXT, []Key {
				_secret_key,
				_ssh_wrap_key,
				_packet_salt,
				_naive_key,
			}, nil)
		
		_secret_argon := argon2_derive (_secret_hash, _secret_salt, ARGON_SECRET_M_COST, ARGON_SECRET_T_COST)
		
		_secret_key = blake3_derive_key (CRYPTO_SECRET_KEY_CONTEXT, []Key {
				_secret_salt,
				_secret_argon,
			}, nil)
	}
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive pins Argon2.
	
	
	_pin_key := _pins_hash
	for _, _pin_hash := range _pin_hashes {
		
		_pin_salt := blake3_derive_key (CRYPTO_PIN_SALT_CONTEXT, []Key {
				_pin_key,
				_ssh_wrap_key,
				_packet_salt,
				_naive_key,
			}, nil)
		
		_pin_argon := argon2_derive (_pin_hash, _pin_salt, ARGON_PIN_M_COST, ARGON_PIN_T_COST)
		
		_pin_key = blake3_derive_key (CRYPTO_PIN_KEY_CONTEXT, []Key {
				_pin_salt,
				_pin_argon,
			}, nil)
	}
	
	
	// --------------------------------------------------------------------------------
	// NOTE:  Derive packet keys.
	
	
	_packet_key := blake3_derive_key (CRYPTO_PACKET_KEY_CONTEXT, []Key {
			_ssh_wrap_key,
			_secret_key,
			_pin_key,
			_packet_salt,
			_naive_key,
		}, nil)
	
	_encryption_key := blake3_derive_key (CRYPTO_ENCRYPTION_KEY_CONTEXT, []Key {
			_packet_key,
		}, nil)
	
	_authentication_key := blake3_derive_key (CRYPTO_AUTHENTICATION_KEY_CONTEXT, []Key {
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
	
	
	_buffer_len -= PACKET_MAC_SIZE
	_authentication_tag_actual := key_from_slice (_buffer[_buffer_len:])
	_buffer = _buffer[:_buffer_len]
	
	_authentication_tag_expected := blake3_keyed_hash (_authentication_key, nil, [][]byte { _buffer })
	
	abort_if_not_equals (0x35cfc683, *_authentication_tag_actual, *_authentication_tag_expected)
	
	
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
	
	
	// --------------------------------------------------------------------------------
	
	
	debug_slice ("decrypted", _decrypted)
	
	
	return _decrypted
}








func blake3_derive_key (_context string, _fixed_elements []Key, _variable_elements [][]byte) (Key) {
	
	_hasher := blake3.NewDeriveKey (_context)
	
	return blake3_continue (_hasher, _fixed_elements, _variable_elements)
}


func blake3_keyed_hash (_key Key, _fixed_elements []Key, _variable_elements [][]byte) (Key) {
	
	_hasher, _error := blake3.NewKeyed (_key[:])
	abort_on_error (0x16f3b835, _error)
	
	return blake3_continue (_hasher, _fixed_elements, _variable_elements)
}


func blake3_continue (_hasher *blake3.Hasher, _fixed_elements []Key, _variable_elements [][]byte) (Key) {
	
	for _, _fixed_element := range _fixed_elements {
		_hasher.Write (_fixed_element[:])
	}
	
	for _, _variable_element := range _variable_elements {
		binary.Write (_hasher, binary.BigEndian, uint32 (len (_variable_element)))
		_hasher.Write (_variable_element)
	}
	
	_hash := key_zero ()
	_hasher.Digest () .Read (_hash[:])
	
	return _hash
}








func argon2_derive (_hash Key, _salt Key, _m_cost uint, _t_cost uint) (Key) {
	
	_argon := argon2.IDKey (_hash[:], _salt[:], uint32 (_t_cost), uint32 (_m_cost), ARGON_ANY_P_COST, 32)
	
	return key_from_slice (_argon)
}








func hash_tokens (_context string, _tokens [][]byte) (Key, []Key) {
	
	_hashes := make ([]Key, len (_tokens))
	for _index, _token := range _tokens {
		_hashes[_index] = blake3_derive_key (_context, nil, [][]byte { []byte (_token) })
	}
	
	// FIXME:  Sort and deduplicate!
	
	_hash := blake3_derive_key (_context, _hashes, nil)
	
	return _hash, _hashes
}


