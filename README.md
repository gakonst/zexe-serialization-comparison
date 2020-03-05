@ -1,8 +1,4 @@
# Output of `cargo bench`

```
   Compiling serialization-benchmark v0.1.0 (<redacted>)
    Finished bench [optimized] target(s) in 0.08s
     Running target/release/deps/serialization_benchmark-0c2a00d8fc8cb3db

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

     Running target/release/deps/io-f0d5151810ca042f
Gnuplot not found, using plotters backend
Benchmarking write/ours
Benchmarking write/ours: Warming up for 3.0000 s
Benchmarking write/ours: Collecting 100 samples in estimated 5.0656 s (40k iterations)
Benchmarking write/ours: Analyzing
write/ours              time:   [124.15 us 125.56 us 127.25 us]
                        change: [-6.5492% -3.9152% -1.6301%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
Benchmarking write/theirs
Benchmarking write/theirs: Warming up for 3.0000 s
Benchmarking write/theirs: Collecting 100 samples in estimated 5.5351 s (35k iterations)
Benchmarking write/theirs: Analyzing
write/theirs            time:   [148.37 us 149.01 us 149.78 us]
                        change: [-5.2742% -2.9668% -0.8397%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking write_uncompressed/ours
Benchmarking write_uncompressed/ours: Warming up for 3.0000 s
Benchmarking write_uncompressed/ours: Collecting 100 samples in estimated 5.1669 s (56k iterations)
Benchmarking write_uncompressed/ours: Analyzing
write_uncompressed/ours time:   [90.158 us 90.752 us 91.521 us]
                        change: [-6.6436% -4.2717% -2.0979%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
Benchmarking write_uncompressed/theirs
Benchmarking write_uncompressed/theirs: Warming up for 3.0000 s
Benchmarking write_uncompressed/theirs: Collecting 100 samples in estimated 5.6969 s (40k iterations)
Benchmarking write_uncompressed/theirs: Analyzing
write_uncompressed/theirs
                        time:   [141.36 us 145.82 us 151.60 us]
                        change: [+0.1545% +3.1456% +6.6283%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe

```