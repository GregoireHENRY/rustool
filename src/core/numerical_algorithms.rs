use crate::List;
use itertools::all;
use na::RealField;
use num_traits::NumCast;

/// After that many iterations, consider the numerical method has failed to converge.
const NUMBER_ITERATION_FAIL: usize = 1e6 as usize;
/// Threshold that defines the convergence of the numerical Newton method.
pub const NEWTON_METHOD_THRESHOLD: f64 = 1e-5;

/// Newton's method algorithm.
///
/// # Definition
///
/// Newton's method is an algorithm to find the approximated value to the exact solution of a
/// variable in an equation that cannot be isolated from other terms. The algorithm consist in this
/// algorithm:
///
/// ```ignore
/// loop:
///     next_value = old_value - function(old_value, *args) / derivative(old_value, *args)
///     if abs(next_value - old_value) < threshold:
///         break
/// ```
///
/// Newton's method function is found by moving all terms of the equation to one side to find
/// `f(x) = 0`. The newton method derivative is the derivative of this function. The `threshold` is
/// the accepted residual between the estimated value and the exact solution.
/// `old_value` is a initial guess on the solution.
/// Newton's method is only guaranteed to converge if certain conditions are met (see
/// [this](https://en.wikipedia.org/wiki/Newton%27s_method#Failure_analysis)).
/// The maximum number of iteration is bounded to assume the algorithm does not converge.
///
/// # Usage
///
/// In order to use this Newton's method implementaton, your need to:
///
/// + create a public struct to hold the arguments `*args` to be sent to both function and derivative of
/// the Newton's method
/// + add the implementation of the trait `NewtonMethodArguments` to your struct
/// + create the function of the Newton's method
/// + create the derivative of the Newton's method
///
/// This implementation is for input and output list of float.
///
/// # Example
///
/// ```
/// // The struct that contains your arguments for the Newton's method's function and derivative
/// pub struct CustomNewtonMethodArguments<'a> {
///     some_other_value: &'a f64,
/// }
///
/// // A simple constructor for your struct
/// impl<'a> CustomNewtonMethodArguments<'a> {
///     fn new(some_other_value: &'a f64) -> Self {
///         CustomNewtonMethodArguments {some_other_value}
///     }
/// }
///
/// // The implementation of the trait to your struct
/// impl<'a> tool::NewtonMethodArguments for CustomNewtonMethodArguments<'a> {}
///
/// // Newton's method's function: f(x) = 100 - x ** 4 - x * some_other_value = 0
/// pub fn newton_method_function(
///     value: &tool::List<f64>,
///     newton_method_arguments: &CustomNewtonMethodArguments,
/// ) -> tool::List<f64> {
///     (-tool::pow(value, 4)).add_scalar(100.) - value * *newton_method_arguments.some_other_value
/// }
///
/// // Newton's method's function: f'(x) = - 4 * x ** 3 - some_other_value
/// pub fn newton_method_derivative(
///     value: &tool::List<f64>,
///     newton_method_arguments: &CustomNewtonMethodArguments,
/// ) ->tool::List<f64> {
///     (- 4. * tool::pow(value, 3)).add_scalar(-newton_method_arguments.some_other_value)
/// }
///
/// // The call of the Newton's method
/// let my_initial_vector = tool::List::<f64>::zeros(3);
/// let some_other_value = 12.;
///
/// let result = tool::newton_method(
///     my_initial_vector,
///     newton_method_function,
///     newton_method_derivative,
///     CustomNewtonMethodArguments::new(&some_other_value),
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
        /*
        crate::debug!(
            "curent: {}, func: {}, deri: {}, new: {}",
            current_value[0],
            func_res[0],
            deri_res[0],
            new_value[0]
        );
        */
        let all_convered = all((&new_value - &current_value).iter(), |residual| {
            residual.abs() < NumCast::from(NEWTON_METHOD_THRESHOLD).unwrap()
        });
        if all_convered {
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

/// Trait to implement on your custom structure holding Newton's method arguments.
pub trait NewtonMethodArguments {}
