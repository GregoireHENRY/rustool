use crate::List;
use itertools::all;
use na::RealField;
use num_traits::NumCast;

/// After so many iterations, consider the numerical method has failed to converge.
pub const NUMBER_ITERATION_FAIL: usize = 1e6 as usize;
/// Threshold that defines the convergence condition of the numerical Newton method.
pub const NEWTON_METHOD_THRESHOLD: f64 = 1e-5;

/// Newton's method algorithm.
///
/// ## Definition
///
/// Newton's method is an algorithm to find the approximated value of the exact solution of a
/// variable in an equation that cannot be isolated from other terms. The algorithm consist in
/// this,
///
/// ```ignore
/// loop:
///     next_value = old_value - function(old_value, *args) / derivative(old_value, *args)
///     if abs(next_value - old_value) < threshold:
///         break
/// ```
///
/// Newton's method function is expressed by moving all terms of the equation to one side to find
/// `f(x) = 0`. The newton method's derivative is the derivative of this function. The `threshold`
/// is the accepted residual between the estimated value and the exact solution. `old_value` is a
/// initial guess of the solution.
/// Newton's method is only guaranteed to converge if certain conditions are met (see
/// [this](https://en.wikipedia.org/wiki/Newton%27s_method#Failure_analysis)).
/// The maximum number of iterations is bounded to assume the algorithm could not converge.
///
/// ## Usage
///
/// In order to use this Newton's method implementaton, your need to:
///
/// + create a struct to hold the arguments `*args` to be sent to both the function and the
///   derivative of the Newton's method
/// + add the trait [`NewtonMethodArguments`] to your struct
/// + create the function of the Newton's method
/// + create the derivative of the Newton's method
///
/// This implementation uses [`List`] for input and output to allow component-wise computations.
///
/// ## Example
///
/// ```
/// use tool::{List, newton_method, NewtonMethodArguments, pows};
///
/// /// The struct that holds your arguments to be sent to the Newton's method's function and
/// /// derivative. There you can define as many arguments as you want.
/// pub struct MyArguments {
///     some_value: f64,
/// }
///
/// /// We add the trait to your struct.
/// impl NewtonMethodArguments for MyArguments {}
///
/// /// Function of the Newton's method equation:
/// ///
/// /// f(x) = 100 - x ^ 4 - x * MyArguments.some_value = 0
/// pub fn newton_method_function(
///     values: &List<f64>,
///     args: &MyArguments,
/// ) -> List<f64> {
///     (-pows(values, 4)).add_scalar(100.) - values * args.some_value
/// }
///
/// /// Derivative of the Newton's method equation:
/// ///
/// /// f'(x) = - 4 * x ^ 3 - MyArguments.some_value
/// pub fn newton_method_derivative(
///     values: &List<f64>,
///     args: &MyArguments,
/// ) -> List<f64> {
///     (- 4. * pows(values, 3)).add_scalar(-args.some_value)
/// }
///
/// // Initialize parameters.
/// let initial_values = List::<f64>::zeros(3);
/// let args = MyArguments { some_value: 12.0 };
///
/// // Calling the Newton's method.
/// let results = newton_method(
///     initial_values,
///     newton_method_function,
///     newton_method_derivative,
///     args,
/// );
/// ```
pub fn newton_method<T, A>(
    start_value: List<T>,
    newton_method_function: impl Fn(&List<T>, &A) -> List<T>,
    newton_method_derivative: impl Fn(&List<T>, &A) -> List<T>,
    newton_method_arguments: A,
) -> List<T>
where
    T: RealField + NumCast,
    A: NewtonMethodArguments,
{
    let mut new_value: List<T>;
    let mut current_value = start_value;
    let mut iteration = 0;
    'convergence: loop {
        let func_res = newton_method_function(&current_value, &newton_method_arguments);
        let deri_res = newton_method_derivative(&current_value, &newton_method_arguments);
        new_value = &current_value - func_res.component_div(&deri_res);
        let all_converged = all((&new_value - &current_value).iter(), |residual| {
            residual.abs() < NumCast::from(NEWTON_METHOD_THRESHOLD).unwrap()
        });
        if all_converged {
            break 'convergence;
        } else if iteration > NUMBER_ITERATION_FAIL {
            panic!("Newton Method could not converge.");
        } else {
            current_value = new_value;
        }
        iteration += 1;
    }
    new_value
}

/// Trait to be added to your custom struc holding Newton's method arguments.
pub trait NewtonMethodArguments {}
