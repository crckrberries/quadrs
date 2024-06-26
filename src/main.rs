use quadrs::*;
use std::env;

fn main() {
    let inp: Vec<String> = env::args().collect();
    if inp.len() == 1 {
        println!("please supply an equation");
        return;
    }

    let [a, b, c] = parse(&inp[1]);
    let root = quad(a, b, c);

    println!("{:?}", root);
}
