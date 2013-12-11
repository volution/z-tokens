
import math
import os
import random
import sys


glyphs = {
			'v' : 'aeiou',
			'V' : 'AEIOU',
			'vV' : 'aeiouAEIOU',
			'c' : 'bcdfghjklmnpqrstvwxyz',
			'C' : 'BCDFGHJKLMNPQRSTVWXYZ',
			'cC' : 'bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ',
			'd' : '0123456789',
			'hd' : '0123456789abcdef',
			's' : '`~!@#$%^&*()[{]}\'\",<.>/?=+-_\\|;:',
			'l' : 'aeioubcdfghjklmnpqrstvwxyz',
			'L' : 'AEIOUBCDFGHJKLMNPQRSTVWXYZ',
			'lL' : 'aeioubcdfghjklmnpqrstvwxyzAEIOUBCDFGHJKLMNPQRSTVWXYZ',
			'ld' : 'aeioubcdfghjklmnpqrstvwxyz0123456789',
			'Ld' : 'AEIOUBCDFGHJKLMNPQRSTVWXYZ0123456789',
			'lLd' : 'aeioubcdfghjklmnpqrstvwxyzAEIOUBCDFGHJKLMNPQRSTVWXYZ0123456789',
			'*' : 'aeioubcdfghjklmnpqrstvwxyzAEIOUBCDFGHJKLMNPQRSTVWXYZ0123456789`~!@#$%^&*()[{]}\'\",<.>/?=+-_\\|;:',
			'#' : '123456',
		}

patterns = {
			'p-aa-1' : [
					'c', 'v', 'c', 'v'],
			'p-aa-2' : [
					'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			'p-aa-3' : [
					'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			'p-aa-4' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			'p-aa-5' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-',
				'c', 'v', 'c', 'v'],
			'p-aa-6' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-',
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			'p-aa-7' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-',
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			'p-aa-8' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-',
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v'],
			
			'p-ab-1' : [
				'cC', 'vV', 'cC', 'vV'],
			'p-ab-2' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			'p-ab-3' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			'p-ab-4' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			'p-ab-5' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-',
				'cC', 'vV', 'cC', 'vV'],
			'p-ab-6' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-',
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			'p-ab-6' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-',
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			'p-ab-8' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-',
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV'],
			
			'p-ba-3+1' : [
				'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'c', 'v', 'c', 'v', '-', 'd', 'd', 'd', 'd'],
			'p-bb-3+1' : [
				'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 'd', 'd'],
			
			'p-ca-3*2' : [
				'c', 'v', 'c', 'v', '-', 'd', 'd', 's', '-', 'c', 'v', 'c', 'v', '-', 'd', 'd', 's', '-',
				'c', 'v', 'c', 'v', '-', 'd', 'd', 's'],
			'p-ca-4*2' : [
				'c', 'v', 'c', 'v', '-', 'd', 'd', 's', '-', 'c', 'v', 'c', 'v', '-', 'd', 'd', 's', '-',
				'c', 'v', 'c', 'v', '-', 'd', 'd', 's', '-', 'c', 'v', 'c', 'v', '-', 'd', 'd', 's'],
			'p-cb-3*2' : [
				'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's', '-', 'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's', '-',
				'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's'],
			'p-cb-4*2' : [
				'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's', '-', 'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's', '-',
				'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's', '-', 'cC', 'vV', 'cC', 'vV', '-', 'd', 'd', 's'],
			
			'n-04' : [
				'd', 'd', 'd', 'd'],
			'n-08' : [
				'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd'],
			'n-12' : [
				'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd'],
			'n-16' : [
				'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd', '-', 'd', 'd', 'd', 'd'],
			
			'r-04' : [
				'*', '*', '*', '*'],
			'r-08' : [
				'*', '*', '*', '*', '*', '*', '*', '*'],
			'r-16' : [
				'*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*'],
			'r-32' : [
				'*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*',
				'*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*'],
			
			'uuid' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', '-', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'mac' : [
				'hd', 'hd', '-', 'hd', 'hd', '-', 'hd', 'hd', '-',
				'hd', 'hd', '-', 'hd', 'hd', '-', 'hd', 'hd'],
			
			'x-016' : [
				'hd', 'hd', 'hd', 'hd'],
			
			'x-032' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-064' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-096' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-128' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-160' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-192' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-224' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-256' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-384' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			'x-512' : [
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', '-',
				'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd', 'hd'],
			
			#'d-1' : [
			#	'#', '#', '#', '#', '#'],
			#'d-2' : [
			#	'#', '#', '#', '#', '#', '-',
			#	'#', '#', '#', '#', '#'],
		}

breaks = {
			'p-aa-1' : '-',
			'p-aa-2' : '-',
			'p-aa-3' : '-',
			'p-aa-4' : '-',
			'p-aa-5' : '-',
			'p-aa-6' : '-',
			'p-aa-7' : '-',
			'p-aa-8' : '-',
			
			'p-ab-1' : '-',
			'p-ab-2' : '-',
			'p-ab-3' : '-',
			'p-ab-4' : '-',
			'p-ab-5' : '-',
			'p-ab-6' : '-',
			'p-ab-6' : '-',
			'p-ab-8' : '-',
			
			'p-ba-3+1' : '-',
			'p-bb-3+1' : '-',
			
			'p-ca-3*2' : ' ',
			'p-ca-4*2' : ' ',
			'p-cb-3*2' : ' ',
			'p-cb-4*2' : ' ',
			
			'n-04' : '-',
			'n-08' : '-',
			'n-12' : '-',
			'n-16' : '-',
			
			'r-04' : ' ',
			'r-08' : ' ',
			'r-16' : ' ',
			'r-32' : ' ',
			
			'uuid' : '-',
			
			'mac' : ':',
			
			'x-016' : '',
			'x-032' : '',
			'x-064' : '',
			'x-096' : '',
			'x-128' : '',
			'x-160' : '',
			'x-192' : '',
			'x-224' : '',
			'x-256' : '',
			'x-384' : '',
			'x-512' : '',
}


def generate (pattern_id) :
	pattern = patterns[pattern_id]
	generator = random.SystemRandom ()
	password = []
	for group in pattern :
		if group == '-' :
			password.append (os.environ.get ('PASSWG_GROUP_BREAK', breaks[pattern_id]))
		else :
			group = glyphs[group]
			glyph = generator.choice (group)
			password.append (glyph)
	password = ''.join (password)
	return password


if __name__ == '__main__' :
	
	if len (sys.argv) != 2 :
		command = 'help'
	else :
		command = sys.argv[1]
	
	if command == 'patterns' :
		identifiers = patterns.keys ()
		identifiers.sort ()
		for identifier in identifiers :
			pattern = patterns[identifier]
			length = 0
			strength = 1.0
			for group in pattern :
				if group != '-' :
					group = glyphs[group]
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
			example = generate (identifier)
			if len (example) > 40 :
				example = example[:36] + ' ...'
			print "%-8s | %6.1f bits (%4d) | %3d len | %-40s | %-40s |" % (identifier, strength, year, length, pattern, example)
		
	elif command == 'glyphs' :
		identifiers = glyphs.keys ()
		identifiers.sort ()
		for identifier in identifiers :
			group = glyphs[identifier]
			length = len (group)
			strength = length
			strength = math.log (strength, 2)
			print "%4s | %2.1f bits | %2d len | %s" % (identifier, strength, length, group)
		
	elif command in patterns :
		for i in xrange (16) :
			password = generate (command)
			print password
		
	else :
		print >> sys.stderr, 'passwg [ patterns | glyphs | <pattern> ]'
		sys.exit (1)
	
	sys.exit (0)
