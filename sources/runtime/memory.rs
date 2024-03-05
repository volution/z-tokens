

use crate::preludes::std_plus_extras::*;




#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct Rb <Value : Sized + 'static> (pub(crate) RbRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub struct RbList <Value : Sized + 'static> (pub(crate) RbListRef<Value>);


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub(crate) enum RbRef <Value : Sized + 'static> {
	Static (&'static Value),
	Rc (Arc<Value>),
}


#[ cfg_attr (debug_assertions, derive (Debug)) ]
pub(crate) enum RbListRef <Value : Sized + 'static> {
	Static (&'static [Rb<Value>]),
	Rc (Arc<[Rb<Value>]>),
}








impl <Value : Sized + 'static> RbRef<Value> {
	
	pub fn new (_value : Value) -> Self {
		RbRef::Rc (Arc::new (_value))
	}
	
	pub const fn new_static (_value : &'static Value) -> Self {
		RbRef::Static (_value)
	}
	
	pub fn new_copy (_value : &Value) -> Self where Value : Copy {
		RbRef::new (*_value)
	}
	
	pub fn new_clone (_value : &Value) -> Self where Value : Clone {
		RbRef::new (Clone::clone (_value))
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




impl <Value : Sized + 'static> RbListRef<Value> {
	
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








impl <Value : Sized + 'static> Rb<Value> {
	
	pub fn new (_value : Value) -> Self {
		Self (RbRef::new (_value))
	}
	
	pub const fn new_static (_value : &'static Value) -> Self {
		Self (RbRef::new_static (_value))
	}
	
	pub fn new_copy (_value : &Value) -> Self where Value : Copy {
		Self (RbRef::new_copy (_value))
	}
	
	pub fn new_clone (_value : &Value) -> Self where Value : Clone {
		Self (RbRef::new_clone (_value))
	}
	
	pub fn clone (&self) -> Self {
		Self (self.0.clone ())
	}
}


impl <Value : Sized + 'static> Deref for Rb<Value> {
	
	type Target = Value;
	
	fn deref (&self) -> &Value {
		self.0.deref ()
	}
}


impl <Value : Sized + 'static> AsRef<Value> for Rb<Value> {
	
	fn as_ref (&self) -> &Value {
		self.deref ()
	}
}




impl <Value : Sized + 'static> RbList <Value> {
	
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


impl <Value : Sized + 'static> Deref for RbList<Value> {
	
	type Target = [Rb<Value>];
	
	fn deref (&self) -> &[Rb<Value>] {
		self.0.deref ()
	}
}


impl <Value : Sized + 'static> AsRef<[Rb<Value>]> for RbList<Value> {
	
	fn as_ref (&self) -> &[Rb<Value>] {
		self.deref ()
	}
}


