

package exchange




type Key *[32]byte




func key_zero () (Key) {
	
	return new ([32]byte)
}


func key_from_slice (_slice []byte) (Key) {
	
	abort_if_not_equals (0xb6b788ae, len (_slice), 32)
	
	_key := key_zero ()
	copy (_key[:], _slice)
	
	return _key
}


