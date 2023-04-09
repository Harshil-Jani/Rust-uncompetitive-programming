// Author : Harshil Jani <harshiljani2002@gmail.com>
// Problem Link : https://codeforces.com/problemset/problem/1676/B

use std::io;

/* DOUBT : What if I want the fixed value for each test case iteration.
I have asked the community about this and they are yet to reply
Maybe anyone can help me with this. Better explaination is that, For whatever code
I will have been written over here, You might feel that there is no significance of `n`.

So, I encourage you to use `n` and face the dilemma of `n` being a `const` variable or a
`let` variable.
*/
fn logic() {
    // Put core problem logic here.
    let mut n: String = String::new();
    let mut a: String = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to take input\n");

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to take input\n");

    // let _n: u32 = n.trim().parse().expect("Failed to convert to integer\n");
    let mut a: Vec<u32> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse integer\n"))
        .collect::<Vec<u32>>();
    let mut eat: u32 = 0;
    a.sort();
    for i in 0..a.len() {
        eat += a[i] - a[0];
    }
    println!("{}", eat);
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
