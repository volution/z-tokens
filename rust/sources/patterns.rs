

use crate::prelude::*;




pub mod glyphs {
	
	use super::*;
	use super::consts::ascii::*;
	use super::consts::mnemonic::*;
	
	
	
	
	macro_rules! define_set {
		( $_visibility : vis $_pattern : ident, $_variant : ident, [ $( $_char : expr, )* ] ) => {
			::paste::paste! {
				
				$(
					static [< _ $_pattern __ $_char __TEXT >] : &Text = & Text::$_variant ($_char);
					static [< _ $_pattern __ $_char __GLYPH >] : &Glyph = & Glyph::Text (Rb::new_static ( [< _ $_pattern __ $_char __TEXT >] ));
				)*
				
				#[ doc = concat! ( "Glyph character set for ", $( "`", stringify! ($_char), "` " ),*, "." ) ]
				$_visibility static [< $_pattern _SET >] : &[Rb<Glyph>] = &[ $(
						Rb::new_static ( [< _ $_pattern __ $_char __GLYPH >] ),
					)* ];
				
				$_visibility static [< $_pattern _GLYPH >] : &GlyphPattern = & GlyphPattern::Set (RbList::from_static ( [< $_pattern _SET >] ));
				$_visibility static [< $_pattern _ATOM >] : &AtomPattern = & AtomPattern::Glyph (Rb::new_static ( [< $_pattern _GLYPH >] ));
				$_visibility static [< $_pattern _TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _ATOM >] ));
			}
		};
	}
	
	
	
	
	define_set! (pub DIGIT_BASE2, Char, [ '0', '1', ]);
	define_set! (pub DIGIT_BASE8, Char, [ '0', '1', '2', '3', '4', '5', '6', '7', ]);
	define_set! (pub DIGIT_BASE10, Char, [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ]);
	
	define_set! (pub DIGIT_BASE16, Char, [
			'0', '1', '2', '3', '4', '5', '6', '7',
			'8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
		]);
	
	define_set! (pub DIGIT_BASE32_HEX, Char, [
			'0', '1', '2', '3', '4', '5', '6', '7',
			'8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
		]);
	
	
	
	
	// NOTE:  =>  https://www.ietf.org/rfc/rfc4648.html
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" abcdefgh ijklmnop qrstuvw xyz234567 """ if c != " " ]))'
	define_set! (pub DIGIT_BASE32_RFC, Char, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
			'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
			'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
			'y', 'z', '2', '3', '4', '5', '6', '7',
		]);
	
	// NOTE:  =>  https://www.ietf.org/rfc/rfc4648.html
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789 +/ """ if c != " " ]))'
	define_set! (pub DIGIT_BASE64_RFC, Char, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
			'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
			'w', 'x', 'y', 'z', '0', '1', '2', '3',
			'4', '5', '6', '7', '8', '9', C2B, C2F,
		]);
	
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789 -_ """ if c != " " ]))'
	define_set! (pub DIGIT_BASE64_URL, Char, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
			'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
			'w', 'x', 'y', 'z', '0', '1', '2', '3',
			'4', '5', '6', '7', '8', '9', C2D, C5F,
		]);
	
	
	
	
	// NOTE:  =>  https://en.bitcoinwiki.org/wiki/Base58
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" 123456789 ABCDEFGH JKLMN PQRSTUVWXYZ abcdefghijk mnopqrstuvwxyz """ if c != " " ]))'
	define_set! (pub DIGIT_BASE58, Char, [
			'1', '2', '3', '4', '5', '6', '7', '8', '9',
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'J', 'K', 'L', 'M', 'N',
			'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
			'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
		]);
	
	// NOTE:  =>  https://en.bitcoin.it/wiki/BIP_0173
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" qpzry9x8 gf2tvdw0 s3jn54kh ce6mua7l """ if c != " " ]))'
	define_set! (pub DIGIT_BECH32, Char, [
			'q', 'p', 'z', 'r', 'y', '9', 'x', '8',
			'g', 'f', '2', 't', 'v', 'd', 'w', '0',
			's', '3', 'j', 'n', '5', '4', 'k', 'h',
			'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
		]);
	
	// NOTE:  =>  https://rfc.zeromq.org/spec/32/
	define_set! (pub DIGIT_Z85, Char, [
			'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
			'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
			'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
			'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
			'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', C2E, C2D, C3A, C2B, C3D, C5E, C21, C2F,
			C2A, C3F, C26, C3C, C3E, C28, C29, C5B, C5D, C7B,
			C7D, C40, C25, C24, C23,
		]);
	
	
	
	
	define_set! (pub ASCII_VOWEL_LOWER, Char, [
			'a', 'e', 'i', 'o', 'u',
		]);
	define_set! (pub ASCII_VOWEL_UPPER, Char, [
			'A', 'B', 'I', 'O', 'U',
		]);
	define_set! (pub ASCII_VOWEL_MIXED, Char, [
			'a', 'e', 'i', 'o', 'u',
			'A', 'B', 'I', 'O', 'U',
		]);
	
	
	define_set! (pub ASCII_CONSONANT_LOWER, Char, [
			'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
		]);
	define_set! (pub ASCII_CONSONANT_UPPER, Char, [
			'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
		]);
	define_set! (pub ASCII_CONSONANT_MIXED, Char, [
			'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
			'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
		]);
	
	
	define_set! (pub ASCII_LETTER_LOWER, Char, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
		]);
	define_set! (pub ASCII_LETTER_UPPER, Char, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
		]);
	define_set! (pub ASCII_LETTER_MIXED, Char, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
		]);
	
	
	// NOTE:  #>  python -c 'print (", ".join ([ "C%0X" % ord (c) for c in r"""!"#$%&'\''()*+,-./:;<=>?@[\]^_`{|}~""" ]))'
	define_set! (pub ASCII_SYMBOL, Char, [
			C21, C22, C23, C24, C25, C26, C27, C28, C29, C2A, C2B, C2C, C2D, C2E, C2F, C3A, C3B, C3C, C3D, C3E, C3F, C40, C5B, C5C, C5D, C5E, C5F, C60, C7B, C7C, C7D, C7E,
		]);
	
	// NOTE:  #>  python -c 'print ("".join ([ chr(c) for c in range (33, 127) ]))'
	// NOTE:  #>  python -c 'print (", ".join ([ "C%0X" % ord (c) for c in r"""!"#$%&'\''()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~""" ]))'
	define_set! (pub ASCII_PRINTABLE, Char, [
			     C21, C22, C23, C24, C25, C26, C27, C28, C29, C2A, C2B, C2C, C2D, C2E, C2F,
			C30, C31, C32, C33, C34, C35, C36, C37, C38, C39, C3A, C3B, C3C, C3D, C3E, C3F,
			C40, C41, C42, C43, C44, C45, C46, C47, C48, C49, C4A, C4B, C4C, C4D, C4E, C4F,
			C50, C51, C52, C53, C54, C55, C56, C57, C58, C59, C5A, C5B, C5C, C5D, C5E, C5F,
			C60, C61, C62, C63, C64, C65, C66, C67, C68, C69, C6A, C6B, C6C, C6D, C6E, C6F,
			C70, C71, C72, C73, C74, C75, C76, C77, C78, C79, C7A, C7B, C7C, C7D, C7E,
		]);
	
	
	
	
	// NOTE:  =>  https://github.com/dsw/proquint/blob/master/proquint-proposal.txt
	
	define_set! (pub PROQUINT_CONSONANT_LOWER, Char, [
			'b', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
			'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
		]);
	define_set! (pub PROQUINT_VOWEL_LOWER, Char, [
			'a', 'i', 'o', 'u',
		]);
	
	define_set! (pub PROQUINT_CONSONANT_UPPER, Char, [
			'B', 'D', 'F', 'G', 'H', 'J', 'K', 'L',
			'M', 'N', 'P', 'R', 'S', 'T', 'V', 'Z',
		]);
	define_set! (pub PROQUINT_VOWEL_UPPER, Char, [
			'A', 'I', 'O', 'U',
		]);
	
	
	
	
	define_set! (pub MNEMONIC_WORD, Str, [
			MW0001, MW0002, MW0003, MW0004, MW0005, MW0006, MW0007, MW0008, MW0009, MW0010, MW0011, MW0012, MW0013, MW0014, MW0015, MW0016, MW0017, MW0018, MW0019, MW0020, MW0021, MW0022, MW0023, MW0024, MW0025, MW0026, MW0027, MW0028, MW0029, MW0030, MW0031, MW0032, MW0033, MW0034, MW0035, MW0036, MW0037, MW0038, MW0039, MW0040, MW0041, MW0042, MW0043, MW0044, MW0045, MW0046, MW0047, MW0048, MW0049, MW0050, MW0051, MW0052, MW0053, MW0054, MW0055, MW0056, MW0057, MW0058, MW0059, MW0060, MW0061, MW0062, MW0063, MW0064, MW0065, MW0066, MW0067, MW0068, MW0069, MW0070, MW0071, MW0072, MW0073, MW0074, MW0075, MW0076, MW0077, MW0078, MW0079, MW0080, MW0081, MW0082, MW0083, MW0084, MW0085, MW0086, MW0087, MW0088, MW0089, MW0090, MW0091, MW0092, MW0093, MW0094, MW0095, MW0096, MW0097, MW0098, MW0099, MW0100, 
			MW0101, MW0102, MW0103, MW0104, MW0105, MW0106, MW0107, MW0108, MW0109, MW0110, MW0111, MW0112, MW0113, MW0114, MW0115, MW0116, MW0117, MW0118, MW0119, MW0120, MW0121, MW0122, MW0123, MW0124, MW0125, MW0126, MW0127, MW0128, MW0129, MW0130, MW0131, MW0132, MW0133, MW0134, MW0135, MW0136, MW0137, MW0138, MW0139, MW0140, MW0141, MW0142, MW0143, MW0144, MW0145, MW0146, MW0147, MW0148, MW0149, MW0150, MW0151, MW0152, MW0153, MW0154, MW0155, MW0156, MW0157, MW0158, MW0159, MW0160, MW0161, MW0162, MW0163, MW0164, MW0165, MW0166, MW0167, MW0168, MW0169, MW0170, MW0171, MW0172, MW0173, MW0174, MW0175, MW0176, MW0177, MW0178, MW0179, MW0180, MW0181, MW0182, MW0183, MW0184, MW0185, MW0186, MW0187, MW0188, MW0189, MW0190, MW0191, MW0192, MW0193, MW0194, MW0195, MW0196, MW0197, MW0198, MW0199, MW0200, 
			MW0201, MW0202, MW0203, MW0204, MW0205, MW0206, MW0207, MW0208, MW0209, MW0210, MW0211, MW0212, MW0213, MW0214, MW0215, MW0216, MW0217, MW0218, MW0219, MW0220, MW0221, MW0222, MW0223, MW0224, MW0225, MW0226, MW0227, MW0228, MW0229, MW0230, MW0231, MW0232, MW0233, MW0234, MW0235, MW0236, MW0237, MW0238, MW0239, MW0240, MW0241, MW0242, MW0243, MW0244, MW0245, MW0246, MW0247, MW0248, MW0249, MW0250, MW0251, MW0252, MW0253, MW0254, MW0255, MW0256, MW0257, MW0258, MW0259, MW0260, MW0261, MW0262, MW0263, MW0264, MW0265, MW0266, MW0267, MW0268, MW0269, MW0270, MW0271, MW0272, MW0273, MW0274, MW0275, MW0276, MW0277, MW0278, MW0279, MW0280, MW0281, MW0282, MW0283, MW0284, MW0285, MW0286, MW0287, MW0288, MW0289, MW0290, MW0291, MW0292, MW0293, MW0294, MW0295, MW0296, MW0297, MW0298, MW0299, MW0300, 
			MW0301, MW0302, MW0303, MW0304, MW0305, MW0306, MW0307, MW0308, MW0309, MW0310, MW0311, MW0312, MW0313, MW0314, MW0315, MW0316, MW0317, MW0318, MW0319, MW0320, MW0321, MW0322, MW0323, MW0324, MW0325, MW0326, MW0327, MW0328, MW0329, MW0330, MW0331, MW0332, MW0333, MW0334, MW0335, MW0336, MW0337, MW0338, MW0339, MW0340, MW0341, MW0342, MW0343, MW0344, MW0345, MW0346, MW0347, MW0348, MW0349, MW0350, MW0351, MW0352, MW0353, MW0354, MW0355, MW0356, MW0357, MW0358, MW0359, MW0360, MW0361, MW0362, MW0363, MW0364, MW0365, MW0366, MW0367, MW0368, MW0369, MW0370, MW0371, MW0372, MW0373, MW0374, MW0375, MW0376, MW0377, MW0378, MW0379, MW0380, MW0381, MW0382, MW0383, MW0384, MW0385, MW0386, MW0387, MW0388, MW0389, MW0390, MW0391, MW0392, MW0393, MW0394, MW0395, MW0396, MW0397, MW0398, MW0399, MW0400, 
			MW0401, MW0402, MW0403, MW0404, MW0405, MW0406, MW0407, MW0408, MW0409, MW0410, MW0411, MW0412, MW0413, MW0414, MW0415, MW0416, MW0417, MW0418, MW0419, MW0420, MW0421, MW0422, MW0423, MW0424, MW0425, MW0426, MW0427, MW0428, MW0429, MW0430, MW0431, MW0432, MW0433, MW0434, MW0435, MW0436, MW0437, MW0438, MW0439, MW0440, MW0441, MW0442, MW0443, MW0444, MW0445, MW0446, MW0447, MW0448, MW0449, MW0450, MW0451, MW0452, MW0453, MW0454, MW0455, MW0456, MW0457, MW0458, MW0459, MW0460, MW0461, MW0462, MW0463, MW0464, MW0465, MW0466, MW0467, MW0468, MW0469, MW0470, MW0471, MW0472, MW0473, MW0474, MW0475, MW0476, MW0477, MW0478, MW0479, MW0480, MW0481, MW0482, MW0483, MW0484, MW0485, MW0486, MW0487, MW0488, MW0489, MW0490, MW0491, MW0492, MW0493, MW0494, MW0495, MW0496, MW0497, MW0498, MW0499, MW0500, 
			MW0501, MW0502, MW0503, MW0504, MW0505, MW0506, MW0507, MW0508, MW0509, MW0510, MW0511, MW0512, MW0513, MW0514, MW0515, MW0516, MW0517, MW0518, MW0519, MW0520, MW0521, MW0522, MW0523, MW0524, MW0525, MW0526, MW0527, MW0528, MW0529, MW0530, MW0531, MW0532, MW0533, MW0534, MW0535, MW0536, MW0537, MW0538, MW0539, MW0540, MW0541, MW0542, MW0543, MW0544, MW0545, MW0546, MW0547, MW0548, MW0549, MW0550, MW0551, MW0552, MW0553, MW0554, MW0555, MW0556, MW0557, MW0558, MW0559, MW0560, MW0561, MW0562, MW0563, MW0564, MW0565, MW0566, MW0567, MW0568, MW0569, MW0570, MW0571, MW0572, MW0573, MW0574, MW0575, MW0576, MW0577, MW0578, MW0579, MW0580, MW0581, MW0582, MW0583, MW0584, MW0585, MW0586, MW0587, MW0588, MW0589, MW0590, MW0591, MW0592, MW0593, MW0594, MW0595, MW0596, MW0597, MW0598, MW0599, MW0600, 
			MW0601, MW0602, MW0603, MW0604, MW0605, MW0606, MW0607, MW0608, MW0609, MW0610, MW0611, MW0612, MW0613, MW0614, MW0615, MW0616, MW0617, MW0618, MW0619, MW0620, MW0621, MW0622, MW0623, MW0624, MW0625, MW0626, MW0627, MW0628, MW0629, MW0630, MW0631, MW0632, MW0633, MW0634, MW0635, MW0636, MW0637, MW0638, MW0639, MW0640, MW0641, MW0642, MW0643, MW0644, MW0645, MW0646, MW0647, MW0648, MW0649, MW0650, MW0651, MW0652, MW0653, MW0654, MW0655, MW0656, MW0657, MW0658, MW0659, MW0660, MW0661, MW0662, MW0663, MW0664, MW0665, MW0666, MW0667, MW0668, MW0669, MW0670, MW0671, MW0672, MW0673, MW0674, MW0675, MW0676, MW0677, MW0678, MW0679, MW0680, MW0681, MW0682, MW0683, MW0684, MW0685, MW0686, MW0687, MW0688, MW0689, MW0690, MW0691, MW0692, MW0693, MW0694, MW0695, MW0696, MW0697, MW0698, MW0699, MW0700, 
			MW0701, MW0702, MW0703, MW0704, MW0705, MW0706, MW0707, MW0708, MW0709, MW0710, MW0711, MW0712, MW0713, MW0714, MW0715, MW0716, MW0717, MW0718, MW0719, MW0720, MW0721, MW0722, MW0723, MW0724, MW0725, MW0726, MW0727, MW0728, MW0729, MW0730, MW0731, MW0732, MW0733, MW0734, MW0735, MW0736, MW0737, MW0738, MW0739, MW0740, MW0741, MW0742, MW0743, MW0744, MW0745, MW0746, MW0747, MW0748, MW0749, MW0750, MW0751, MW0752, MW0753, MW0754, MW0755, MW0756, MW0757, MW0758, MW0759, MW0760, MW0761, MW0762, MW0763, MW0764, MW0765, MW0766, MW0767, MW0768, MW0769, MW0770, MW0771, MW0772, MW0773, MW0774, MW0775, MW0776, MW0777, MW0778, MW0779, MW0780, MW0781, MW0782, MW0783, MW0784, MW0785, MW0786, MW0787, MW0788, MW0789, MW0790, MW0791, MW0792, MW0793, MW0794, MW0795, MW0796, MW0797, MW0798, MW0799, MW0800, 
			MW0801, MW0802, MW0803, MW0804, MW0805, MW0806, MW0807, MW0808, MW0809, MW0810, MW0811, MW0812, MW0813, MW0814, MW0815, MW0816, MW0817, MW0818, MW0819, MW0820, MW0821, MW0822, MW0823, MW0824, MW0825, MW0826, MW0827, MW0828, MW0829, MW0830, MW0831, MW0832, MW0833, MW0834, MW0835, MW0836, MW0837, MW0838, MW0839, MW0840, MW0841, MW0842, MW0843, MW0844, MW0845, MW0846, MW0847, MW0848, MW0849, MW0850, MW0851, MW0852, MW0853, MW0854, MW0855, MW0856, MW0857, MW0858, MW0859, MW0860, MW0861, MW0862, MW0863, MW0864, MW0865, MW0866, MW0867, MW0868, MW0869, MW0870, MW0871, MW0872, MW0873, MW0874, MW0875, MW0876, MW0877, MW0878, MW0879, MW0880, MW0881, MW0882, MW0883, MW0884, MW0885, MW0886, MW0887, MW0888, MW0889, MW0890, MW0891, MW0892, MW0893, MW0894, MW0895, MW0896, MW0897, MW0898, MW0899, MW0900, 
			MW0901, MW0902, MW0903, MW0904, MW0905, MW0906, MW0907, MW0908, MW0909, MW0910, MW0911, MW0912, MW0913, MW0914, MW0915, MW0916, MW0917, MW0918, MW0919, MW0920, MW0921, MW0922, MW0923, MW0924, MW0925, MW0926, MW0927, MW0928, MW0929, MW0930, MW0931, MW0932, MW0933, MW0934, MW0935, MW0936, MW0937, MW0938, MW0939, MW0940, MW0941, MW0942, MW0943, MW0944, MW0945, MW0946, MW0947, MW0948, MW0949, MW0950, MW0951, MW0952, MW0953, MW0954, MW0955, MW0956, MW0957, MW0958, MW0959, MW0960, MW0961, MW0962, MW0963, MW0964, MW0965, MW0966, MW0967, MW0968, MW0969, MW0970, MW0971, MW0972, MW0973, MW0974, MW0975, MW0976, MW0977, MW0978, MW0979, MW0980, MW0981, MW0982, MW0983, MW0984, MW0985, MW0986, MW0987, MW0988, MW0989, MW0990, MW0991, MW0992, MW0993, MW0994, MW0995, MW0996, MW0997, MW0998, MW0999, MW1000, 
			MW1001, MW1002, MW1003, MW1004, MW1005, MW1006, MW1007, MW1008, MW1009, MW1010, MW1011, MW1012, MW1013, MW1014, MW1015, MW1016, MW1017, MW1018, MW1019, MW1020, MW1021, MW1022, MW1023, MW1024, MW1025, MW1026, MW1027, MW1028, MW1029, MW1030, MW1031, MW1032, MW1033, MW1034, MW1035, MW1036, MW1037, MW1038, MW1039, MW1040, MW1041, MW1042, MW1043, MW1044, MW1045, MW1046, MW1047, MW1048, MW1049, MW1050, MW1051, MW1052, MW1053, MW1054, MW1055, MW1056, MW1057, MW1058, MW1059, MW1060, MW1061, MW1062, MW1063, MW1064, MW1065, MW1066, MW1067, MW1068, MW1069, MW1070, MW1071, MW1072, MW1073, MW1074, MW1075, MW1076, MW1077, MW1078, MW1079, MW1080, MW1081, MW1082, MW1083, MW1084, MW1085, MW1086, MW1087, MW1088, MW1089, MW1090, MW1091, MW1092, MW1093, MW1094, MW1095, MW1096, MW1097, MW1098, MW1099, MW1100, 
			MW1101, MW1102, MW1103, MW1104, MW1105, MW1106, MW1107, MW1108, MW1109, MW1110, MW1111, MW1112, MW1113, MW1114, MW1115, MW1116, MW1117, MW1118, MW1119, MW1120, MW1121, MW1122, MW1123, MW1124, MW1125, MW1126, MW1127, MW1128, MW1129, MW1130, MW1131, MW1132, MW1133, MW1134, MW1135, MW1136, MW1137, MW1138, MW1139, MW1140, MW1141, MW1142, MW1143, MW1144, MW1145, MW1146, MW1147, MW1148, MW1149, MW1150, MW1151, MW1152, MW1153, MW1154, MW1155, MW1156, MW1157, MW1158, MW1159, MW1160, MW1161, MW1162, MW1163, MW1164, MW1165, MW1166, MW1167, MW1168, MW1169, MW1170, MW1171, MW1172, MW1173, MW1174, MW1175, MW1176, MW1177, MW1178, MW1179, MW1180, MW1181, MW1182, MW1183, MW1184, MW1185, MW1186, MW1187, MW1188, MW1189, MW1190, MW1191, MW1192, MW1193, MW1194, MW1195, MW1196, MW1197, MW1198, MW1199, MW1200, 
			MW1201, MW1202, MW1203, MW1204, MW1205, MW1206, MW1207, MW1208, MW1209, MW1210, MW1211, MW1212, MW1213, MW1214, MW1215, MW1216, MW1217, MW1218, MW1219, MW1220, MW1221, MW1222, MW1223, MW1224, MW1225, MW1226, MW1227, MW1228, MW1229, MW1230, MW1231, MW1232, MW1233, MW1234, MW1235, MW1236, MW1237, MW1238, MW1239, MW1240, MW1241, MW1242, MW1243, MW1244, MW1245, MW1246, MW1247, MW1248, MW1249, MW1250, MW1251, MW1252, MW1253, MW1254, MW1255, MW1256, MW1257, MW1258, MW1259, MW1260, MW1261, MW1262, MW1263, MW1264, MW1265, MW1266, MW1267, MW1268, MW1269, MW1270, MW1271, MW1272, MW1273, MW1274, MW1275, MW1276, MW1277, MW1278, MW1279, MW1280, MW1281, MW1282, MW1283, MW1284, MW1285, MW1286, MW1287, MW1288, MW1289, MW1290, MW1291, MW1292, MW1293, MW1294, MW1295, MW1296, MW1297, MW1298, MW1299, MW1300, 
			MW1301, MW1302, MW1303, MW1304, MW1305, MW1306, MW1307, MW1308, MW1309, MW1310, MW1311, MW1312, MW1313, MW1314, MW1315, MW1316, MW1317, MW1318, MW1319, MW1320, MW1321, MW1322, MW1323, MW1324, MW1325, MW1326, MW1327, MW1328, MW1329, MW1330, MW1331, MW1332, MW1333, MW1334, MW1335, MW1336, MW1337, MW1338, MW1339, MW1340, MW1341, MW1342, MW1343, MW1344, MW1345, MW1346, MW1347, MW1348, MW1349, MW1350, MW1351, MW1352, MW1353, MW1354, MW1355, MW1356, MW1357, MW1358, MW1359, MW1360, MW1361, MW1362, MW1363, MW1364, MW1365, MW1366, MW1367, MW1368, MW1369, MW1370, MW1371, MW1372, MW1373, MW1374, MW1375, MW1376, MW1377, MW1378, MW1379, MW1380, MW1381, MW1382, MW1383, MW1384, MW1385, MW1386, MW1387, MW1388, MW1389, MW1390, MW1391, MW1392, MW1393, MW1394, MW1395, MW1396, MW1397, MW1398, MW1399, MW1400, 
			MW1401, MW1402, MW1403, MW1404, MW1405, MW1406, MW1407, MW1408, MW1409, MW1410, MW1411, MW1412, MW1413, MW1414, MW1415, MW1416, MW1417, MW1418, MW1419, MW1420, MW1421, MW1422, MW1423, MW1424, MW1425, MW1426, MW1427, MW1428, MW1429, MW1430, MW1431, MW1432, MW1433, MW1434, MW1435, MW1436, MW1437, MW1438, MW1439, MW1440, MW1441, MW1442, MW1443, MW1444, MW1445, MW1446, MW1447, MW1448, MW1449, MW1450, MW1451, MW1452, MW1453, MW1454, MW1455, MW1456, MW1457, MW1458, MW1459, MW1460, MW1461, MW1462, MW1463, MW1464, MW1465, MW1466, MW1467, MW1468, MW1469, MW1470, MW1471, MW1472, MW1473, MW1474, MW1475, MW1476, MW1477, MW1478, MW1479, MW1480, MW1481, MW1482, MW1483, MW1484, MW1485, MW1486, MW1487, MW1488, MW1489, MW1490, MW1491, MW1492, MW1493, MW1494, MW1495, MW1496, MW1497, MW1498, MW1499, MW1500, 
			MW1501, MW1502, MW1503, MW1504, MW1505, MW1506, MW1507, MW1508, MW1509, MW1510, MW1511, MW1512, MW1513, MW1514, MW1515, MW1516, MW1517, MW1518, MW1519, MW1520, MW1521, MW1522, MW1523, MW1524, MW1525, MW1526, MW1527, MW1528, MW1529, MW1530, MW1531, MW1532, MW1533, MW1534, MW1535, MW1536, MW1537, MW1538, MW1539, MW1540, MW1541, MW1542, MW1543, MW1544, MW1545, MW1546, MW1547, MW1548, MW1549, MW1550, MW1551, MW1552, MW1553, MW1554, MW1555, MW1556, MW1557, MW1558, MW1559, MW1560, MW1561, MW1562, MW1563, MW1564, MW1565, MW1566, MW1567, MW1568, MW1569, MW1570, MW1571, MW1572, MW1573, MW1574, MW1575, MW1576, MW1577, MW1578, MW1579, MW1580, MW1581, MW1582, MW1583, MW1584, MW1585, MW1586, MW1587, MW1588, MW1589, MW1590, MW1591, MW1592, MW1593, MW1594, MW1595, MW1596, MW1597, MW1598, MW1599, MW1600, 
			MW1601, MW1602, MW1603, MW1604, MW1605, MW1606, MW1607, MW1608, MW1609, MW1610, MW1611, MW1612, MW1613, MW1614, MW1615, MW1616, MW1617, MW1618, MW1619, MW1620, MW1621, MW1622, MW1623, MW1624, MW1625, MW1626, MW1627, MW1628, MW1629, MW1630, MW1631, MW1632, MW1633,
		]);
}




pub mod tokens {
	
	use super::*;
	
	
	
	
	macro_rules! define_sequence {
		( $_visibility : vis $_pattern : ident, $_identifier : literal, [ $( $_element : expr, )* ], $_separator : expr ) => {
			::paste::paste! {
				
				static [< _ $_pattern __SEQUENCE >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ($_element),
					)* ];
				
				static [< _ $_pattern __NO_NAME >] : &TokenPattern = & TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SEQUENCE >] ), $_separator);
				$_visibility static [< $_pattern >] : &TokenPattern = & TokenPattern::Named ($_identifier, Rb::new_static ( [< _ $_pattern __NO_NAME >] ));
			}
		};
	}
	
	
	macro_rules! define_repeat {
		
		( $_visibility : vis $_pattern : ident, $_identifier : literal, $_element : expr, $_separator : expr, ( $_length : tt : $_each : tt ) ) => {
			macros::__count_call_with! ( [ $_length : $_each ] => define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, ));
		};
		
		( $_visibility : vis $_pattern : ident, $_identifier : literal, $_element : expr, $_separator : expr, [ $( $_count : literal, )* ] ) => {
			::paste::paste! {
				
				$(
					static [< _ $_pattern _ $_count __NO_NAME >] : &TokenPattern = & TokenPattern::Repeat (Rb::new_static ($_element), $_separator, $_count);
					$_visibility static [< $_pattern _ $_count >] : &TokenPattern = & TokenPattern::Named (concat! ($_identifier, "-", $_count), Rb::new_static ( [< _ $_pattern _ $_count __NO_NAME >] ));
				)*
				
				$_visibility static [< $_pattern _ALL >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ( [< $_pattern _ $_count >] ),
					)* ];
			}
		};
	}
	
	
	
	
	define_repeat! (pub DIGITS_BASE10, "digits-base10", glyphs::DIGIT_BASE10_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN), (256 : 4));
	
	define_repeat! (pub DIGITS_BASE2, "digits-base2", glyphs::DIGIT_BASE2_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	define_repeat! (pub DIGITS_BASE8, "digits-base8", glyphs::DIGIT_BASE8_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	
	define_repeat! (pub DIGITS_BASE16, "digits-base16", glyphs::DIGIT_BASE16_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN), (256 : 4));
	
	define_repeat! (pub DIGITS_BASE32_HEX, "digits-base32-hex", glyphs::DIGIT_BASE32_HEX_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	define_repeat! (pub DIGITS_BASE32_RFC, "digits-base32-rfc", glyphs::DIGIT_BASE32_RFC_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	
	define_repeat! (pub DIGITS_BASE64_URL, "digits-base64-url", glyphs::DIGIT_BASE64_URL_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	define_repeat! (pub DIGITS_BASE64_RFC, "digits-base64-rfc", glyphs::DIGIT_BASE64_RFC_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	
	define_repeat! (pub DIGITS_BASE58, "digits-base58", glyphs::DIGIT_BASE58_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (256 : 8));
	
	define_repeat! (pub DIGITS_BECH32, "digits-bech32", glyphs::DIGIT_BECH32_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN), (256 : 4));
	
	define_repeat! (pub DIGITS_Z85, "digits-z85", glyphs::DIGIT_Z85_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN), (260 : 5));
	
	
	
	
	define_repeat! (pub ASCII_LETTER_LOWER, "ascii-lower", glyphs::ASCII_LETTER_LOWER_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (256 : 4));
	define_repeat! (pub ASCII_LETTER_UPPER, "ascii-upper", glyphs::ASCII_LETTER_UPPER_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (256 : 4));
	define_repeat! (pub ASCII_LETTER_MIXED, "ascii-mixed", glyphs::ASCII_LETTER_MIXED_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (256 : 4));
	
	define_repeat! (pub ASCII_SYMBOLS, "ascii-symbols", glyphs::ASCII_SYMBOL_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (256 : 4));
	define_repeat! (pub ASCII_PRINTABLE, "ascii-any", glyphs::ASCII_PRINTABLE_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (256 : 4));
	
	
	
	
	define_sequence! (pub ASCII_CONSONANT_VOWEL_LOWER_WORD, "cv-lower-word", [
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_sequence! (pub ASCII_CONSONANT_VOWEL_UPPER_WORD, "cv-upper-word", [
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_sequence! (pub ASCII_CONSONANT_VOWEL_MIXED_WORD, "cv-mixed-word", [
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_repeat! (pub ASCII_CONSONANT_VOWEL_LOWER, "cv-lower", ASCII_CONSONANT_VOWEL_LOWER_WORD, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (64 : 1));
	define_repeat! (pub ASCII_CONSONANT_VOWEL_UPPER, "cv-upper", ASCII_CONSONANT_VOWEL_UPPER_WORD, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (64 : 1));
	define_repeat! (pub ASCII_CONSONANT_VOWEL_MIXED, "cv-mixed", ASCII_CONSONANT_VOWEL_MIXED_WORD, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (64 : 1));
	
	
	
	
	define_sequence! (pub PROQUINT_LOWER_WORD, "proquint-lower-word", [
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_sequence! (pub PROQUINT_UPPER_WORD, "proquint-upper-word", [
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_repeat! (pub PROQUINT_LOWER, "proquint-lower", PROQUINT_LOWER_WORD, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (64 : 1));
	define_repeat! (pub PROQUINT_UPPER, "proquint-upper", PROQUINT_UPPER_WORD, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (64 : 1));
	
	
	
	
	define_sequence! (pub MNEMONIC_TUPLE, "mnemonic-tuple", [
			glyphs::MNEMONIC_WORD_TOKEN,
			glyphs::MNEMONIC_WORD_TOKEN,
			glyphs::MNEMONIC_WORD_TOKEN,
		], Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN));
	
	define_repeat! (pub MNEMONIC, "mnemonic", MNEMONIC_TUPLE, Rb::new_static (separators::SPACE_HYPHEN_SPACE_MANDATORY_INFIX_PATTERN), (66 : 1));
	
	
	
	
	pub static ALL : &[&[Rb<TokenPattern>]] = &[
			
			DIGITS_BASE2_ALL,
			DIGITS_BASE8_ALL,
			DIGITS_BASE10_ALL,
			DIGITS_BASE16_ALL,
			DIGITS_BASE32_HEX_ALL,
			DIGITS_BASE32_RFC_ALL,
			DIGITS_BASE64_URL_ALL,
			DIGITS_BASE64_RFC_ALL,
			DIGITS_BASE58_ALL,
			DIGITS_BECH32_ALL,
			DIGITS_Z85_ALL,
			
			ASCII_LETTER_LOWER_ALL,
			ASCII_LETTER_UPPER_ALL,
			ASCII_LETTER_MIXED_ALL,
			
			ASCII_SYMBOLS_ALL,
			ASCII_PRINTABLE_ALL,
			
			ASCII_CONSONANT_VOWEL_LOWER_ALL,
			ASCII_CONSONANT_VOWEL_UPPER_ALL,
			ASCII_CONSONANT_VOWEL_MIXED_ALL,
			
			PROQUINT_LOWER_ALL,
			PROQUINT_UPPER_ALL,
			
			MNEMONIC_ALL,
			
		];
}




pub mod separators {
	
	use super::*;
	
	
	
	
	macro_rules! define_separator {
		( $_visibility : vis $_pattern : ident, $_variant : ident, $_text : expr, infix, ( $_length : tt : $_each : tt ) ) => {
			macros::__count_call_with! ( [ $_length : $_each ] => define_separator! ($_visibility $_pattern, $_variant, $_text, infix, ));
		};
		( $_visibility : vis $_pattern : ident, $_variant : ident, $_text : expr, infix, [ $( $_infix_each : literal, )* ] ) => {
			::paste::paste! {
				
				static [< _ $_pattern _TEXT >] : &Text = & Text::$_variant ($_text);
				
				$_visibility static [< $_pattern _MANDATORY_SEPARATOR >] : &Separator = & Separator::Mandatory (Rb::new_static ( [< _ $_pattern _TEXT >] ));
				$_visibility static [< $_pattern _OPTIONAL_SEPARATOR >] : &Separator = & Separator::Optional (Rb::new_static ( [< _ $_pattern _TEXT >] ));
				
				$_visibility static [< $_pattern _MANDATORY_INFIX_PATTERN >] : &SeparatorPattern = & SeparatorPattern::Infix (Rb::new_static ( [< $_pattern _MANDATORY_SEPARATOR >] ));
				$_visibility static [< $_pattern _OPTIONAL_INFIX_PATTERN >] : &SeparatorPattern = & SeparatorPattern::Infix (Rb::new_static ( [< $_pattern _OPTIONAL_SEPARATOR >] ));
				
				$(
					$_visibility static [< $_pattern _MANDATORY_INFIX_EACH_ $_infix_each _PATTERN >] : &SeparatorPattern = & SeparatorPattern::InfixEach (Rb::new_static ( [< $_pattern _MANDATORY_SEPARATOR >] ), $_infix_each);
					$_visibility static [< $_pattern _OPTIONAL_INFIX_EACH_ $_infix_each _PATTERN >] : &SeparatorPattern = & SeparatorPattern::InfixEach (Rb::new_static ( [< $_pattern _OPTIONAL_SEPARATOR >] ), $_infix_each);
				)*
			}
		};
	}
	
	
	
	
	pub static NONE_PATTERN : &SeparatorPattern = & SeparatorPattern::None;
	
	
	define_separator! (pub SPACE, Char, ' ', infix, ( 16 : 1 ));
	define_separator! (pub DOT, Char, '.', infix, ( 16 : 1 ));
	define_separator! (pub HYPHEN, Char, '-', infix, ( 16 : 1 ));
	
	define_separator! (pub SPACE_HYPHEN_SPACE, Str, " - ", infix, ( 16 : 1 ));
}




pub fn all_token_patterns () -> RbList<(String, Rb<TokenPattern>)> {
	
	let mut _collector = Vec::with_capacity (1024);
	
	for _patterns in tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Named (_identifier, _) =>
					_collector.push ((String::from (*_identifier), _pattern.clone ())),
				_ =>
					panic! (0xcb0098dd),
			}
		}
	}
	
	RbList::from_vec (_collector)
}




pub mod consts {
	
	pub mod ascii {
		// NOTE:  python -c 'for c in range (32, 127) : print ("pub const C%02X : char = %r;" % (c, chr(c)))'
		include! ("./patterns_consts_ascii.in");
	}
	
	pub mod mnemonic {
		// NOTE:  => https://github.com/singpolyma/mnemonicode
		// NOTE:  => https://github.com/mbrubeck/rust-mnemonic
		include! ("./patterns_consts_mnemonic.in");
	}
}




pub(crate) mod macros {
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_list {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( %d, %d )" % (n, e) + "=> { [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] };")\nprint ("}")' >| ./sources/patterns_count_list.in
	include! ("./patterns_count_list.in");
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_call_each {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> {\\n" + "\\n".join (["\t$c! ( $($p)* %d );" % c for c in range (0, n + 1, e) if c != 0]) + "\\n};")\nprint ("}")' >| ./sources/patterns_count_call_each.in
	include! ("./patterns_count_call_each.in");
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_call_with {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> { $c! ( $($p)* [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] ); };")\nprint ("}")' >| ./sources/patterns_count_call_with.in
	include! ("./patterns_count_call_with.in");
	
	pub(crate) use __count_list;
	pub(crate) use __count_call_each;
	pub(crate) use __count_call_with;
}

