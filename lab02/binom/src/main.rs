use num::integer::binomial;

fn main() {
    let p: f64 = 0.2;
    let mut total_p: f64 = 0.0;
    for i in 0..12 {
        total_p += binomial(60i64, i as i64) as f64 * p.powi(i) * (1.0-p).powi(60 - i);
    }
    println!("{}", total_p);
    println!("{}", 1.0 - total_p);
}
