

use crate::prelude::*;




#[ derive (Clone) ]
pub struct Rb <Value : ?Sized> (Rc<Value>);


#[ derive (Clone) ]
pub struct RbList <Value : ?Sized> (Rc<Box<[Rb<Value>]>>);




impl <Value> Rb<Value> {
	
	pub fn new (_value : Value) -> Self {
		Self (Rc::new (_value))
	}
	
	pub fn clone (&self) -> Self {
		Self (Rc::clone (&self.0))
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
	
	pub fn from_vec (_values : Vec<Value>) -> Self {
		let _values = _values.into_iter () .map (Rb::new) .collect ();
		Self::from_vec_rb (_values)
	}
	
	pub fn from_vec_rb (_values : Vec<Rb<Value>>) -> Self {
		let _values = _values.into_boxed_slice ();
		Self (Rc::new (_values))
	}
	
	pub fn clone (&self) -> Self {
		Self (Rc::clone (&self.0))
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


