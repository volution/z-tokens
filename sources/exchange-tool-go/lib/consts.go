

package exchange








const PACKET_PADDING_SIZE = 256
const PACKET_MAC_SIZE = 32
const PACKET_SALT_SIZE = 32




const ARGON_SECRET_M_COST = 512 * 1024
const ARGON_SECRET_T_COST = 8

const ARGON_PIN_M_COST = 32 * 1024
const ARGON_PIN_T_COST = 4

const ARGON_ANY_P_COST = 1




const BECH32_RECIPIENT_PRIVATE_PREFIX = "ztxrk"
const BECH32_RECIPIENT_PUBLIC_PREFIX = "ztxrp"

const BECH32_SENDER_PRIVATE_PREFIX = "ztxsk"
const BECH32_SENDER_PUBLIC_PREFIX = "ztxsp"

const BECH32_SECRET_PREFIX = "ztxcs"

const BECH32_SSH_WRAP_HANDLE_PREFIX = "ztxws"




var CRYPTO_DHE_KEY_CONTEXT = cryptographic_context ("encryption", "dhe_key")
var CRYPTO_NAIVE_KEY_CONTEXT = cryptographic_context ("encryption", "naive_key")
var CRYPTO_AONT_KEY_CONTEXT = cryptographic_context ("encryption", "aont_key")

var CRYPTO_PACKET_SALT_CONTEXT = cryptographic_context ("encryption", "packet_salt")
var CRYPTO_PACKET_KEY_CONTEXT = cryptographic_context ("encryption", "packet_key")
var CRYPTO_ENCRYPTION_KEY_CONTEXT = cryptographic_context ("encryption", "encryption_key")
var CRYPTO_AUTHENTICATION_KEY_CONTEXT = cryptographic_context ("encryption", "authentication_key")

var CRYPTO_SECRET_HASH_CONTEXT = cryptographic_context ("encryption", "secret_hash")
var CRYPTO_SECRET_SALT_CONTEXT = cryptographic_context ("encryption", "secret_salt")
var CRYPTO_SECRET_KEY_CONTEXT = cryptographic_context ("encryption", "secret_key")

var CRYPTO_PIN_HASH_CONTEXT = cryptographic_context ("encryption", "pin_hash")
var CRYPTO_PIN_SALT_CONTEXT = cryptographic_context ("encryption", "pin_salt")
var CRYPTO_PIN_KEY_CONTEXT = cryptographic_context ("encryption", "pin_key")

var CRYPTO_SSH_WRAP_INPUT_CONTEXT = cryptographic_context ("encryption", "ssh_wrap_input")
var CRYPTO_SSH_WRAP_OUTPUT_CONTEXT = cryptographic_context ("encryption", "ssh_wrap_output")

var CRYPTO_DERIVE_SALT_CONTEXT = cryptographic_context ("password", "salt")
var CRYPTO_DERIVE_OUTPUT_CONTEXT = cryptographic_context ("password", "output")

var SSH_WRAP_KEY_HASH_CONTEXT = cryptographic_context ("ssh_wrap", "key_hash")
var SSH_WRAP_INPUT_HASH_CONTEXT = cryptographic_context ("ssh_wrap", "input_hash")
var SSH_WRAP_OUTPUT_HASH_CONTEXT = cryptographic_context ("ssh_wrap", "output_hash")








func cryptographic_context (_namespace string, _purpose string) (string) {
	return "z-tokens / exchange / " + _namespace + " / " + _purpose + " / " + "(2023a)"
}


