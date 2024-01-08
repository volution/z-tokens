

# `zt-exchange` design document




----




## Use cases

...




----




## Threat model

...




----




## High-level description

...




----




## Concepts




### Sender

A Curve25519 private / public key pair
that authenticates the sender to the recipient.
For encryption the sender private key must be known.
For decryption the sender public key must be known.

Using a sender key pair is optional.
However, if a recipient key pair is used,
then the sender key pair is mandatory.

Unlike PGP or Age, the sender public key (its identity)
is not stored in the message,
thus must be known in advance through other means.

There is the option to use multiple senders,
and their number can be different than that of the recipients.
(Their order and duplication does not matter.)




### Recipient

Like in the case of the sender,
the recipient is a Curve25519 private / public key pair
that permits the decryption of the ciphertext.
For encryption the recipient public key must be known.
For decryption the recipient private key must be known.

Using a recipient key pair is optional.
However, if a sender key pair is used,
but no recipient key pair is presented,
then the sender key pair is used for both roles,
thus serving the practical role of
symmetric encryption.

Unlike PGP or Age, the recipient public key (its identity)
is not stored in the message,
thus must be either known in advance through other means.

There is the option to use multiple recipients,
and their number can be different than that of the senders.
(Their order and duplication does not matter.)




### Secret

This is an arbitrary blob of data
that plays the role of
a high entropy password,
which is the basis of
a symmetric encryption key.

Using a secret is optional.

This input should be used
for long-term and/or high-risk
security needs.

By design, the derivation of the
symmetric encryption key
should have moderate requirements
in terms of CPU and memory.
(In the current design it takes ~1 second.)

By default the tool generates
a random 128 bit token,
which the user has to keep safe through other means.

There is the option of using multiple secrets.
In terms of derivation requirements, by design
the time increases linearly
(without the possibility of parallelization).
(Their order and duplication does not matter.)

If the user chooses the this input,
the user is responsible for the entropy quality.




### PIN

This is an arbitrary blob of data
that plays the role of
a low entropy password,
which is the basis of
a symmetric encryption key.

Using a PIN is optional.

This input should be used
for short-term or low-risk
security needs
(like for example when a brute-force attack isn't a concern).

By design, the derivation of the
symmetric encryption key
should have low requirements
in terms of CPU and memory.
(In the current design it takes ~0.1 seconds.)

There is the option of using multiple PINs.
In terms of derivation requirements, by design
the time increases linearly
(without the possibility of parallelization).
(Their order and duplication does not matter.)




### Ballast

This is an arbitrary blob of data
that plays the role of
a high entropy password,
which is the basis of
a symmetric encryption key.

Using a ballast is optional.

This input should be used
for long-term and/or high-risk
security needs.

By design, the derivation of the
symmetric encryption key
should have high requirements
in terms of CPU and memory.
(In the current design it takes ~3 seconds.)

By default the tool generates
a random 128 bit token,
which the user has to keep safe through other means.

There is the option of using multiple ballasts.
In terms of derivation requirements, by design
the time increases linearly
and the memory logaritmically
(without the possibility of parallelization).
(Their order and duplication does not matter.)

If the user chooses the this input,
the user is responsible for the entropy quality.




### Oracles

These are external functions,
similar to hash functions,
that should play the role of
theoretical "random oracles",
namely deterministic functions
that return unguessable
high-entropy outputs.

Using an oracle is optional.

For the moment the only oracle implementation
is based on SSH agent signatures (with RSA or Ed25519).

In future anything that resembles
a PKCS-11 or FIDO token could be used.

There is the option of using multiple oracles,
and by design their execution chained linearly
(and can't be parallelized,
with one's output threaded as the next one's input)
(Their order and duplication does not matter.)




### Associated data

This is an arbitrary blob of data
that plays the role of aditional authenticated data.

Using associated data is optional.
There is the option to use multiple associated data,
however their order and duplication is important.

Unlike PGP or Age, this associated data
is not stored in the message,
thus must be either known in advance through other means.
(Most likely it would be hard-coded in the code
that calls this tool.)




----




## Cryptography




### Primitives

The current design uses the following well-known algorithms:

* ChaCha20 for symmetric encryption;
* Blake3 for key derivation and MAC;
* Curve25519 for private / public keys;
* Argon2 for password derivation;




##### `blake3_derive_key` and `blake3_keyed_hash`

~~~~
fn blake3_derive_key (
		_purpose : const_string,
		_fixed_inputs : Vec<[u8; 32]>,
		_variable_inputs : Vec<[u8]>,
	) -> [u8; 32]

fn blake3_keyed_hash (
		_purpose : const_string,
		_fixed_inputs : Vec<[u8; 32]>,
		_variable_inputs : Vec<[u8]>,
	) -> [u8; 32]
~~~~

* the purpose is used for domain separation
  and should constant (hard-coded in the code);
* the number of fixed or variable inputs
  must be constant for the same purpose
  (thus hard-coded in the code);
* either the fixed or variable inputs are optional;




##### `blake3_derive_key_join`

~~~~
fn blake3_derive_key_join (
		_purpose : const_string,
		_fixed_inputs : Vec<[u8; 32]>,
	) -> [u8; 32]
~~~~

* similar to the previous two functions;
* the number of inputs can be variable for the same purpose;




##### `argon_derive`

~~~~
fn argon_derive (
		_secret : [u8; 32],
		_salt : [u8; 32],
		_m_cost : u32,
		_t_cost : u32,
	) -> [u8; 32]
~~~~




----




### Inputs

Both the encryption and decryption
take the same kinds of inputs
as described in the concepts section.




#### X25519 private and public keys

Sender (for encryption)
or recipient (for decryption)
private keys,
sorted by their public key big-endian binary encoding:
~~~~
let _private_keys : Vec<x25519::PrivateKey> = ...;
_private_keys.sort (|_key| _key.public_key().as_big_endian_bytes_vec());
~~~~

Recipient (for encryption)
or sender (for decryption)
public keys,
sorted by their big-endian binary encoding:
~~~~
let _public_keys : Vec<x25519::PublicKey> = ...;
_public_keys.sort (|_key| _key.as_big_endian_bytes_vec());
~~~~

> ###### Question
> Should the conversion to public keys,
> their encoding, and their sorting
> de done in constant time?




#### Secrets, PINs and ballasts

> ###### Note
> All of the following snippets
> are almost identical,
> with the exception of purpose constant string
> and not sorting / deduplicating associated data.

> ###### Note
> In the following snippets,
> each input is hashed (so we can work with fixed length data),
> and then a compound hash of all the inputs (of the same type) is computed.
> This compound hash is only used as an intermediary step,
> so that changing one input forces the re-derivation of everything else,
> and further derivation is executed over each individual input hash.

~~~~
let _secret_inputs : Vec<[u8]> = ...;
let _pin_inputs : Vec<[u8]> = ...;
let _ballast_inputs : Vec<[u8]> = ...;
let _associated_inputs : Vec<[u8]> = ...;
~~~~

~~~~
let _secret_hashes : Vec<[u8; 32]>
	= _secret_inputs.map (|_secret_input|
		blake3_derive_key (
			purpose : SECRET_HASH_PURPOSE,
			fixed_inputs : [],
			variable_inputs : [_secret_input],
		));

_secret_hashes.secure_sort ();
_secret_hashes.secure_deduplicate ();

_secrets_hash : [u8; 32] = blake3_derive_key_join (
		purpose : SECRET_HASH_PURPOSE,
		fixed_inputs : _secret_hashes,
	);
~~~~

~~~~
let _pin_hashes : Vec<[u8; 32]>
	= _pin_inputs.map (|_pin_input|
		blake3_derive_key (
			purpose : PIN_HASH_PURPOSE,
			fixed_inputs : [],
			variable_inputs : [_pin_input],
		));

_pin_hashes.secure_sort ();
_pin_hashes.secure_deduplicate ();

_pin_hash : [u8; 32] = blake3_derive_key_join (
		purpose : PIN_HASH_PURPOSE,
		fixed_inputs : _pin_hashes,
	);
~~~~

~~~~
let _ballast_hashes : Vec<[u8; 32]>
	= _ballast_inputs.map (|_ballast_input|
		blake3_derive_key (
			purpose : BALLAST_HASH_PURPOSE,
			fixed_inputs : [],
			variable_inputs : [_ballast_input],
		));

_ballast_hashes.secure_sort ();
_ballast_hashes.secure_deduplicate ();

_ballast_hash : [u8; 32] = blake3_derive_key_join (
		purpose : BALLAST_HASH_PURPOSE,
		fixed_inputs : _ballast_hashes,
	);
~~~~

~~~~
let _associated_hashes : Vec<[u8; 32]>
	= _associated_inputs.map (|_associated_input|
		blake3_derive_key (
			purpose : ASSOCIATED_HASH_PURPOSE,
			fixed_inputs : [],
			variable_inputs : [_associated_input],
		));

//  NOTE:  No sorting or deduplication!

_associated_hash : [u8; 32] = blake3_derive_key_join (
		purpose : ASSOCIATED_HASH_PURPOSE,
		fixed_inputs : _associated_hashes,
	);
~~~~

> ###### Question
> How should we implement secure sorting and deduplication?
> Just using constant comparisons shouldn't be enough.
> Perhaps something like selection sort?

> ###### Question
> We re-use the same purpose twice:
> once for computing an individual input hash;
> and a second time when computing the compounded hash for all inputs (of the same type).
> Is this a weakening of the scheme?
> I don't believe so because the scheme is "canonical" (i.e. you always do the same steps).




----




### Shared secret derivation




#### Curve25519 DHE between senders and recipients

...




#### Oracles entanglement

...




#### Argon2 derivation for secrets, PINs and ballasts

...




----




## Implementation

See <https://github.com/volution/z-tokens/blob/development/sources/exchange/crypto.rs>
for the current Rust implementation.

See <https://github.com/volution/z-tokens/blob/development/sources/exchange-tool-go/lib/decrypt.go>
for (an earlier scheme) implementation in Go that only supports decryption.
(And should be used to cross-check the Rust implementation.)

