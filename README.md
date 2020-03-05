# Output of `cargo bench`

```
   Compiling serialization-benchmark v0.1.0 (<redacted>)
    Finished bench [optimized] target(s) in 7.43s
     Running target/release/deps/serialization_benchmark-0c2a00d8fc8cb3db

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

     Running target/release/deps/io-f0d5151810ca042f
Gnuplot not found, using plotters backend
Benchmarking write/ours
Benchmarking write/ours: Warming up for 3.0000 s
Benchmarking write/ours: Collecting 100 samples in estimated 5.4122 s (45k iterations)
Benchmarking write/ours: Analyzing
write/ours              time:   [116.67 us 118.75 us 122.17 us]
                        change: [-2.9693% -2.0176% -0.7077%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
Benchmarking write/theirs
Benchmarking write/theirs: Warming up for 3.0000 s
Benchmarking write/theirs: Collecting 100 samples in estimated 5.7396 s (35k iterations)
Benchmarking write/theirs: Analyzing
write/theirs            time:   [163.77 us 167.61 us 172.34 us]
                        change: [-4.0755% -1.8991% +0.5459%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Benchmarking write/ours #2
Benchmarking write/ours #2: Warming up for 3.0000 s
Benchmarking write/ours #2: Collecting 100 samples in estimated 5.2563 s (61k iterations)
Benchmarking write/ours #2: Analyzing
write/ours #2           time:   [85.483 us 85.697 us 85.942 us]
                        change: [-13.314% -10.280% -7.4250%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  9 (9.00%) high mild
  2 (2.00%) high severe
Benchmarking write/theirs #2
Benchmarking write/theirs #2: Warming up for 3.0000 s
Benchmarking write/theirs #2: Collecting 100 samples in estimated 5.6881 s (30k iterations)
Benchmarking write/theirs #2: Analyzing
write/theirs #2         time:   [185.31 us 186.96 us 188.56 us]
                        change: [-0.4224% +1.3179% +3.1449%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) low mild
  1 (1.00%) high mild

```