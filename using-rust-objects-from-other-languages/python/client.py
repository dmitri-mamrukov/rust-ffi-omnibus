#!/usr/bin/env python3

import ctypes
from ctypes import c_char_p, c_uint32, POINTER, Structure
import sys
import unittest

LIB_PREFIXES = {'win32': ''}
LIB_EXTENSIONS = {'darwin': '.dylib', 'win32': '.dll'}

prefix = LIB_PREFIXES.get(sys.platform, 'lib')
extension = LIB_EXTENSIONS.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + 'api' + extension)

class ZipCodeDatabaseStruct(Structure):
    pass

lib.zip_code_database_new.restype = POINTER(ZipCodeDatabaseStruct)

lib.zip_code_database_free.argtypes = (POINTER(ZipCodeDatabaseStruct), )

lib.zip_code_database_populate.argtypes = (POINTER(ZipCodeDatabaseStruct), )

lib.zip_code_database_population_of.argtypes = \
    (POINTER(ZipCodeDatabaseStruct), c_char_p)
lib.zip_code_database_population_of.restype = c_uint32

class ZipCodeDatabase:

    def __init__(self):
        self.obj = lib.zip_code_database_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.zip_code_database_free(self.obj)

    def populate(self):
        lib.zip_code_database_populate(self.obj)

    def population_of(self, zip_code):
        return lib.zip_code_database_population_of(
            self.obj,
            zip_code.encode('utf-8'))

with ZipCodeDatabase() as database:
    database.populate()
    pop1 = database.population_of('90210')
    pop2 = database.population_of('20500')
    print('pop1: {}, pop2: {}'.format(pop1, pop2))
    assert 90210 == pop1
    assert 20500 == pop2

class ApiTest(unittest.TestCase):

    def test_new_and_populate_and_free(self):
        with ZipCodeDatabase() as database:
            database.populate()

    def test_population_of(self):
        with ZipCodeDatabase() as database:
            database.populate()

            result = database.population_of('00000')

            self.assertEqual(0, result)

            result = database.population_of('90210')

            self.assertEqual(90210, result)

            result = database.population_of('20500')

            self.assertEqual(20500, result)

            result = database.population_of('99999')

            self.assertEqual(99999, result)

            result = database.population_of('non-existing-zip-code')

            self.assertEqual(0, result)

if __name__ == '__main__':
    unittest.main()
