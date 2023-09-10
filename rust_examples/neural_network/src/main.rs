mod gates;
use crate::gates::*;

fn main() {
    let x = 2.0;
    let y = -1.0;
    let z = forward_multiply_gate(x, y);
    println!("z = {:.1}", z);
}
