use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn factorize(n: u128) -> (u128, u128) {
    if n % 2 == 0 {
        return (2, n / 2);
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return (i, n / i);
        }
        i += 2;
    }
    (n, 1) // fallback
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: factors <file>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(num_str) = line {
                if let Ok(n) = num_str.trim().parse::<u128>() {
                    let (p, q) = factorize(n);
                    println!("{}={}*{}", n, p, q);
                }
            }
        }
    } else {
        eprintln!("Error: could not read file {}", file_path);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
