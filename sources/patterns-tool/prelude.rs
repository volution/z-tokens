

#![ allow (unused_imports) ]




pub(crate) use ::z_tokens_runtime::preludes::std_plus_extras::*;
pub(crate) use ::z_tokens_runtime::preludes::errors::*;


pub(crate) use ::z_tokens_patterns::{
		
		model::*,
		randomizer::*,
		entropy::*,
		entropy_estimates::*,
		generator::*,
		output::*,
	};


pub(crate) use ::z_tokens_patterns_definitions::{
		
		all::*,
	};


pub(crate) use ::z_tokens_runtime::{
		
		memory::*,
	};


pub(crate) use ::z_tokens_runtime_flags::{
		
		*,
	};


pub(crate) use ::z_tokens_runtime_hashes::{
		
		crates::xxhash,
	};




pub(crate) use crate::flags::*;




pub(crate) const IO_BUFFER_SIZE : usize = 4 * 1024;


