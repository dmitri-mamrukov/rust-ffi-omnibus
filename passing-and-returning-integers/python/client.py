#!/usr/bin/env python3

import ctypes
from ctypes import c_uint32, Structure
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

class Tuple(Structure):
    _fields_ = [('sum', c_uint32),
                ('err_code', c_uint32)]

    def __str__(self):
        return '(sum: {}, err_code: {})'.format(self.sum, self.err_code)

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

lib.addition.argtypes = (c_uint32, c_uint32)
lib.addition.restype = Tuple

result = lib.addition(1, 2)
print(result)
assert '(sum: 3, err_code: 0)' == str(result)

class ApiTest(unittest.TestCase):

    UINT_MAX = 4294967295

    def test_zero_and_zero(self):
        result = lib.addition(0, 0)

        self.assertEqual('(sum: 0, err_code: 0)', str(result))

    def test_zero_and_positive(self):
        result = lib.addition(0, 1)

        self.assertEqual('(sum: 1, err_code: 0)', str(result))

    def test_zero_and_max(self):
        result = lib.addition(0, ApiTest.UINT_MAX)

        self.assertEqual('(sum: 4294967295, err_code: 0)', str(result))

    def test_positive_and_zero(self):
        result = lib.addition(1, 0)

        self.assertEqual('(sum: 1, err_code: 0)', str(result))

    def test_positive_and_positive(self):
        result = lib.addition(1, 1)

        self.assertEqual('(sum: 2, err_code: 0)', str(result))

    def test_one_and_max_minus_one(self):
        result = lib.addition(1, ApiTest.UINT_MAX - 1)

        self.assertEqual('(sum: 4294967295, err_code: 0)', str(result))

    def test_one_and_max(self):
        result = lib.addition(1, ApiTest.UINT_MAX)

        self.assertEqual('(sum: 0, err_code: 1)', str(result))

if __name__ == '__main__':
    unittest.main()
