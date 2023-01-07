

use crate::prelude::*;




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct Rb <Value : Sized + 'static> (RbRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct RbList <Value : Sized + 'static> (RbListRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
enum RbRef <Value : Sized + 'static> {
	Static (&'static Value),
	Rc (Arc<Value>),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
enum RbListRef <Value : Sized + 'static> {
	Static (&'static [Rb<Value>]),
	Rc (Arc<[Rb<Value>]>),
}




impl <Value> RbRef<Value> {
	
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




impl <Value> RbListRef<Value> {
	
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




impl <Value> Rb<Value> {
	
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


impl <Value> Deref for Rb<Value> {
	
	type Target = Value;
	
	fn deref (&self) -> &Value {
		self.0.deref ()
	}
}


impl <Value> AsRef<Value> for Rb<Value> {
	
	fn as_ref (&self) -> &Value {
		self.deref ()
	}
}




impl <Value> RbList <Value> {
	
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


impl <Value> Deref for RbList<Value> {
	
	type Target = [Rb<Value>];
	
	fn deref (&self) -> &[Rb<Value>] {
		self.0.deref ()
	}
}


impl <Value> AsRef<[Rb<Value>]> for RbList<Value> {
	
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


