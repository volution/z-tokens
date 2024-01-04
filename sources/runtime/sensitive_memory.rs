

use crate::preludes::std_plus_extras::*;

use crate::sensitive::*;
use crate::memory::*;




include! ("./sensitive_macros.in");








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
			Self::Static (ref mut _ref) =>
				<&'static Value as Sensitive>::erase (_ref),
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}


impl <Value : Sized + Sensitive + 'static> Sensitive for RbListRef<Value> {
	
	fn erase (&mut self) -> () {
		match self {
			Self::Static (ref mut _ref) =>
				<&'static [Rb<Value>] as Sensitive>::erase (_ref),
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}








// FIXME!
// impl_sensitive_drop! (<{Value}> Rb <Value> where <{ Value : Sized + Sensitive + 'static }>);
// impl_sensitive_drop! (<{Value}> RbRef<Value> where <{ Value : Sized + Sensitive + 'static }>);

// FIXME!
// impl_sensitive_drop! (<{Value}> RbList<Value> where <{ Value : Sized + Sensitive + 'static }>);
// impl_sensitive_drop! (<{Value}> RbListRef<Value> where <{ Value : Sized + Sensitive + 'static }>);


