

package exchange


import (
		"bytes"
		
		"golang.org/x/crypto/ssh"
	)




type sshWrapper struct {
	public_key_algorithm string
	signature_algorithm string
	public_key ssh.PublicKey
	signer ssh.Signer
}

type SshWrapper *sshWrapper




func ssh_wrap (_ssh_wrapper SshWrapper, _input_data []byte) (Key) {
	
	_public_key_algorithm := _ssh_wrapper.public_key_algorithm
	_signature_algorithm := _ssh_wrapper.signature_algorithm
	_public_key := _ssh_wrapper.public_key
	_public_key_serialized := _public_key.Marshal ()
	_signer := _ssh_wrapper.signer
	_ssh_wrapper = nil
	
	_key_hash := blake3_derive_key (SSH_WRAP_KEY_HASH_CONTEXT, nil, [][]byte {
		//	[]byte (_signature_algorithm),
			[]byte (_public_key_algorithm),
			[]byte (_public_key_serialized),
		})
	
	debug_key ("ssh_wrap_key_hash", _key_hash)
	
	_input_hash := blake3_derive_key (SSH_WRAP_INPUT_HASH_CONTEXT,
		[]Key {
			_key_hash,
		},
		[][]byte {
			_input_data,
		})
	
	debug_key ("ssh_wrap_input_hash", _input_hash)
	
	// NOTE:  Here we should actually contact the `ssh-agent` and ask for the signature
	// FIXME:  We can't ask for a particular signature, thus we hope the default chosen one matches our desired one...
	
	_signature, _error := _signer.Sign (nil, _input_hash[:])
	abort_on_error (0xd80c431b, _error)
	
	abort_if_not_equals (0x5dda6367, _signature.Format, _signature_algorithm)
	_signature_data := _signature.Blob
	
	debug_slice ("ssh_wrap_signature_data", _signature_data)
	
	_error = _public_key.Verify (_input_hash[:], _signature)
	abort_on_error (0x6e341779, _error)
	
	_output_hash := blake3_derive_key (SSH_WRAP_OUTPUT_HASH_CONTEXT,
		[]Key {
			_key_hash,
			_input_hash,
		},
		[][]byte {
			_signature_data,
		})
	
	debug_key ("ssh_wrap_output_hash", _output_hash)
	
	return _output_hash
}








func ssh_wrapper_decode (_bech32 string) (SshWrapper) {
	
	_buffer := bech32_decode_bytes (BECH32_SSH_WRAP_HANDLE_PREFIX, _bech32)
	_buffer_len := len (_buffer)
	
	debug_slice ("ssh_wrapper_handle_bytes", _buffer)
	
	abort_if (0xd0b77034, _buffer_len < 1)
	
	_buffer_len -= 1
	_key_type := _buffer[_buffer_len]
	_buffer = _buffer[:_buffer_len]
	
	var _public_key_algorithm string
	var _signature_algorithm string
	
	switch _key_type {
		
		case 1 :
			_public_key_algorithm = ssh.KeyAlgoED25519
			_signature_algorithm = ssh.KeyAlgoED25519
		
		case 2 :
			_public_key_algorithm = ssh.KeyAlgoRSA
			_signature_algorithm = ssh.KeyAlgoRSA
		
		case 3 :
			// FIXME:  Currently not supported!
			_public_key_algorithm = ssh.KeyAlgoRSA
			_signature_algorithm = ssh.KeyAlgoRSASHA256
		
		case 4 :
			// FIXME:  Currently not supported!
			_public_key_algorithm = ssh.KeyAlgoRSA
			_signature_algorithm = ssh.KeyAlgoRSASHA512
		
		default :
			abort (0xf6867c6f, nil)
	}
	
	var _public_key ssh.PublicKey
	var _signer ssh.Signer
	
	var _error error
	
	_public_key, _error = ssh.ParsePublicKey (_buffer)
	abort_on_error (0x0f58c245, _error)
	
	abort_if_not_equals (0xd0a78102, _public_key.Type (), _public_key_algorithm)
	
	{
		_private_key, _error := ssh.ParsePrivateKey (test_private_ssh_key)
		abort_on_error (0x01b6af09, _error)
		
		_private_public_key := _private_key.PublicKey ()
		
		abort_if (0x4816c504, ! bytes.Equal (_private_public_key.Marshal (), _public_key.Marshal ()))
		
		_signer = _private_key
	}
	
	_ssh_wrapper := & sshWrapper {
			public_key_algorithm : _public_key_algorithm,
			signature_algorithm : _signature_algorithm,
			public_key : _public_key,
			signer : _signer,
		}
	
	return _ssh_wrapper
}




var test_private_ssh_key = []byte (`
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACC5LNdWIb7GoVWjecSPqzgpDL04TnyQgaTVVYy+pmcEHAAAALCIOkpDiDpK
QwAAAAtzc2gtZWQyNTUxOQAAACC5LNdWIb7GoVWjecSPqzgpDL04TnyQgaTVVYy+pmcEHA
AAAEBt/+OBtWrN7kZOK/AidxjmK+6hgt80u9RMII7Qf6ADY7ks11YhvsahVaN5xI+rOCkM
vThOfJCBpNVVjL6mZwQcAAAAKnotdG9rZW5zIGV4Y2hhbmdlIHNzaCB3cmFwIGtleSBmb3
IgdGVzdGluZwECAw==
-----END OPENSSH PRIVATE KEY-----
`)


