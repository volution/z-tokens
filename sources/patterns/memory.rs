

use crate::prelude::*;




macro_rules! impl_as_ref {
	( <{ $( $_template_a : tt )* }> $_type : ty where <{ $( $_where : tt )+ }> ) => {
		impl < $( $_template_a )* > AsRef<Self> for $_type where $( $_where )+ {
			fn as_ref (&self) -> &Self {
				self
			}
		}
	};
	( <{ $( $_template_a : tt )* }> $_type : ty ) => {
		impl < $( $_template_a )* > AsRef<Self> for $_type {
			fn as_ref (&self) -> &Self {
				self
			}
		}
	};
	( $_type : ty ) => {
		impl AsRef<Self> for $_type {
			fn as_ref (&self) -> &Self {
				self
			}
		}
	};
}


impl_as_ref! (Token);
impl_as_ref! (Atom);
impl_as_ref! (Glyph);
impl_as_ref! (Separator);

impl_as_ref! (TokenPattern);
impl_as_ref! (TokenPatternTags);
impl_as_ref! (SeparatorPattern);
impl_as_ref! (AtomPattern);
impl_as_ref! (GlyphPattern);

impl_as_ref! (Text);
impl_as_ref! (Bytes);

impl_as_ref! (IntegerFormat);
impl_as_ref! (BytesFormat);
impl_as_ref! (TimestampFormat);


