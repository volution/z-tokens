

#![ no_implicit_prelude ]




pub use bin::{
		
		bin,
	};


pub use tools::{
		
		main,
		premain,
		
		MainError,
		MainResult,
	};




pub(crate) mod bin;
pub(crate) mod tools;


