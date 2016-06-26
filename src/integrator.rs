use num::Float;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign};

pub trait Dependent<F: Float> where
    Self: Sized + Debug + Clone, // + Clone + Copy + Debug,
    Self: AddAssign + MulAssign<F>,
    Self: Add + Mul<F, Output = Self>
{}

pub trait Independent where
    Self: Float + Debug, // + Sized + Clone + Copy + Debug + Float,
    Self: AddAssign + Add
{}

impl<F: Float + Debug + AddAssign> Independent for F {}

pub trait FirstOrderODE<X: Dependent<T>, T: Independent>
    where Self: Debug
{
    fn derivative(&self, pos: &X, time: &T) -> X;
}

pub trait Integrator<X: Dependent<T>, T: Independent>
    where Self: Debug
{
    fn step(&mut self, delta_time: &T) -> X;
    fn dependent(&self) -> X;
    fn independent(&self) -> T;
    fn set_dependent(&mut self, new_dependent: X);
    fn set_independent(&mut self, new_independent: T);
}

#[derive(Debug)]
pub struct Euler<X: Dependent<T>, T: Independent> {
    ode: Box<FirstOrderODE<X, T>>,
    dependent: X,
    independent: T,
}

impl<X: Dependent<T>, T: Independent> Euler<X, T> {
    pub fn new(ode: Box<FirstOrderODE<X, T>>, x0: X, t0: T) -> Euler<X, T> {
        Euler {
            ode: ode,
            dependent: x0,
            independent: t0,
        }
    }
}

impl<X: Dependent<T>, T: Independent> Integrator<X, T> for Euler<X, T> {
    fn step(&mut self, delta_time: &T) -> X {
        let xdot = self.ode.derivative(&self.dependent, &self.independent);
        self.dependent += xdot * *delta_time;
        self.independent += *delta_time;
        self.dependent.clone()
    }

    fn dependent(&self) -> X { self.dependent.clone() }
    fn independent(&self) -> T { self.independent }
    fn set_dependent(&mut self, new_dependent: X) { self.dependent = new_dependent }
    fn set_independent(&mut self, new_independent: T) { self.independent = new_independent }
}
