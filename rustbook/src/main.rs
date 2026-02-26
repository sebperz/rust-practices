//Punto fijo

use libm::exp;

fn main() {
    let mut x = 0.0;
    let mut x_plus_one: f64;
    let target_error = 0.01;

    loop {
        x_plus_one = function(x);
        println!("Value of Xi: \t{}", x);
        println!("Value of Xi+1: \t{}", x_plus_one);
        let error: f64 = calc_error(x, x_plus_one);
        println!("Error: {}\n", error);

        if error <= target_error {
            println!("END of process");
            break;
        }
        x = x_plus_one;
    }
}

fn function(num: f64) -> f64 {
    exp(-num)
    // 0.4 * exp(num.powf(2.0))
}

fn calc_error(x0: f64, x1: f64) -> f64 {
    ((x1 - x0) / x1).abs()
}
