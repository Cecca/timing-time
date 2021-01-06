use cpu_time::ProcessTime;
use precision::Precision;
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

#[allow(dead_code)]
fn run_wall_clock_inner(n: usize, m: usize) -> (Duration, usize) {
    let v: Vec<usize> = (0..m).collect();

    let mut sum: usize = 0;
    let mut total = Duration::from_secs(0);
    for _ in 0..n {
        let s = Instant::now();
        sum += v.iter().sum::<usize>();
        let e = Instant::now();
        let elapsed = e - s;
        total += elapsed;
    }
    (total, sum)
}

// This is super duper slow
#[allow(dead_code)]
fn run_cpu_time(n: usize, m: usize) -> (Duration, usize) {
    let v: Vec<usize> = (0..m).collect();

    let start = Instant::now();
    let mut sum: usize = 0;
    for _ in 0..n {
        let s = ProcessTime::now();
        sum += v.iter().sum::<usize>();
        let _elapsed = s.elapsed();
    }
    let end = Instant::now();
    (end - start, sum)
}

fn run_precision(n: usize, m: usize) -> (Duration, usize) {
    let v: Vec<usize> = (0..m).collect();

    let p = Precision::new(precision::Config::default()).unwrap();

    let start = Instant::now();
    let mut sum: usize = 0;
    for _ in 0..n {
        let s = p.now();
        sum += v.iter().sum::<usize>();
        let e = p.now();
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

    let mut measures = Vec::new();

    let mut sink = 0usize;

    for _ in 0..samples {
        // print!("Setting the baseline... ");
        let (baseline, sum1) = run_baseline(n, m);
        // println!("{:?}", baseline);
        measures.push(("baseline", samples, n, m, baseline / n as u32));

        // print!("Measuring wall clock... ");
        let (wall_clock, sum2) = run_wall_clock(n, m);
        // println!("{:?}", wall_clock);
        measures.push(("wall_clock", samples, n, m, wall_clock / n as u32));

        // print!("Measuring `precision` time... ");
        let (precision_time, sum3) = run_precision(n, m);
        // println!("{:?}", precision_time);
        measures.push(("precision_time", samples, n, m, precision_time / n as u32));

        sink += sum1 + sum2 + sum3;
        // println!("{}", sum1 + sum2 + sum3);
    }
    eprintln!("sink sum {}", sink);

    // println!("method, samples, n, m, time_ns");
    for (method, samples, n, m, time) in measures.into_iter() {
        println!("{}, {}, {}, {}, {}", method, samples, n, m, time.as_nanos());
    }
}
