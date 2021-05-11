# Go Rust Channel Benchmark

Compare performance of Go channels and goroutines with Rust channels and coroutines provided by runtimes `async-std` and `tokio`.

----------------
The benchmarks create a u64 channel, then spawn some threads (10 and 10000),
where each thread sends a u64 over the channel. Then we wait until all channel
messages have been received. Also different sizes of buffers are used in case 
of 10000 threads.

In case of Go we start reading after spawning goroutines but experiments showed
no difference between this case and case when we spawn reader in the beginning.
In case of `tokio` running reader early provides performance boost, but in case 
of `async-std` reading from channel after spawning writers is faster. So for 
each runtime fastest version was used.

To run the benchmark, clone this repository and then run in a terminal: ./run.sh

----------------
Results on a AMD Ryzen 7 3700X:

Go:  
```
BenchmarkGo10threads_1bufsize-16                  131868               8.902 ns/op                                        
BenchmarkGo10000threads_1bufsize-16                  100          11.841.428 ns/op                                        
BenchmarkGo10000threads_10bufsize-16                  85          11.815.912 ns/op                                        
BenchmarkGo10000threads_100bufsize-16                 85          11.788.588 ns/op                                        
BenchmarkGo10000threads_1000bufsize-16                91          11.079.378 ns/op                                        
BenchmarkGo10000threads_10000bufsize-16              276           4.353.314 ns/op
```

Rust (`tokio`): 
```
tokio test 10 threads 1 buf size                 time:   [18.856 us 19.559 us 20.257 us]  
tokio test 10000 threads 1 buf size              time:   [9.5865 ms 9.8393 ms 10.085 ms]  
tokio test 10000 threads 10 buf size             time:   [13.607 ms 14.016 ms 14.428 ms]  
tokio test 10000 threads 100 buf size            time:   [6.3051 ms 6.3938 ms 6.4887 ms]  
tokio test 10000 threads 1000 buf size           time:   [6.1118 ms 6.1327 ms 6.1536 ms]  
tokio test 10000 threads 10000 buf size          time:   [6.0412 ms 6.0665 ms 6.0915 ms]  
```
Rust (`async-std`): 
```
async-std test 10                                time:   [16.722 us 16.814 us 16.903 us]  
async-std test 10000 threads 1 buf size          time:   [18.196 ms 18.348 ms 18.500 ms]  
async-std test 10000 threads 10 buf size         time:   [12.733 ms 12.872 ms 13.021 ms]  
async-std test 10000 threads 100 buf size        time:   [12.398 ms 12.468 ms 12.541 ms]  
async-std test 10000 threads 1000 buf size       time:   [12.501 ms 12.571 ms 12.646 ms]  
async-std test 10000 threads 10000 buf size      time:   [14.371 ms 14.402 ms 14.432 ms]  
```
----------------

Results show that `tokio` runtime is faster than Go with some buffer sizes 
(100, 1000) and slower with small buffer sizes (1, 10) and buffer sizes close 
to the amount of threads (10000). `async-std` is slower than `tokio` on average and it almost matches performance of Go on buffer sizes 100 and 1000.