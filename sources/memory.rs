

use crate::prelude::*;




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct Rb <Value : Sized + Sensitive + 'static> (RbRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct RbList <Value : Sized + Sensitive + 'static> (RbListRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
enum RbRef <Value : Sized + Sensitive + 'static> {
	Static (&'static Value),
	Rc (Arc<Value>),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
enum RbListRef <Value : Sized + Sensitive + 'static> {
	Static (&'static [Rb<Value>]),
	Rc (Arc<[Rb<Value>]>),
}


pub trait Sensitive {}




impl <Value : Sized + Sensitive + 'static> RbRef<Value> {
	
	pub fn new (_value : Value) -> Self {
		RbRef::Rc (Arc::new (_value))
	}
	
	pub const fn new_static (_value : &'static Value) -> Self {
		RbRef::Static (_value)
	}
	
	pub fn clone (&self) -> Self {
		match self {
			RbRef::Static (_value) =>
				RbRef::Static (_value),
			RbRef::Rc (_value) =>
				RbRef::Rc (Arc::clone (_value)),
		}
	}
	
	pub fn deref (&self) -> &Value {
		match self {
			RbRef::Static (_value) =>
				_value,
			RbRef::Rc (_value) =>
				Arc::deref (_value),
		}
	}
}




impl <Value : Sized + Sensitive + 'static> RbListRef<Value> {
	
	pub fn clone (&self) -> Self {
		match self {
			RbListRef::Static (_value) =>
				RbListRef::Static (_value),
			RbListRef::Rc (_value) =>
				RbListRef::Rc (Arc::clone (_value)),
		}
	}
	
	pub fn deref (&self) -> &[Rb<Value>] {
		match self {
			RbListRef::Static (_value) =>
				_value,
			RbListRef::Rc (_value) =>
				Arc::deref (_value),
		}
	}
}




impl <Value : Sized + Sensitive + 'static> Rb<Value> {
	
	pub fn new (_value : Value) -> Self {
		Self (RbRef::new (_value))
	}
	
	pub const fn new_static (_value : &'static Value) -> Self {
		Self (RbRef::new_static (_value))
	}
	
	pub fn clone (&self) -> Self {
		Self (self.0.clone ())
	}
}


impl <Value : Sized + Sensitive + 'static> Deref for Rb<Value> {
	
	type Target = Value;
	
	fn deref (&self) -> &Value {
		self.0.deref ()
	}
}


impl <Value : Sized + Sensitive + 'static> AsRef<Value> for Rb<Value> {
	
	fn as_ref (&self) -> &Value {
		self.deref ()
	}
}




impl <Value : Sized + Sensitive + 'static> RbList <Value> {
	
	pub const fn from_static (_values : &'static [Rb<Value>]) -> Self {
		Self (RbListRef::Static (_values))
	}
	
	pub fn from_vec (_values : Vec<Value>) -> Self {
		let _values = _values.into_iter () .map (Rb::new) .collect ();
		Self::from_vec_rb (_values)
	}
	
	pub fn from_vec_rb (_values : Vec<Rb<Value>>) -> Self {
		let _values = Arc::from (_values);
		Self (RbListRef::Rc (_values))
	}
	
	pub fn clone (&self) -> Self {
		Self (self.0.clone ())
	}
}


impl <Value : Sized + Sensitive + 'static> Deref for RbList<Value> {
	
	type Target = [Rb<Value>];
	
	fn deref (&self) -> &[Rb<Value>] {
		self.0.deref ()
	}
}


impl <Value : Sized + Sensitive + 'static> AsRef<[Rb<Value>]> for RbList<Value> {
	
	fn as_ref (&self) -> &[Rb<Value>] {
		self.deref ()
	}
}




macro_rules! impl_as_ref {
	( $_type : ident ) => {
		impl AsRef<$_type> for $_type {
			fn as_ref (&self) -> &Self {
				self
			}
		}
	};
}


impl_as_ref! (Token);
impl_as_ref! (Atom);
impl_as_ref! (Glyph);
impl_as_ref! (Separator);

impl_as_ref! (TokenPattern);
impl_as_ref! (AtomPattern);
impl_as_ref! (GlyphPattern);

impl_as_ref! (Text);




impl <Value : Sized + Sensitive + 'static> Drop for RbRef<Value> {
	
	fn drop (&mut self) -> () {
		match self {
			Self::Static (_) => {
				if crate::allocator::USE_MEMZERO {
					let _junk_zero = [0 as u8; mem::size_of::<&'static u8> ()];
					let mut _junk = Self::Static (unsafe { mem::transmute (_junk_zero) });
					mem::swap (self, &mut _junk);
					let _pointer = (&mut _junk) as *mut Self as *mut u8;
					unsafe {
						::memsec::memzero (_pointer, mem::size_of::<Self> ());
					}
					mem::forget (_junk);
				}
			}
			Self::Rc (_) => {
				// NOP
			}
		}
	}
}


impl <Value : Sized + Sensitive + 'static> Drop for RbListRef<Value> {
	
	fn drop (&mut self) -> () {
		match self {
			Self::Static (_reference) => {
				if crate::allocator::USE_MEMZERO {
					let _junk_zero = [0 as u8; mem::size_of::<&'static [u8]> ()];
					let mut _junk = Self::Static (unsafe { mem::transmute (_junk_zero) });
					mem::swap (self, &mut _junk);
					let _pointer = (&mut _junk) as *mut Self as *mut u8;
					unsafe {
						::memsec::memzero (_pointer, mem::size_of::<Self> ());
					}
					mem::forget (_junk);
				}
			}
			Self::Rc (_) => {
				// NOP
			}
		}
	}
}




impl <Value : Sized + Sensitive + 'static> Sensitive for Rb<Value> {}
impl <Value : Sized + Sensitive + 'static> Sensitive for RbList<Value> {}


impl <A : Sensitive, B : Sensitive> Sensitive for (A, B) {}


impl Sensitive for Token {}
impl Sensitive for Atom {}
impl Sensitive for Glyph {}
impl Sensitive for Separator {}

impl Sensitive for TokenPattern {}
impl Sensitive for TokenPatternTags {}
impl Sensitive for SeparatorPattern {}
impl Sensitive for AtomPattern {}
impl Sensitive for GlyphPattern {}




// FIXME!
impl Sensitive for Text {}
impl Sensitive for Bytes {}
impl Sensitive for u128 {}
impl Sensitive for Cow<'static, str> {}


