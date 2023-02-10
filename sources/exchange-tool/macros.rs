

macro_rules! define_cryptographic_purpose {
	
	( $_visibility : vis $_identifier : ident, $_namespace : ident, $_purpose : ident ) => {
		
		$_visibility const $_identifier : &str = concat! ("z-tokens / exchange / ", stringify! ($_namespace), " / ", stringify! ($_purpose), " / ", "(2023a)");
	};
}




macro_rules! define_cryptographic_material {
	
	( $_visibility : vis $_identifier : ident, $_size : literal ) => {
		
		$_visibility struct $_identifier {
				material : [u8; $_identifier::SIZE],
			}
		
		impl $_identifier {
			
			#[ allow (dead_code) ]
			pub fn wrap (_material : [u8; $_identifier::SIZE]) -> Self {
				Self {
						material : _material,
					}
			}
			
			#[ allow (dead_code) ]
			pub fn wrap_copy (_material : &[u8; $_identifier::SIZE]) -> Self {
				Self {
						material : *_material,
					}
			}
			
			#[ allow (dead_code) ]
			pub fn zero () -> Self {
				Self::wrap ([0u8; $_identifier::SIZE])
			}
			
			pub const SIZE : usize = $_size;
		}
		
		impl CryptographicMaterial<{ $_identifier::SIZE }> for $_identifier {
			
			fn consume (self) -> () {
				// FIXME:  zeroize...
			}
			
			fn access (&self) -> &[u8; $_identifier::SIZE] {
				&self.material
			}
		}
	};
	
	( $_visibility : vis $_identifier : ident, input, slice ) => {
		
		$_visibility struct $_identifier <'a> {
				material : &'a [u8],
			}
		
		impl <'a> $_identifier<'a> {
			
			#[ allow (dead_code) ]
			pub fn wrap (_material : &'a [u8]) -> Self {
				Self {
						material : _material,
					}
			}
			
			#[ allow (dead_code) ]
			pub fn empty () -> Self {
				Self::wrap (&[])
			}
		}
		
		impl <'a> CryptographicInput<'a> for $_identifier<'a> {
			
			fn consume (self) -> () {}
			
			fn access (&self) -> &'a [u8] {
				self.material
			}
		}
	};
}




macro_rules! drop {
	
	( $_identifier : ident ) => {
		let $_identifier = mem::drop ($_identifier);
	};
	
	( $( $_identifier : ident ),+ ) => {
		$( drop! ($_identifier); )+
	}
}




pub(crate) use define_cryptographic_material;
pub(crate) use define_cryptographic_purpose;
pub(crate) use drop;


