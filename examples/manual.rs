// Manual trait implementations without macros.

#![allow(incomplete_features)]
#![feature(generic_associated_types)]

extern crate cl_traits;

use cl_traits::*;

trait GenericVector<E>:
  Capacity<Output = usize>
  + Clear<Output = ()>
  + Length<Output = usize>
  + Push<Input = E, Output = ()>
  + Swap<Input = (usize, usize), Output = ()>
  + Truncate<Input = usize, Output = ()>
{
}

impl<E, T> GenericVector<E> for T where
  T: Capacity<Output = usize>
    + Clear<Output = ()>
    + Length<Output = usize>
    + Push<Input = E, Output = ()>
    + Swap<Input = (usize, usize), Output = ()>
    + Truncate<Input = usize, Output = ()>
{
}

struct SomeCustomVector<E>(Vec<E>);

impl<E> Capacity for SomeCustomVector<E> {
  type Output = usize;

  fn capacity(&self) -> Self::Output {
    self.0.capacity()
  }
}

impl<T> Clear for SomeCustomVector<T> {
  type Output = ();

  fn clear(&mut self) -> Self::Output {
    self.0.clear()
  }
}

impl<E> Length for SomeCustomVector<E> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.0.length()
  }
}

impl<E> Push for SomeCustomVector<E> {
  type Input = E;
  type Output = ();

  fn push(&mut self, elem: E) -> Self::Output {
    self.0.push(elem);
  }
}

impl<T> Swap for SomeCustomVector<T> {
  type Input = (usize, usize);
  type Output = ();

  fn swap(&mut self, input: Self::Input) -> Self::Output {
    self.0.swap(input)
  }
}

impl<T> Truncate for SomeCustomVector<T> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.0.truncate(input)
  }
}

fn stuff<E, T>(_: &mut T)
where
  T: GenericVector<E>,
{
}

fn main() {
  let mut v = SomeCustomVector(vec![1, 2, 3, 4]);
  stuff(&mut v);
}
