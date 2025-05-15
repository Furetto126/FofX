use super::constraint::Constraint;

#[derive(Debug, Clone, Copy)]
pub struct Derivative {
    pub value: f64,
    pub at_x:  f64
}

impl Constraint for Derivative {
    fn to_linear_equation(&self, num_variables: usize) -> Vec<f64> {
        let mut linear = vec![0.0; num_variables+1];

        for i in 0..num_variables-1 {
            let original_degree = (num_variables-i-1) as i32;
            linear[i] = (original_degree as f64)*self.at_x.powi(original_degree-1)
        }
        linear[num_variables] = self.value;

        linear
    }
    
    fn to_string(&self, _: String) -> String {
        format!("f'({})", self.at_x)
    }
}