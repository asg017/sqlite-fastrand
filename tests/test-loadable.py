import sqlite3
import unittest
import time
import os

EXT_PATH="./dist/debug/fastrand0"

def connect(ext):
  db = sqlite3.connect(":memory:")

  db.execute("create table base_functions as select name from pragma_function_list")
  db.execute("create table base_modules as select name from pragma_module_list")

  db.enable_load_extension(True)
  db.load_extension(ext)

  db.execute("create temp table loaded_functions as select name from pragma_function_list where name not in (select name from base_functions) order by name")
  db.execute("create temp table loaded_modules as select name from pragma_module_list where name not in (select name from base_modules) order by name")

  db.row_factory = sqlite3.Row
  return db


db = connect(EXT_PATH)

def explain_query_plan(sql):
  return db.execute("explain query plan " + sql).fetchone()["detail"]

def execute_all(sql, args=None):
  if args is None: args = []
  results = db.execute(sql, args).fetchall()
  return list(map(lambda x: dict(x), results))

FUNCTIONS = [
  'fastrand_alphabetic',
  'fastrand_alphanumeric',
  'fastrand_blob',
  'fastrand_bool',
  'fastrand_char',
  'fastrand_debug',
  'fastrand_digit',
  'fastrand_double',
  'fastrand_int',
  'fastrand_int',
  'fastrand_int',
  'fastrand_int64',
  'fastrand_int64',
  'fastrand_int64',
  'fastrand_lowercase',
  'fastrand_seed_get',
  'fastrand_seed_set',
  'fastrand_uppercase',
  'fastrand_version'

]

MODULES = [
]
def spread_args(args):                                                          
  return ",".join(['?'] * len(args))

class TestFastrand(unittest.TestCase):
  def test_funcs(self):
    funcs = list(map(lambda a: a[0], db.execute("select name from loaded_functions").fetchall()))
    self.assertEqual(funcs, FUNCTIONS)

  def test_modules(self):
    modules = list(map(lambda a: a[0], db.execute("select name from loaded_modules").fetchall()))
    self.assertEqual(modules, MODULES)
    
  def test_fastrand_version(self):
    self.assertEqual(db.execute("select fastrand_version()").fetchone()[0][0], "v")
  
  def test_fastrand_debug(self):
    debug = db.execute("select fastrand_debug()").fetchone()[0]
    self.assertEqual(len(debug.splitlines()), 2)

  def test_fastrand_int(self):
    self.skipTest("TODO")
    fastrand_int = lambda x: db.execute("select fastrand_int(?)", []).fetchone()[0]
    self.assertEqual(fastrand_int(), 0)

  def test_fastrand_int64(self):
    self.skipTest("TODO")
    fastrand_int64 = lambda x: db.execute("select fastrand_int64(?)", []).fetchone()[0]
    self.assertEqual(fastrand_int64(), 0)
  
  def test_fastrand_double(self):
    self.skipTest("TODO")
    fastrand_double = lambda x: db.execute("select fastrand_double(?)", []).fetchone()[0]
    self.assertEqual(fastrand_double(), 0)
  
  def test_fastrand_blob(self):
    self.skipTest("TODO")
    fastrand_blob = lambda x: db.execute("select fastrand_blob(?)", []).fetchone()[0]
    self.assertEqual(fastrand_blob(), 0)

  def test_fastrand_seed_get(self):
    self.skipTest("TODO")
    fastrand_seed_get = lambda x: db.execute("select fastrand_seed_get(?)", []).fetchone()[0]
    self.assertEqual(fastrand_seed_get(), 0)

  def test_fastrand_seed_set(self):
    self.skipTest("TODO")
    fastrand_seed_set = lambda x: db.execute("select fastrand_seed_set(?)", []).fetchone()[0]
    self.assertEqual(fastrand_seed_set(), 0)
  

  
  def test_fastrand_alphabetic(self):
    self.skipTest("TODO")
    fastrand_alphabetic = lambda x: db.execute("select fastrand_alphabetic(?)", []).fetchone()[0]
    self.assertEqual(fastrand_alphabetic(), 0)

  def test_fastrand_alphanumeric(self):
    self.skipTest("TODO")
    fastrand_alphanumeric = lambda x: db.execute("select fastrand_alphanumeric(?)", []).fetchone()[0]
    self.assertEqual(fastrand_alphanumeric(), 0)

  def test_fastrand_bool(self):
    self.skipTest("TODO")
    fastrand_bool = lambda x: db.execute("select fastrand_bool(?)", []).fetchone()[0]
    self.assertEqual(fastrand_bool(), 0)

  def test_fastrand_char(self):
    self.skipTest("TODO")
    fastrand_char = lambda x: db.execute("select fastrand_char(?)", []).fetchone()[0]
    self.assertEqual(fastrand_char(), 0)

  def test_fastrand_digit(self):
    self.skipTest("TODO")
    fastrand_digit = lambda x: db.execute("select fastrand_digit(?)", []).fetchone()[0]
    self.assertEqual(fastrand_digit(), 0)
  

  def test_fastrand_lowercase(self):
    self.skipTest("TODO")
    fastrand_lowercase = lambda x: db.execute("select fastrand_lowercase(?)", []).fetchone()[0]
    self.assertEqual(fastrand_lowercase(), 0)

  def test_fastrand_uppercase(self):
    self.skipTest("TODO")
    fastrand_uppercase = lambda x: db.execute("select fastrand_uppercase(?)", []).fetchone()[0]
    self.assertEqual(fastrand_uppercase(), 0)

class TestCoverage(unittest.TestCase):                                      
  def test_coverage(self):                                                      
    test_methods = [method for method in dir(TestFastrand) if method.startswith('test_')]
    funcs_with_tests = set([x.replace("test_", "") for x in test_methods])
    
    for func in FUNCTIONS:
      self.assertTrue(func in funcs_with_tests, f"{func} does not have corresponding test in {funcs_with_tests}")
    
    for module in MODULES:
      self.assertTrue(module in funcs_with_tests, f"{module} does not have corresponding test in {funcs_with_tests}")

if __name__ == '__main__':
    unittest.main()