

use crate::prelude::*;




pub trait Sensitive {
	fn erase (&mut self);
}








impl <Value : Sized + Sensitive + 'static> Sensitive for Rb<Value> {
	fn erase (&mut self) -> () {
		self.0.erase ();
	}
}

impl <Value : Sized + Sensitive + 'static> Sensitive for RbList<Value> {
	fn erase (&mut self) -> () {
		self.0.erase ();
	}
}




impl <Value : Sized + Sensitive + 'static> Sensitive for RbRef<Value> {
	
	fn erase (&mut self) -> () {
		match self {
			Self::Static (_) =>
				unsafe {
					let _junk_ref = [0u8; mem::size_of::<&'static u8> ()];
					let mut _junk_self = Self::Static (mem::transmute (_junk_ref));
					mem::swap (self, &mut _junk_self);
					let _junk_mem : &mut [u8] = ::std::slice::from_raw_parts_mut (&mut _junk_self as *mut Self as *mut u8, mem::size_of::<Self> ());
					::zeroize::Zeroize::zeroize (_junk_mem);
					mem::forget (_junk_self);
				}
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}


impl <Value : Sized + Sensitive + 'static> Sensitive for RbListRef<Value> {
	
	fn erase (&mut self) -> () {
		match self {
			Self::Static (_) => {
				unsafe {
					let _junk_ref = [0u8; mem::size_of::<&'static [u8]> ()];
					let mut _junk_self = Self::Static (mem::transmute (_junk_ref));
					mem::swap (self, &mut _junk_self);
					let _junk_mem : &mut [u8] = ::std::slice::from_raw_parts_mut (&mut _junk_self as *mut Self as *mut u8, mem::size_of::<Self> ());
					::zeroize::Zeroize::zeroize (_junk_mem);
					mem::forget (_junk_self);
				}
			}
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}








macro_rules! impl_sensitive_drop {
	( <{ $( $_template_a : tt )* }> $_type : ty where <{ $( $_where : tt )+ }> ) => {
		impl < $( $_template_a )* > Drop for $_type where $( $_where )+ {
			fn drop (&mut self) -> () {
				self.erase ();
			}
		}
	};
	( <{ $( $_template_a : tt )* }> $_type : ty ) => {
		impl < $( $_template_a )* > Drop for $_type {
			fn drop (&mut self) -> () {
				self.erase ();
			}
		}
	};
	( $_type : ty ) => {
		impl Drop for $_type {
			fn drop (&mut self) -> () {
				self.erase ();
			}
		}
	};
}




impl_sensitive_drop! (<{Value}> Rb <Value> where <{ Value : Sized + Sensitive + 'static }>);
impl_sensitive_drop! (<{Value}> RbRef<Value> where <{ Value : Sized + Sensitive + 'static }>);

impl_sensitive_drop! (<{Value}> RbList<Value> where <{ Value : Sized + Sensitive + 'static }>);
impl_sensitive_drop! (<{Value}> RbListRef<Value> where <{ Value : Sized + Sensitive + 'static }>);








macro_rules! impl_sensitive_nop {
	( $_type : ident ) => {
		impl Sensitive for $_type {
			fn erase (&mut self) -> () {
				// NOP
			}
		}
	};
}



// NOTE:  Has only public information.
impl_sensitive_nop! (TokenPattern);
impl_sensitive_nop! (TokenPatternTags);
impl_sensitive_nop! (SeparatorPattern);
impl_sensitive_nop! (AtomPattern);
impl_sensitive_nop! (GlyphPattern);








macro_rules! impl_sensitive {
	( <{ $( $_template_a : tt )* }> $_type : ty where <{ $( $_where : tt )+ }> => |$_self : ident| $_block : block ) => {
		impl < $( $_template_a )* > Sensitive for $_type where $( $_where )+ {
			fn erase (&mut $_self) -> () {
				$_block
			}
		}
	};
	( <{ $( $_template_a : tt )* }> $_type : ty => |$_self : ident| $_block : block ) => {
		impl < $( $_template_a )* > Sensitive for $_type {
			fn erase (&mut $_self) -> () {
				$_block
			}
		}
	};
	( $_type : ty => |$_self : ident| $_block : block ) => {
		impl Sensitive for $_type {
			fn erase (&mut $_self) -> () {
				$_block
			}
		}
	};
}




impl_sensitive! (Token => |self| {
	self.atoms.erase ();
});

impl_sensitive! (Atom => |self| {
	match self {
		Atom::Separator (_separator) =>
			_separator.erase (),
		Atom::Constant (_constant) =>
			_constant.erase (),
		Atom::Glyph (_glyph) =>
			_glyph.erase (),
	}
});

impl_sensitive! (Glyph => |self| {
	match self {
		Glyph::Text (_text) =>
			_text.erase (),
		Glyph::Integer (_integer, _) =>
			_integer.erase (),
		Glyph::Bytes (_bytes, _) =>
			_bytes.erase (),
		Glyph::Timestamp (_timestamp, _) =>
			_timestamp.erase (),
	}
});

impl_sensitive! (Separator => |self| {
	match self {
		Separator::Mandatory (_text) =>
			_text.erase (),
		Separator::Optional (_text) =>
			_text.erase (),
	}
});


impl_sensitive! (Text => |self| {
	match self {
		Text::Char (_char) =>
			_char.erase (),
		Text::Str (_str) =>
			_str.erase (),
		Text::String (_string) =>
			_string.erase (),
	}
});

impl_sensitive! (Bytes => |self| {
	match self {
		Bytes::Static (_slice) =>
			_slice.erase (),
		Bytes::Boxed (_box) =>
			_box.as_mut () .erase (),
	}
});




impl_sensitive! ( <{ Value }> Arc<Value> where <{ Value : Sensitive }> => |self| {
	if let Some (_value) = Arc::get_mut (self) {
		_value.erase ()
	}
});

impl_sensitive! ( <{ Value }> Arc<[Value]> where <{ Value : Sensitive }> => |self| {
	if let Some (_values) = Arc::get_mut (self) {
		for _value in _values {
			_value.erase ()
		}
	}
});

impl_sensitive! ( <{ A, B }> (A, B) where <{ A : Sensitive, B : Sensitive }> => |self| {
	self.0.erase ();
	self.1.erase ();
});

impl_sensitive! ( <{'a}> &'a str => |self| {
	*self = "";
});

impl_sensitive! ( <{'a, Value}> &'a [Value] => |self| {
	*self = &[];
});

impl_sensitive! ( <{'a}> Cow<'a, str> => |self| {
	match self {
		Cow::Borrowed (_str) =>
			_str.erase (),
		Cow::Owned (_string) =>
			_string.erase (),
	}
});








macro_rules! impl_sensitive_zeroize {
	( <{ $( $_template_a : tt )* }> $_type : ty where <{ $( $_where : tt )+ }> ) => {
		impl < $( $_template_a )* > Sensitive for $_type where $( $_where )+ {
			fn erase (&mut self) -> () {
				::zeroize::Zeroize::zeroize (self);
			}
		}
	};
	( <{ $( $_template_a : tt )* }> $_type : ty ) => {
		impl < $( $_template_a )* > Sensitive for $_type {
			fn erase (&mut self) -> () {
				::zeroize::Zeroize::zeroize (self);
			}
		}
	};
	( $_type : ty ) => {
		impl Sensitive for $_type {
			fn erase (&mut self) -> () {
				::zeroize::Zeroize::zeroize (self);
			}
		}
	};
}




impl_sensitive_zeroize! (u128);
impl_sensitive_zeroize! (char);
impl_sensitive_zeroize! (String);
impl_sensitive_zeroize! ([u8]);



