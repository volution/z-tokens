
package exchange


import (
		"github.com/btcsuite/btcd/btcutil/bech32"
	)




func bech32_decode_key (_prefix string, _encoded string) (Key) {
	
	_data := bech32_decode_bytes (_prefix, _encoded)
	abort_if_not_equals (0xe071efb5, len (_data), 32)
	
	return key_from_slice (_data)
}


func bech32_decode_bytes (_prefix string, _encoded string) ([]byte) {
	
	_prefix_actual, _u5_slice, _error := bech32.DecodeNoLimit (_encoded)
	abort_on_error (0x01dfd25c, _error)
	
	abort_if_not_equals (0x22a377c0, _prefix_actual, _prefix)
	
	_u8_slice, _error := bech32.ConvertBits (_u5_slice, 5, 8, false)
	abort_on_error (0x43c3f46a, _error)
	
	return _u8_slice
}


