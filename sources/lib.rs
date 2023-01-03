

#![ no_implicit_prelude ]


pub mod model;
pub mod generator;
pub mod randomizer;
pub mod entropy;
pub mod output;
pub mod memory;
pub mod patterns;


pub mod tools;
pub mod tools_generate;
pub mod tools_patterns;
pub mod tools_flags;


pub(crate) mod prelude;


pub use tools::{ main, MainResult, };


