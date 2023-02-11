

use ::vrl_preludes::std_plus_extras::*;


use crate::main_helpers::*;
use crate::embeddings::*;
use crate::runtime::*;








pub(crate) fn main_unknown (_commands : Vec<String>, _arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		(&[], &["--project-url"]) =>
			print_and_exit (&[
					PROJECT_URL,
					"\n",
				], true),
		
		#[ cfg (feature = "zt-embedded-help") ]
		(&["help"], _) | (&["h"], _) |
		(&[], &["--help"]) | (&[], &["-h"]) |
		(&[], &["--help-text"]) =>
			print_and_exit (&[
						HELP_HEADER_TEXT,
						HELP_MAIN_TEXT.trim_matches ('\n'),
						"\n",
						HELP_FOOTER_TEXT,
					], true),
		
		(&["version"], _) | (&["v"], _) |
		(&[], &["--version"]) | (&[], &["-v"]) =>
			print_version_and_exit (true),
		
		#[ cfg (feature = "zt-embedded-readme") ]
		(&[], &["--readme"]) |
		(&[], &["--readme-text"]) =>
			print_and_exit (&[README_TEXT], true),
		#[ cfg (feature = "zt-embedded-readme") ]
		(&[], &["--readme-html"]) =>
			print_and_exit (&[README_HTML], true),
		
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom"]) |
		(&[], &["--sbom-text"]) =>
			print_and_exit (&[SBOM_TEXT], true),
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom-html"]) =>
			print_and_exit (&[SBOM_HTML], true),
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom-json"]) =>
			print_and_exit (&[SBOM_JSON], true),
		
		#[ cfg (feature = "zt-embedded-sources") ]
		(&[], &["--sources-md5"]) =>
			dump_and_exit (BUILD_SOURCES_MD5.as_bytes (), true),
		#[ cfg (feature = "zt-embedded-sources") ]
		(&[], &["--sources-cpio"]) =>
			dump_and_exit (BUILD_SOURCES_CPIO_GZ, true),
		
		(&[], _) =>
			print_and_exit (&["[ee] [427cd93b]  expected command and arguments;  see `z-tokens help`;  aborting!", "\n"], false),
		
		_ =>
			print_and_exit (&["[ee] [37d61e27]  invalid command;  see `z-tokens help`;  aborting!", "\n"], false),
	}
}


