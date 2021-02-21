// Sometimes bad things happen and there's nothing
// you can do about it. In these cases Rust has
// the panic! macro.
// By default when a panic occurs the program will start
// unwinding, which means Rust walks back up the stack
// and cleans up the data from each function it
// encounters. But this is a lot of work. The alternative
// is to immediately abort which ends up the program
// without cleaning up.
//
// #Cargo.toml
// [profile.release]
// panic = 'abort'

fn main() {
    // panic!("crashed");

    let v = vec![1, 2, 3];
    v[99]; // panics
}
