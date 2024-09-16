use statrs::function::factorial::binomial;

fn main() {
    // コインを24回なげたときに、12回表が出る確率
    let p:f64 = 0.5;
    let n = 24;
    let k = 12;
    let result = binomial(n, k) * p.powf(k as f64) * (1.0 - p).powf((n - k) as f64);

    println!("{}", result);
}
