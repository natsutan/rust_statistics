use statrs::distribution::{Beta, Continuous};
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use statrs::distribution::ContinuousCDF;
fn main() {
    let a = 14.0;
    let b  = 27.0;

    let n = Beta::new(a, b).unwrap();
    let pdf = n.pdf(0.5);
    println!("pdf(0.5) = {}", pdf);

    // 0.5までのcdfを求める。
    let cdf = n.cdf(0.5);
    println!("cdf(0.5) = {}", cdf);


    let xs = (0..100).map(|x| x as f64 / 100.0).collect::<Vec<f64>>();
    let ys = xs.iter().map(|&x| n.pdf(x)).collect::<Vec<f64>>();


    let trace = Scatter::new(xs, ys).mode(Mode::Lines).name("normal distribution");

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}