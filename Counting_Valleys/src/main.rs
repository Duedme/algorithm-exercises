use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut number = 0;
    let mut level = 0;

    path.chars().for_each(|l| {
        match l {
            'D' => level -= 1,
            'U' => {
                level += 1;
                if level == 0 { number += 1; }
            },
            _ => {},
        }
    });

    number

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();

}