use quadrs::*;
use std::env;

fn main() {
    let inp: Vec<String> = env::args().collect();

    // discard the second element in the array, as that is the exponent of the first term
    let [a, _, b, c] = parse(&inp[1]);
    let root = quad(a, b, c);

    println!("{:?}", root);
}
