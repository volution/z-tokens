

#![ no_implicit_prelude ]




#[ cfg (all (feature = "development", feature = "release", not (feature = "zt-runtime-do-not-enable-this"))) ]
::std::compile_error! ("[c63017a5]  both `development` and `release` features requested!");




pub mod preludes;
pub mod errors;
pub mod memory;


pub mod sensitive;
pub(crate) mod sensitive_memory;


#[ cfg (feature = "zt-runtime-allocator") ]
pub mod allocator;




pub mod crates {
	
	pub use ::vrl_errors;
	pub use ::vrl_preludes;
	
	pub use ::byteorder;
	pub use ::constant_time_eq;
	pub use ::zeroize;
	pub use ::memsec;
	
	pub use ::num_bigint;
	pub use ::num_traits;
	
	pub use ::chrono;
	pub use ::platform_info;
	
	pub use ::paste;
	pub use ::const_format;
}


