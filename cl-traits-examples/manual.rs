//! Manual trait implementations.

use cl_traits::*;

trait GenericVector<I>:
  Capacity<Output = usize>
  + Clear<Output = ()>
  + Length<Output = usize>
  + Push<Input = I, Ok = ()>
  + Swap<Input = [usize; 2], Output = ()>
  + Truncate<Input = usize, Output = ()>
{
}

impl<I, T> GenericVector<I> for T where
  T: Capacity<Output = usize>
    + Clear<Output = ()>
    + Length<Output = usize>
    + Push<Input = I, Ok = ()>
    + Swap<Input = [usize; 2], Output = ()>
    + Truncate<Input = usize, Output = ()>
{
}

struct SomeCustomVector<I>(Vec<I>);

impl<I> Capacity for SomeCustomVector<I> {
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

impl<I> Length for SomeCustomVector<I> {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.0.length()
  }
}

impl<I> Push for SomeCustomVector<I> {
  type Error = core::convert::Infallible;
  type Input = I;
  type Ok = ();

  fn push(&mut self, elem: I) -> Result<Self::Ok, Self::Error> {
    self.0.push(elem);
    Ok(())
  }
}

impl<T> Swap for SomeCustomVector<T> {
  type Input = [usize; 2];
  type Output = ();

  fn swap(&mut self, [a, b]: Self::Input) -> Self::Output {
    self.0.as_mut_slice().swap(a, b)
  }
}

impl<T> Truncate for SomeCustomVector<T> {
  type Input = usize;
  type Output = ();

  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.0.truncate(input)
  }
}

fn stuff<I, T>(_: &mut T)
where
  T: GenericVector<I>,
{
}

fn main() {
  let mut v = SomeCustomVector(vec![1, 2, 3, 4]);
  stuff(&mut v);
}
