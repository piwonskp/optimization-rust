
use types::Function;


/// Specifies a well known optimization problem.
pub trait Problem: Function + Default {
    /// Returns the dimensionality of the input domain.
    fn dimensions(&self) -> usize;

    /// Returns the input domain of the function in terms of upper and lower,
    /// respectively, for each input dimension.
    fn domain(&self) -> Vec<(f64, f64)>;

    /// Returns the position as well as the value of the global minimum.
    fn minimum(&self) -> (Vec<f64>, f64);

    /// Generates a random and **feasible** position to start a minimization.
    fn random_start(&self) -> Vec<f64>;

    /// Tests whether the supplied position is legal for this function.
    fn is_legal_position(&self, position: &[f64]) -> bool {
        position.len() == self.dimensions() &&
        position.iter().zip(self.domain()).all(|(&x, (lower, upper))| {
            lower < x && x < upper
        })
    }
}


#[cfg(feature = "rand")]
pub mod problems;
#[cfg(feature = "rand")]
pub use self::problems::*;