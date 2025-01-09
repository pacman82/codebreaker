use peg::Peg;
use rand::random;

mod peg;

fn main() {
    let peg: Peg = random();
    println!("{peg}");
}
