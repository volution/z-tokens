

pub(crate) use ::vrl_preludes::std_plus_extras::*;

pub(crate) use ::vrl_errors::*;

pub(crate) use ::vrl_random::*;


pub(crate) use crate::model::*;
pub(crate) use crate::generator::*;
pub(crate) use crate::randomizer::*;
pub(crate) use crate::entropy::*;
pub(crate) use crate::output::*;
pub(crate) use crate::memory::*;

pub(crate) use crate::patterns;


pub(crate) use ::std::process::ExitCode;




pub(crate) const IO_BUFFER_SIZE : usize = 4 * 1024;

pub(crate) const TOKEN_VEC_CAPACITY : usize = 4096;
pub(crate) const TOKEN_STRING_CAPACITY : usize = 128;
pub(crate) const ATOM_VEC_CAPACITY : usize = 16;

