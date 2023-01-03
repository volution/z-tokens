

import json
import math
import os
import random
import sys


generator = random.SystemRandom ()


def generate_token (schema, pattern_identifier, pattern_break) :
	while pattern_identifier in schema["aliases"] :
		pattern_identifier = schema["aliases"][pattern_identifier]
	pattern_groups = generate_groups (schema, pattern_identifier)
	pattern_break = generate_break (schema, pattern_identifier, pattern_break)
	token_glyphs = []
	for pattern_group in pattern_groups :
		if pattern_group == "-" :
			token_glyphs.append (pattern_break)
		elif pattern_group.startswith (":") :
			token_glyphs.append (pattern_group[1:])
		elif pattern_group.startswith ("!") :
			for token in generate_token (schema, pattern_group[1:], pattern_break) :
				token_glyphs.append (token)
		else :
			token_group = generate_group (schema, pattern_identifier, pattern_group)
			token_glyph = generate_glyph (token_group)
			token_glyphs.append (token_glyph)
	token = "".join (token_glyphs)
	return token

def generate_tokens (schema, pattern_identifier, pattern_break, count) :
	return [generate_token (schema, pattern_identifier, pattern_break) for index in xrange (count)]


def generate_groups (schema, pattern_identifier) :
	return schema["patterns"][pattern_identifier]


def generate_group (schema, pattern_identifier, pattern_group) :
	group = schema["glyphs"][pattern_group]
	if isinstance (group, basestring) :
		return group
	elif isinstance (group, list) :
		return group
	elif isinstance (group, dict) :
		if group["type"] == "range" :
			return xrange (group["min"], group["max"] + 1)
	else :
		raise Exception ()


def generate_break (schema, pattern_identifier, pattern_break) :
	pattern_break = schema["breaks"].get (pattern_identifier + "*", pattern_break)
	if pattern_break is True :
		return schema["breaks"][pattern_identifier]
	elif pattern_break is False :
		return ""
	elif isinstance (pattern_break, basestring) :
		return pattern_break
	else :
		raise Exception


def generate_glyph (group) :
	return str (generator.choice (group))


def display_tokens (stream, tokens, separator) :
	for token in tokens :
		display_token (stream, token, separator)

def display_token (stream, token, separator) :
	stream.write (token)
	if separator is not None :
		stream.write (separator)


def display_patterns (stream) :
	trim_pattern = True
	trim_pattern_length = 20
	trim_token = True
	trim_token_length = 40
	show_pattern = False
	identifiers = schema["patterns"].keys ()
	identifiers.sort ()
	for identifier in identifiers :
		pattern = schema["patterns"][identifier]
		length = 0
		strength = 1.0
		for group in pattern :
			if group == "-" :
				pass
			elif group.startswith (":") :
				pass
			elif group.startswith ("!") :
				# FIXME:  Should compute recursive pattern strength!
				pass
			else :
				group = generate_group (schema, identifier, group)
				length += 1
				strength *= len (group)
		strength = math.log (strength, 2)
		if strength >= 56 :
			year = math.floor (1982 + 1.5 * (math.floor (strength) - 56))
		else :
			year = -1
		pattern = " ".join (pattern)
		if trim_pattern and len (pattern) > trim_pattern_length :
			pattern = pattern[:(trim_pattern_length - 4)] + " ..."
		token = generate_token (schema, identifier, True)
		if trim_token and len (token) > trim_token_length :
			token = token[:(trim_token_length - 4)] + " ..."
		if show_pattern :
			line = "| %-16s | %3d (%5.1f bits) | %-20s | %s\n" % (identifier, length, strength, pattern, token)
		else :
			line = "| %-16s | %3d (%5.1f bits) | %s\n" % (identifier, length, strength, token)
		stream.write (line)


def display_glyphs (stream) :
	trim_example = True
	trim_example_length = 40
	identifiers = schema["glyphs"].keys ()
	identifiers.sort ()
	for identifier in identifiers :
		group = generate_group (schema, None, identifier)
		length = len (group)
		strength = length
		strength = math.log (strength, 2)
		example = str (group)
		if trim_example and len (example) > trim_example_length :
			example = example[:(trim_example_length - 4)] + " ..."
		line = "| %-16s | %5d (%5.1f bits) | %s\n" % (identifier, length, strength, example)
		stream.write (line)


def display_help (stream) :
	print >> stream, r"""`tokeng` (Token Generator) [https://github.com/cipriancraciun/token-generator]

tokeng generate [ <pattern> [ <count> [ <group> [ <separator> ]]]]

  Generates a couple of tokens (given by `<count>`), where:
  * characters are grouped (for easy reading) if `<group>` is `t` (the default);
  * characters are not grouped (ready for copy-paste) if `<group>` is `f`;
  * tokens are separated by `<separator>` (new-line by default);

  tokeng generate p-aa-4 10
  tokeng generate p-aa-4 10 f
  tokeng generate p-aa-4 10 t
  tokeng generate p-aa-4 10 f '  '

tokeng one [ <pattern> ]

  Generates a single token of the given pattern, ready to be copy-pasted.

  tokeng one x-128

tokeng one-n [ <pattern> ]

  Generates a single token of the given pattern, without printing a new-line at the end.

  This invocation could be used as part of a shell pipeline like:
      tokeng one-n x-128 | xclip -i

tokeng patterns

  Displays the list of available patterns, the columns being:
  * the identifier (to be used for the `generate` or `one` commands);
  * the character length (without character grouping);
  * the token "security" bits (i.e. the number of possible combinations is 2 raised at this number);
  * an example of the resulting token (with character grouping);

tokeng glyphs

  Displays the list of available glyphs.
"""


def load_schema () :
	with open (os.path.join (os.path.dirname (__file__), "tokeng.json")) as stream :
		return json.load (stream)


if __name__ == "__main__" :
	
	default_pattern_identifier = "p-aa-4"
	default_pattern_count = 4
	default_pattern_break = True
	default_token_separator = "\n"
	
	if len (sys.argv) == 1 :
		command = "one"
		arguments = []
	elif len (sys.argv) >= 1 :
		command = sys.argv[1]
		arguments = sys.argv[2:]
	else :
		raise Exception ()
	
	schema = load_schema ()
	
	if command == "generate" and len (arguments) <= 4 :
		pattern_identifier = arguments[0] if len (arguments) >= 1 else default_pattern_identifier
		pattern_count = int (arguments[1]) if len (arguments) >= 2 else default_pattern_count
		pattern_break = arguments[2] if len (arguments) >= 3 else default_pattern_break
		if pattern_break == 't' :
			pattern_break = True
		elif pattern_break == 'f' :
			pattern_break = False
		elif pattern_break == 's' :
			pattern_break = ' '
		elif pattern_break == '' :
			pattern_break = False
		else :
			pass
		token_separator = arguments[3] if len (arguments) >= 4 else default_token_separator
		tokens = generate_tokens (schema, pattern_identifier, pattern_break, pattern_count)
		display_tokens (sys.stdout, tokens, token_separator)
		
	elif command == "one" and len (arguments) <= 1 :
		pattern_identifier = arguments[0] if len (arguments) >= 1 else default_pattern_identifier
		token = generate_token (schema, pattern_identifier, False)
		display_token (sys.stdout, token, default_token_separator)
		
	elif command == "one-n" and len (arguments) <= 1 :
		pattern_identifier = arguments[0] if len (arguments) >= 1 else default_pattern_identifier
		token = generate_token (schema, pattern_identifier, False)
		display_token (sys.stdout, token, None)
		
	elif (command in schema["patterns"] or command in schema["aliases"]) and len (arguments) == 0 :
		tokens = generate_token (schema, command, False)
		display_token (sys.stdout, tokens, default_token_separator)
		
	elif command == "patterns" and len (arguments) == 0 :
		display_patterns (sys.stdout)
		
	elif command == "glyphs" and len (arguments) == 0 :
		display_glyphs (sys.stdout)
		
	elif command == "help" :
		display_help (sys.stderr)
		
	else :
		display_help (sys.stderr)
		sys.exit (1)
	
	sys.exit (0)
