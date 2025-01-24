# Performance Comparison


```
hyperfine --warmup=3 'aws s3 ls s3://hakusai-test-bucket' './target/release/aws_sdk_rust_bench hakusai-test-bucket' './awslim-s3 s3 listObjectsV2 --bucket hakusai-test-bucket  '
Benchmark 1: aws s3 ls s3://hakusai-test-bucket
  Time (mean ± σ):     454.6 ms ±  11.3 ms    [User: 322.1 ms, System: 58.5 ms]
  Range (min … max):   439.6 ms … 473.7 ms    10 runs
 
Benchmark 2: ./target/release/aws_sdk_rust_bench hakusai-test-bucket
  Time (mean ± σ):     187.1 ms ±   6.1 ms    [User: 83.0 ms, System: 8.8 ms]
  Range (min … max):   179.5 ms … 198.6 ms    14 runs
 
Benchmark 3: ./awslim-s3 s3 listObjectsV2 --bucket hakusai-test-bucket  
  Time (mean ± σ):      80.8 ms ±   5.8 ms    [User: 11.0 ms, System: 9.1 ms]
  Range (min … max):    66.1 ms …  94.1 ms    38 runs
 
Summary
  ./awslim-s3 s3 listObjectsV2 --bucket hakusai-test-bucket   ran
    2.32 ± 0.18 times faster than ./target/release/aws_sdk_rust_bench hakusai-test-bucket
    5.63 ± 0.43 times faster than aws s3 ls s3://hakusai-test-bucket
```
