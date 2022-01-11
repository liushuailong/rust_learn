fn main() {
    // 输出数字的格式化
    println!("32f is {}c", fahrenheit_2_celsius(32.0));
    println!("100c is {}f", celsius_2_fahrenheit(100.0));
    println!("Hello, world!");
}

fn fahrenheit_2_celsius(feh: f64) -> f64 {
    (feh - 32f64) * ((212f64 - 32f64) / 100f64)
}

fn celsius_2_fahrenheit(cel: f64) -> f64 {
    32.0 + cel * (212.0 - 32.0) / 100.0
}
