#!/usr/bin/env python3

import ctypes
from ctypes import c_int32, c_char_p
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

lib.how_many_characters.argtypes = (c_char_p,)
lib.how_many_characters.restype = c_int32

result = lib.how_many_characters('rust')
print(result)
assert 4 == result

class ApiTest(unittest.TestCase):

    def test_null_text(self):
        result = lib.how_many_characters(None)

        self.assertEqual(-1, result)

    def test_empty_text(self):
        result = lib.how_many_characters('')

        self.assertEqual(0, result)

    def test_text(self):
        result = lib.how_many_characters('rust')

        self.assertEqual(4, result)

if __name__ == '__main__':
    unittest.main()
