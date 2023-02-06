

#![ no_implicit_prelude ]


pub mod keys;
pub mod crypto;
pub mod armor;
pub mod coding;
pub mod io;
pub mod low;
pub mod tool;
pub mod macros;


#[ cfg (any (target_os = "linux", target_os = "android", target_os = "openbsd", target_os = "freebsd", target_os = "netbsd", target_os = "macos")) ]
pub mod ssh;

#[ cfg (target_os = "windows") ]
#[ path = "./ssh_no.rs" ]
pub mod ssh;


