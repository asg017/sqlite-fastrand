#!/bin/bash

END=1e6

hyperfine --warmup=10 \
  "sqlite3 :memory: 'select count(random()) from generate_series(1, $END);'" \
  "sqlite3 :memory: '.load dist/release/fastrand0' 'select count(fastrand_int64()) from generate_series(1, $END);'" 


END=1e5
LENGTH=1024
hyperfine --warmup=10 \
  "sqlite3 :memory: 'select count(randomblob($LENGTH)) from generate_series(1, $END);'" \
  "sqlite3 :memory: '.load target/release/libfastrand0' 'select count(fastrand_blob($LENGTH)) from generate_series(1, $END);'"