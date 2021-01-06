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
    let n = std::env::args()
        .nth(1)
        .expect("provide the number of iterations")
        .parse::<usize>()
        .expect("the number of iterations should be an integer");
    let m = std::env::args()
        .nth(2)
        .expect("provide the length of the vector to sum")
        .parse::<usize>()
        .expect("the length of the vector should be an integer");

    print!("Setting the baseline... ");
    let (baseline, sum1) = run_baseline(n, m);
    println!("{:?}", baseline);

    print!("Measuring wall clock... ");
    let (wall_clock, sum2) = run_wall_clock(n, m);
    println!("{:?}", wall_clock);

    print!("Measuring wall clock, summing inner iteration... ");
    let (wall_clock_inner, sum3) = run_wall_clock_inner(n, m);
    println!("{:?}", wall_clock_inner);

    // print!("Measuring CPU time... ");
    // let (cpu_time, sum4) = run_cpu_time(n, m);
    // println!("{:?}", cpu_time);

    print!("Measuring `precision` time... ");
    let (precision_time, sum5) = run_precision(n, m);
    println!("{:?}", precision_time);

    println!("{}", sum1 + sum2 + sum3 + sum5);

    println!(
        "Baseline:         {:?} ({:?}/iter) ",
        baseline,
        baseline / n as u32,
    );
    println!(
        "Wall clock:       {:?} ({:?}/iter) ",
        wall_clock,
        wall_clock / n as u32,
    );
    println!(
        "Wall clock inner: {:?} ({:?}/iter) ",
        wall_clock_inner,
        wall_clock_inner / n as u32,
    );
    // println!(
    //     "CPU time:         {:?} ({:?}/iter) ",
    //     cpu_time,
    //     cpu_time / n as u32,
    // );
    println!(
        "`precision` time: {:?} ({:?}/iter) ",
        precision_time,
        precision_time / n as u32,
    );
}
