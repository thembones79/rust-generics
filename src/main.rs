use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;


    println!("{}", solve(a, b));
}
