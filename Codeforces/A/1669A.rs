// Author : Harshil Jani <harshiljani2002@gmail.com>
// Problem Link : https://codeforces.com/problemset/problem/1669/A

use std::io;

fn logic() {
    // Put core problem logic here.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input\n");

    let input: i32 = input
        .trim()
        .parse()
        .expect("Failed to convert to integer\n");
    match input {
        ..=1399 => println!("Division 4"),
        1400..=1599 => println!("Division 3"),
        1600..=1899 => println!("Division 2"),
        1900.. => println!("Division 1"),
    }
}

fn main() {
    // Number of test cases.
    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line\n");
    let mut t: i32 = t.trim().parse().expect("Failed to convert to integer\n");

    // Running the test cases.
    while t > 0 {
        t = t - 1;
        logic();
    }
}

/*
Apology note to god :
-------------------

"Dear Lord ! I have always been against people who uploaded leetcode or any equivalent solutions on github.
The reason was quite simple, I hate it. It is open source by no means if you can already have access in the
chat or discussion sections of the forum.

I am here doing same mistake but the user base who codes such questions in rust is vanishingly small and the content
which exist in the repository is unique I promise. Please forgive me to do this if this is against the law of universe."

Yours Apologetic
Harshil Jani.
*/
