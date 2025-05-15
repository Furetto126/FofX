use super::constraint::Constraint;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Constraint for Point {
    fn to_linear_equation(&self, num_variables: usize) -> Vec<f64> {
        let mut linear = vec![0.0; num_variables+1];
        for i in 0..num_variables {
            linear[i] = self.x.powi((num_variables-i-1) as i32);
        }
        linear[num_variables] = self.y;

        linear
    }
    
    fn to_string(&self, name: String) -> String {
        format!("P_{{{name}}}=({}, {})", self.x, self.y)
    }
}