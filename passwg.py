

import json
import math
import os
import random
import sys


def load () :
	with open (os.path.join (os.path.dirname (__file__), "passwg.json")) as stream :
		return json.load (stream)

def generate (schema, pattern_id) :
	pattern = schema["patterns"][pattern_id]
	generator = random.SystemRandom ()
	password = []
	for group in pattern :
		if group == '-' :
			password.append (os.environ.get ('PASSWG_GROUP_BREAK', schema["breaks"][pattern_id]))
		else :
			group = group_generator (schema["glyphs"][group])
			glyph = generator.choice (group)
			password.append (glyph)
	password = ''.join ([str (password) for password in password])
	return password

def group_generator (group) :
	if isinstance (group, basestring) :
		return group
	elif isinstance (group, list) :
		return group
	elif isinstance (group, dict) :
		if group["type"] == "range" :
			return range (group["min"], group["max"] + 1)
	else :
		raise Exception ()


if __name__ == '__main__' :
	
	if len (sys.argv) != 2 :
		command = 'help'
	else :
		command = sys.argv[1]
	
	schema = load ()
	
	if command == 'patterns' :
		identifiers = schema["patterns"].keys ()
		identifiers.sort ()
		for identifier in identifiers :
			pattern = schema["patterns"][identifier]
			length = 0
			strength = 1.0
			for group in pattern :
				if group != '-' :
					group = group_generator (schema["glyphs"][group])
					length += 1
					strength *= len (group)
			strength = math.log (strength, 2)
			if strength >= 56 :
				year = math.floor (1982 + 1.5 * (math.floor (strength) - 56))
			else :
				year = -1
			pattern = ' '.join (pattern)
			if len (pattern) > 40 :
				pattern = pattern[:36] + ' ...'
			example = generate (schema, identifier)
			#if len (example) > 40 :
			#	example = example[:36] + ' ...'
			print "| %-16s | %6.1f bits | %5d len | %-40s | %-40s" % (identifier, strength, length, pattern, example)
		
	elif command == 'glyphs' :
		identifiers = schema["glyphs"].keys ()
		identifiers.sort ()
		for identifier in identifiers :
			group = group_generator (schema["glyphs"][identifier])
			length = len (group)
			strength = length
			strength = math.log (strength, 2)
			example = str (group)
			if len (example) > 40 :
				example = example[:36] + ' ...'
			print "| %-16s | %4.1f bits | %5d len | %s" % (identifier, strength, length, example)
		
	elif command in schema["patterns"] :
		for i in xrange (16) :
			password = generate (schema, command)
			print password
		
	else :
		print >> sys.stderr, 'passwg [ patterns | glyphs | <pattern> ]'
		sys.exit (1)
	
	sys.exit (0)
