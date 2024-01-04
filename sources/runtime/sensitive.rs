

use crate::preludes::std_plus_extras::*;




include! ("./sensitive_macros.in");




pub trait Sensitive {
	fn erase (&mut self);
}


pub use ::zeroize::Zeroize;


pub struct SensitiveZeroize <Value : ?Sized + Zeroize> (pub Value);
pub struct SensitiveIgnored <Value : ?Sized> (pub Value);








impl <Value : ?Sized + Zeroize> Sensitive for SensitiveZeroize<Value> {
	fn erase (&mut self) -> () {
		Zeroize::zeroize (&mut self.0);
	}
}


impl <Value : ?Sized> Sensitive for SensitiveIgnored<Value> {
	fn erase (&mut self) -> () {
		// NOP
	}
}


impl <Value : Sized + Zeroize> From<Value> for SensitiveZeroize<Value> {
	fn from (_value : Value) -> Self {
		SensitiveZeroize (_value)
	}
}


impl <Value : Sized> From<Value> for SensitiveIgnored<Value> {
	fn from (_value : Value) -> Self {
		SensitiveIgnored (_value)
	}
}








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

impl_sensitive! ( <{'a, Value}> &'a Value => |self| {
	*self = unsafe { mem::transmute ( [0xffu8; mem::size_of::<&'static u8> ()] ) };
});

impl_sensitive! ( <{'a, Value}> &'a [Value] => |self| {
	*self = unsafe { mem::transmute ( [0xffu8; mem::size_of::<&'static [u8]> ()] ) };
});

impl_sensitive! ( <{'a}> Cow<'a, str> => |self| {
	match self {
		Cow::Borrowed (_str) =>
			_str.erase (),
		Cow::Owned (_string) =>
			_string.erase (),
	}
});








impl_sensitive_zeroize! (for { u8, u16, u32, u64, u128, usize, });
impl_sensitive_zeroize! (for { i8, i16, i32, i64, i128, isize, });

impl_sensitive_zeroize! (char);
impl_sensitive_zeroize! (String);

// impl_sensitive_zeroize! (impl <V> Sensitive for V where V : Zeroize);
impl_sensitive_zeroize! (impl <V> Sensitive for [V] where [V] : Zeroize);
impl_sensitive_zeroize! (impl <const N : usize, V> Sensitive for [V; N] where [V; N] : Zeroize);




pub fn zeroize_and_drop <Value : Sized + Zeroize> (mut _value : Value) -> () {
	Zeroize::zeroize (&mut _value);
	mem::drop (_value);
}








#[ macro_export ]
macro_rules! drop {
	
	( $_identifier : ident ) => {
		let $_identifier = mem::drop ($_identifier);
	};
	
	( $( $_identifier : ident ),+ ) => {
		$( drop! ($_identifier); )+
	}
}


pub use drop;


