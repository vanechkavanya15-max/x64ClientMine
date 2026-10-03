use std::backtrace::Backtrace;

fn main() {
    let bt = Backtrace::force_capture();
    println!("{}", bt);
}
