

macro_rules! define_cryptographic_context {
	
	( $_visibility : vis $_identifier : ident, $_namespace : ident, $_purpose : ident ) => {
		
		$_visibility const $_identifier : &str = concat! ("z-tokens / exchange / ", stringify! ($_namespace), " / ", stringify! ($_purpose), " / ", "(2023a)");
	};
}


pub(crate) use define_cryptographic_context;


