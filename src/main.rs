use std::time::{Duration, Instant};

fn run_baseline(n: usize, m: usize) -> (Duration, usize) {
    let v: Vec<usize> = (0..m).collect();

    let start = Instant::now();
    let mut sum: usize = 0;
    for _ in 0..n {
        sum += v.iter().sum::<usize>();
    }
    let end = Instant::now();
    (end - start, sum)
}

fn run_wall_clock(n: usize, m: usize) -> (Duration, usize) {
    let v: Vec<usize> = (0..m).collect();

    let start = Instant::now();
    let mut sum: usize = 0;
    for _ in 0..n {
        let s = Instant::now();
        sum += v.iter().sum::<usize>();
        let e = Instant::now();
        let _elapsed = e - s;
    }
    let end = Instant::now();
    (end - start, sum)
}

fn main() {
    let samples = std::env::args()
        .nth(1)
        .expect("provide the number of samples")
        .parse::<usize>()
        .expect("the number of iterations should be an integer");
    let n = std::env::args()
        .nth(2)
        .expect("provide the number of iterations")
        .parse::<usize>()
        .expect("the number of iterations should be an integer");
    let m = std::env::args()
        .nth(3)
        .expect("provide the length of the vector to sum")
        .parse::<usize>()
        .expect("the length of the vector should be an integer");

    let mut sink = 0usize;

    let mut baseline_times = Vec::new();
    let mut wall_clock_times = Vec::new();

    for _ in 0..samples {
        let (baseline, sum1) = run_baseline(n, m);
        baseline_times.push(baseline / n as u32);

        let (wall_clock, sum2) = run_wall_clock(n, m);
        wall_clock_times.push(wall_clock / n as u32);

        sink += sum1 + sum2;
    }
    eprintln!("sink sum {}", sink);

    baseline_times.sort();
    wall_clock_times.sort();

    let baseline_avg: Duration =
        baseline_times.iter().sum::<Duration>() / baseline_times.len() as u32;
    let baseline_median = baseline_times[baseline_times.len() / 2];

    let wall_clock_avg: Duration =
        wall_clock_times.iter().sum::<Duration>() / wall_clock_times.len() as u32;
    let wall_clock_median = wall_clock_times[wall_clock_times.len() / 2];

    println!("| {} | {} | {} |", "     ", "average", "median");
    println!("|:----|----:|-----:|");
    println!(
        "| baseline   | {:?} | {:?} |",
        baseline_avg, baseline_median
    );
    println!(
        "| wall_clock | {:?} | {:?} |",
        wall_clock_avg, wall_clock_median
    );
    println!(
        "| overhead   | {:?} | {:?} |",
        wall_clock_avg - baseline_avg,
        wall_clock_median - baseline_median
    )
}
