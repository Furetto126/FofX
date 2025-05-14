use solver::{function_through, Point};

use rand::Rng;

mod solver;

fn main() {
    let mut points = vec![];
    
    let mut rng = rand::rng();
    let mut prev_x = 0.0;
    for i in 0..20 {
        let x = rng.random_range(prev_x+1.0..prev_x+20.0);
        let y = rng.random_range(-100.0..100.0);

        points.push(Point { x, y });
        prev_x = x;

        println!("P_{{{i}}} = ({x}, {y})");
    }


    let coefficients = function_through(&points);
    print!("y = ");
    for i in 0..coefficients.len() {
        if i < coefficients.len()-1 {
            print!("{}x^{{{}}} + ", coefficients[i], coefficients.len()-i-1);
        } else {
            print!("{}.", coefficients[i]);
        }
    }
}
