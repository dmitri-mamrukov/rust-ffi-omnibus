#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import ctypes
from ctypes import c_void_p, c_uint8
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

lib.generate_theme_song.argtypes = (c_uint8, )
lib.generate_theme_song.restype = c_void_p

lib.free_theme_song.argtypes = (c_void_p, )

def generateThemeSong(count):
    song_ptr = lib.generate_theme_song(count)
    try:
        return ctypes.cast(song_ptr, ctypes.c_char_p).value.decode('utf-8')
    finally:
        lib.free_theme_song(song_ptr)

result = generateThemeSong(5)
print(type(result))
print(result)
assert u'💣 na na na na na Batman! 💣' == result

class ApiTest(unittest.TestCase):

    def test_length_as_zero(self):
        result = generateThemeSong(0)

        self.assertEqual(u'💣 Batman! 💣', result)

    def test_length_as_one(self):
        result = generateThemeSong(1)

        self.assertEqual(u'💣 na Batman! 💣', result)

    def test_length_as_two(self):
        result = generateThemeSong(2)

        self.assertEqual(u'💣 na na Batman! 💣', result)

if __name__ == '__main__':
    unittest.main()
