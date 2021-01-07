# How much time to take the time?

This simple benchmark measures how much it costs to take the wall clock time in Rust, using 
[`std::time::Instant::now()`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now).
This call requires a system call to the OS, and then a bunch of machiney (including a mutex) 
to ensure that the clock is monotonic.

## tl;dr

Measuring elapsed time is in the ballpark of 30 to 100 nanoseconds.

## The benchmark

The benchmark uses as a baseline the time required to sum `m` consecutive integers 
stored in a `Vec`.
Since this operation is very fast, it is repeated in a loop `n` times and we measure
the total time of the `n` iterations: we then take the average of the `n` runs.
Since the loop does not have side effects, in order to prevent the compiler from being too smart
we accumulate the sums into a global counter, which will eventually be printed to the console.
```rust
let mut sum: usize = 0;
for _ in 0..n {
    sum += v.iter().sum::<usize>();
}
```
This is a single sample: we repeat the process `samples` times, computing the average and the median.

To measure the impact of taking the time, we add to each of the `n` iterations two 
invocations to `std::time::Instant::now()` and a computation
of the elapsed time as follows:
```rust
let mut sum: usize = 0;
for _ in 0..n {
    let s = Instant::now();
    sum += v.iter().sum::<usize>();
    let e = Instant::now();
    let _elapsed = e - s;
}
```

## Results

All the following results have been obtained with

    cargo run --release -- 9999 999 999 
    
that is taking 9999 samples, each performing 999 iterations where each iteration is summing the contents of a vector of 999 elements

### MacBook pro - Intel(R) Core(TM) i5-7360U CPU @ 2.30GHz 

|       | average | median |
|:----|----:|-----:|
| baseline   | 92ns | 88ns |
| wall_clock | 146ns | 143ns |
| overhead   | 54ns | 55ns |

### Linux 4.4.0 - Intel(R) Xeon(R) CPU E5-2667 v3 @ 3.20GHz

|       | average | median |
|:----|----:|-----:|
| baseline   | 97ns | 89ns |
| wall_clock | 136ns | 125ns |
| overhead   | 39ns | 36ns |

### Linux 4.4.0 - Intel(R) Xeon(R) CPU E5-2690 v4 @ 2.60GHz

|       | average | median |
|:----|----:|-----:|
| baseline   | 116ns | 100ns |
| wall_clock | 167ns | 142ns |
| overhead   | 51ns | 42ns |
