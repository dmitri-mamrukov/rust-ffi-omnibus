#!/usr/bin/env python3

import ctypes
from ctypes import c_uint32, Structure
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

class Tuple(Structure):
    _fields_ = [('x', c_uint32),
                ('y', c_uint32)]

    def __str__(self):
        return '(x: {}, y: {})'.format(self.x, self.y)

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

lib.flip_things_around.argtypes = (Tuple,)
lib.flip_things_around.restype = Tuple

result = lib.flip_things_around(Tuple(1, 2))
print(result)
assert 3 == result.x
assert 0 == result.y

class ApiTest(unittest.TestCase):

    UINT_MAX = 4294967295

    def test_zero_and_zero(self):
        result = lib.flip_things_around(Tuple(0, 0))

        self.assertEqual(1, result.x)
        self.assertEqual(ApiTest.UINT_MAX, result.y)

    def test_zero_and_two(self):
        result = lib.flip_things_around(Tuple(0, 2))

        self.assertEqual(3, result.x)
        self.assertEqual(ApiTest.UINT_MAX, result.y)

    def test_zero_and_max(self):
        result = lib.flip_things_around(Tuple(0, ApiTest.UINT_MAX))

        self.assertEqual(0, result.x)
        self.assertEqual(ApiTest.UINT_MAX, result.y)

    def test_one_and_zero(self):
        result = lib.flip_things_around(Tuple(1, ApiTest.UINT_MAX))

        self.assertEqual(0, result.x)
        self.assertEqual(0, result.y)

    def test_one_and_two(self):
        result = lib.flip_things_around(Tuple(1, 2))

        self.assertEqual(3, result.x)
        self.assertEqual(0, result.y)

    def test_one_and_max(self):
        result = lib.flip_things_around(Tuple(1, ApiTest.UINT_MAX))

        self.assertEqual(0, result.x)
        self.assertEqual(0, result.y)

    def test_max_and_zero(self):
        result = lib.flip_things_around(Tuple(ApiTest.UINT_MAX, 0))

        self.assertEqual(1, result.x)
        self.assertEqual(ApiTest.UINT_MAX - 1, result.y)

    def test_max_and_two(self):
        result = lib.flip_things_around(Tuple(ApiTest.UINT_MAX, 2))

        self.assertEqual(3, result.x)
        self.assertEqual(ApiTest.UINT_MAX - 1, result.y)

    def test_max_and_max(self):
        result = lib.flip_things_around(Tuple(ApiTest.UINT_MAX, ApiTest.UINT_MAX))

        self.assertEqual(0, result.x)
        self.assertEqual(ApiTest.UINT_MAX - 1, result.y)

if __name__ == '__main__':
    unittest.main()
