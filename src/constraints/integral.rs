use super::constraint::Constraint;

#[derive(Debug, Clone, Copy)]
pub struct Integral {
    pub value: f64,
    pub from:  f64,
    pub to:    f64
}

impl Constraint for Integral {
    fn to_linear_equation(&self, num_variables: usize) -> Vec<f64> {
        let mut linear = vec![0.0; num_variables+1];

        for i in 0..num_variables {
            let exp = (num_variables-i) as i32;
            linear[i] = (self.to.powi(exp) - self.from.powi(exp)) / (exp as f64);
        }
        linear[num_variables] = self.value;

        linear
    }

    fn to_string(&self, _: String) -> String {
        format!("\\int_{{{}}}^{{{}}}f(x) dx", self.from, self.to)
    }
}

