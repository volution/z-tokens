

#![ no_implicit_prelude ]


#![ allow (unused_parens) ]




pub use tools::{
		
		main,
		premain,
		MainResult,
	};




pub mod model;
pub mod generator;
pub mod randomizer;
pub mod entropy;
pub mod output;
pub mod memory;
pub(crate) mod sensitive;


pub mod patterns;
pub(crate) mod patterns_tokens;
pub(crate) mod patterns_glyphs;
pub(crate) mod patterns_separators;
pub(crate) mod patterns_consts;
pub(crate) mod patterns_macros;


pub mod tools;
pub mod tools_generate;
pub mod tools_patterns;
pub mod tools_flags;


pub(crate) mod prelude;


pub(crate) mod allocator;


