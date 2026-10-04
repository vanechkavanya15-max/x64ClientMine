use m2_client::cpoly_evaluator::CPoly;

fn main() {
    let mut poly = CPoly::new();
    poly.set_str("k * 10");
    poly.set_var("k", 10.0);
    println!("{}", poly.eval());
}
