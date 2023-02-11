

use ::vrl_preludes::std_plus_extras::*;




pub(crate) static PROJECT_URL : &'static str = "https://github.com/volution/z-tokens";




#[ cfg (feature = "zt-embedded-readme") ]
pub(crate) static README_TEXT : &'static str = include_str! ("../embedded/readme/readme.txt");

#[ cfg (feature = "zt-embedded-readme") ]
pub(crate) static README_HTML : &'static str = include_str! ("../embedded/readme/readme.html");




#[ cfg (feature = "zt-embedded-help") ]
pub(crate) static HELP_MAIN_TEXT : &'static str = include_str! ("../embedded/help/main.txt");

#[ cfg (feature = "zt-embedded-help") ]
pub(crate) static HELP_HEADER_TEXT : &'static str = include_str! ("../embedded/help/_header.txt");

#[ cfg (feature = "zt-embedded-help") ]
pub(crate) static HELP_FOOTER_TEXT : &'static str = include_str! ("../embedded/help/_footer.txt");




#[ cfg (feature = "zt-embedded-sbom") ]
pub(crate) static SBOM_TEXT : &'static str = include_str! ("../embedded/sbom/sbom.txt");

#[ cfg (feature = "zt-embedded-sbom") ]
pub(crate) static SBOM_HTML : &'static str = include_str! ("../embedded/sbom/sbom.html");

#[ cfg (feature = "zt-embedded-sbom") ]
pub(crate) static SBOM_JSON : &'static str = include_str! ("../embedded/sbom/sbom.json");




#[ cfg (feature = "zt-embedded-build-meta") ]
pub(crate) static BUILD_VERSION : &'static str = include_str! ("../embedded/build/version.txt");

#[ cfg (feature = "zt-embedded-build-meta") ]
pub(crate) static BUILD_NUMBER : &'static str = include_str! ("../embedded/build/number.txt");

#[ cfg (feature = "zt-embedded-build-meta") ]
pub(crate) static BUILD_TIMESTAMP : &'static str = include_str! ("../embedded/build/timestamp.txt");




#[ cfg (feature = "zt-embedded-sources") ]
pub(crate) static BUILD_SOURCES_HASH : &'static str = include_str! ("../embedded/build/sources.hash");

#[ cfg (feature = "zt-embedded-sources") ]
pub(crate) static BUILD_SOURCES_MD5 : &'static str = include_str! ("../embedded/build/sources.md5");

#[ cfg (feature = "zt-embedded-sources") ]
pub(crate) static BUILD_SOURCES_CPIO_GZ : &'static [u8] = include_bytes! ("../embedded/build/sources.cpio.gz");




#[ cfg (feature = "zt-embedded-build-meta") ]
pub(crate) static BUILD_GIT_HASH : &'static str
	= if let Some (_value) = ::std::option_env! ("__META__BUILD_GIT_HASH") { _value } else { "{unknown-bgh}" };

#[ cfg (feature = "zt-embedded-build-meta") ]
pub(crate) static BUILD_TARGET_TYPE : &'static str =
	if let Some (_value) = ::std::option_env! ("__META__BUILD_TARGET_TYPE") { _value } else { "{unknown-btt}" };


