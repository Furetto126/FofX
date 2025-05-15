pub trait Constraint {
    fn to_linear_equation(&self, num_variables: usize) -> Vec<f64>;
    fn to_string(&self, name: String) -> String;
}