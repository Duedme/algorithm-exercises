use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pangrams' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn pangrams(s: &str) -> String {
    let mut abc: String = "abcdefghijklmnopqrstuvwxyz".to_string();

    for l in s.chars() {
        abc = abc.replace(l.to_ascii_lowercase(), "");
    };

    if !abc.is_empty() { return "not pangram".to_string() }

    "pangram".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = pangrams(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
