# Performance Comparison


```
hyperfine --warmup=3 'aws s3 ls s3://hakusai-test-bucket' './target/release/aws_sdk_rust_bench hakusai-test-bucket' './awslim-s3 s3 listObjects --bucket hakusai-test-bucket  '
Benchmark 1: aws s3 ls s3://hakusai-test-bucket
  Time (mean ± σ):     454.4 ms ±  11.1 ms    [User: 323.4 ms, System: 58.6 ms]
  Range (min … max):   439.9 ms … 473.6 ms    10 runs
 
Benchmark 2: ./target/release/aws_sdk_rust_bench hakusai-test-bucket
  Time (mean ± σ):     429.2 ms ± 190.7 ms    [User: 92.1 ms, System: 16.8 ms]
  Range (min … max):   192.0 ms … 618.9 ms    11 runs
 
Benchmark 3: ./awslim-s3 s3 listObjects --bucket hakusai-test-bucket  
  Time (mean ± σ):     214.7 ms ± 115.2 ms    [User: 15.3 ms, System: 13.3 ms]
  Range (min … max):    81.2 ms … 437.0 ms    35 runs
 
Summary
  ./awslim-s3 s3 listObjects --bucket hakusai-test-bucket   ran
    2.00 ± 1.39 times faster than ./target/release/aws_sdk_rust_bench hakusai-test-bucket
    2.12 ± 1.14 times faster than aws s3 ls s3://hakusai-test-bucket
```
