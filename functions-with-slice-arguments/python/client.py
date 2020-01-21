#!/usr/bin/env python3

import ctypes
from ctypes import c_int32, c_size_t, c_uint32, POINTER
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

lib.sum_of_even.argtypes = (POINTER(c_uint32), c_size_t)
lib.sum_of_even.restype = c_int32

def sum_of_even(numbers):
    # Create a _ctypes.PyCArrayType object.
    buffer_type = c_uint32 * len(numbers)
    # *args is used to send a non-keyworded variable length argument
    # list to the function.
    buffer = buffer_type(*numbers)

    return lib.sum_of_even(buffer, len(numbers))

result = sum_of_even([0, 1, 2, 3, 4, 5, 6])
print(result)
assert 12 == result

class ApiTest(unittest.TestCase):

    def test_null_numbers(self):
        result = lib.sum_of_even(None, 0)

        self.assertEqual(-1, result)

    def test_empty_numbers(self):
        result = sum_of_even([])

        self.assertEqual(0, result)

    def test_numbers_as_zero(self):
        result = sum_of_even([0])

        self.assertEqual(0, result)

    def test_numbers_as_one(self):
        result = sum_of_even([1])

        self.assertEqual(0, result)

    def test_numbers_as_two(self):
        result = sum_of_even([2])

        self.assertEqual(2, result)

    def test_numbers_as_several_elements(self):
        result = sum_of_even([0, 1, 2, 3, 4, 5, 6])

        self.assertEqual(12, result)

if __name__ == '__main__':
    unittest.main()
