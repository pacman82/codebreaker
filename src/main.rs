use code::Code;
use rand::random;

mod code;
mod peg;

fn main() {
    let code: Code = random();
    println!("{code}");
}
