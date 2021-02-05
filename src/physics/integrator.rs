use num::Float;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// An independent variable, i.e, time in a differential
/// equation. Basically can only be f32 or f64.
pub trait Independent: Float + Clone + Copy + AddAssign + Add {}

impl<F: Float + AddAssign> Independent for F {}

/// The dependent variable, i.e, position in a differential equation.
///
/// This is also the "shape" of the equation's derivatives; i.e, if
/// the function is vector-valued, we assume its derivatives are also
/// vector-valued.
pub trait Dependent<T: Independent>
where
    Self: Clone + Copy,
    Self: AddAssign + MulAssign<T>,
    Self: Add<Self, Output = Self> + Mul<T, Output = Self>,
{
}

/// A first-order ordinary differential equation.
///
/// In mixed Rust / math terms, X' = self.derivative(X, T)
pub trait FirstOrderODE<X, T>
where
    X: Dependent<T>,
    T: Independent,
{
    fn derivative(&self, dep: X, indep: T) -> X;
}

impl<F: Fn(X, T) -> X, X: Dependent<T>, T: Independent> FirstOrderODE<X, T> for F {
    fn derivative(&self, dep: X, indep: T) -> X {
        self(dep, indep)
    }
}

/// Something which can numerically integrate one step of a
/// first-order ODE, given an equation, the current values of X and T,
/// and the time step.
pub trait Integrator<X, T>
where
    X: Dependent<T>,
    T: Independent,
{
    fn step(&self, dep: X, indep: T, step: T, equation: &dyn FirstOrderODE<X, T>) -> X;
}

impl<F, X, T> Integrator<X, T> for F
where
    F: Fn(X, T, T, &dyn FirstOrderODE<X, T>) -> X,
    X: Dependent<T>,
    T: Independent,
{
    fn step(&self, dep: X, indep: T, step: T, equation: &dyn FirstOrderODE<X, T>) -> X {
        self(dep, indep, step, equation)
    }
}

/// Euler's method.
pub fn euler<X, T>(x0: X, t0: T, dt: T, equation: &dyn FirstOrderODE<X, T>) -> X
where
    X: Dependent<T>,
    T: Independent,
{
    let xdot = equation.derivative(x0, t0);
    let dx = xdot * dt;
    x0 + dx
}

/// Fourth-order Runge-Kutta method.
pub fn rk4<X, T>(x0: X, t0: T, h: T, equation: &dyn FirstOrderODE<X, T>) -> X
where
    X: Dependent<T>,
    T: Independent,
{
    let two = T::from(2).unwrap();
    let six = T::from(6).unwrap();
    let h2 = h / two;
    let h6 = h / six;

    let k1 = equation.derivative(x0, t0);
    let k2 = equation.derivative(x0 + k1 * h2, t0 + h2);
    let k3 = equation.derivative(x0 + k2 * h2, t0 + h2);
    let k4 = equation.derivative(x0 + k3 * h, t0 + h);

    x0 + (k1 + k2 * two + k3 * two + k4) * h6
}
