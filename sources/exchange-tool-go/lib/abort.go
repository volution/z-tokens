

package exchange


import (
		"fmt"
		"os"
	)




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


