# Performance Comparison


```
> hyperfine --warmup=3 './target/release/aws_sdk_rust_bench' './gos3'
Benchmark 1: ./target/release/aws_sdk_rust_bench
  Time (mean ± σ):      94.9 ms ±   9.0 ms    [User: 8.7 ms, System: 9.6 ms]
  Range (min … max):    74.6 ms … 108.0 ms    30 runs
 
Benchmark 2: ./gos3
  Time (mean ± σ):      79.3 ms ±   8.3 ms    [User: 9.1 ms, System: 8.1 ms]
  Range (min … max):    67.0 ms … 102.5 ms    40 runs
 
Summary
  ./gos3 ran
    1.20 ± 0.17 times faster than ./target/release/aws_sdk_rust_bench
```
