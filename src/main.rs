use constraints::{constraint::Constraint, Integral, Derivative, Point};
use solver::function_through;

use num_rational::Ratio;

mod constraints;
mod solver;

fn float_to_fraction(x: f64) -> Option<(i64, i64)> {
    Ratio::approximate_float(x).map(|r| (*r.numer(), *r.denom()))
}

fn main() {
    let constraints: Vec<Box<dyn Constraint>> = vec![
        Box::new(Point { x: 1.0, y: 5.0 }),
        Box::new(Point { x: 3.0, y: 4.0 }),
        Box::new(Point { x: 5.0, y: 1.0 }),
        Box::new(Derivative {
            value: 5.0,
            at_x: 6.0,
        }),
        Box::new(Integral {
            value: 10.0,
            from: 0.0,
            to: 20.0 
        })
    ];
    for i in 0..constraints.len() {
        println!("{}", constraints[i].to_string(i.to_string()));
    }
    println!();

    let coefficients = function_through(&constraints);
    print!("f(x) = ");
    for i in 0..coefficients.len() {
        if i < coefficients.len() - 1 {
            if let Some((num, den)) = float_to_fraction(coefficients[i]) {
                print!(
                    "\\frac{{{num}}}{{{den}}}x^{{{}}} + ",
                    coefficients.len() - i - 1
                );
            }
        } else {
            if let Some((num, den)) = float_to_fraction(coefficients[i]) {
                print!("\\frac{{{num}}}{{{den}}}");
            }
        }
    }
}
