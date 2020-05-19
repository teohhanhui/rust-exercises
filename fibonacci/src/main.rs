use im::ordmap;
use im::ordmap::OrdMap;
use std::io::{self, BufRead};
use std::process;

fn fibonacci_recursion_naive(n: u8) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursion_naive(n - 2) + fibonacci_recursion_naive(n - 1),
    }
}

fn fibonacci_iter(n: u8) -> u128 {
    match (0..=n).fold((0, 0), |(fib_nm1, fib_nm2), n| match n {
        0 => (0, 0),
        1 => (1, 0),
        _ => (fib_nm2 + fib_nm1, fib_nm1),
    }) {
        (n, _) => n,
    }
}

fn main() {
    const DEFAULT_N: u8 = 8;
    const DEFAULT_F: fn(u8) -> u128 = fibonacci_iter;

    // let fibonacci_fns: OrdMap<&str, fn(u8) -> u128> =
    //     ordmap! {"recursion_naive" => fibonacci_recursion_naive, "iter" => fibonacci_iter};
    let fibonacci_fns: OrdMap<&str, fn(u8) -> u128> = ordmap! {};
    let fibonacci_fns = fibonacci_fns.update("recursion_naive", fibonacci_recursion_naive);
    let fibonacci_fns = fibonacci_fns.update("iter", fibonacci_iter);

    println!("n [{}]:", DEFAULT_N);

    let n = match io::stdin().lock().lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => panic!("Failed to read line: {:?}", err),
        None => {
            eprintln!("No input");
            process::exit(1);
        }
    };
    let n = if n.is_empty() {
        DEFAULT_N
    } else {
        n.trim().parse::<u8>().unwrap_or_else(|_err| {
            eprintln!("Invalid n: {}", n);
            process::exit(1);
        })
    };

    println!("");
    for (i, (name, _f)) in fibonacci_fns.iter().enumerate() {
        println!("{}) {}", i + 1, name);
    }

    println!(
        "F [{}]:",
        fibonacci_fns
            .iter()
            .position(|(_name, &f)| f == DEFAULT_F)
            .map(|i| i + 1)
            .unwrap()
    );

    let f = match io::stdin().lock().lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => panic!("Failed to read line: {:?}", err),
        None => {
            eprintln!("No input");
            process::exit(1);
        }
    };
    let f = if f.is_empty() {
        &DEFAULT_F
    } else {
        let f: usize = f.trim().parse().unwrap_or_else(|_err| {
            eprintln!("Invalid F: {}", f);
            process::exit(1);
        });

        fibonacci_fns
            .iter()
            .nth(f - 1)
            .map(|(_name, f)| f)
            .unwrap_or_else(|| {
                eprintln!("Invalid F: {}", f);
                process::exit(1);
            })
    };

    println!("F({})", n);
    println!("= {}", f(n));
}
