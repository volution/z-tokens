

package exchange


import (
		"fmt"
		"os"
	)




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


