// Trait implementations with macros.

use cl_traits::*;

#[derive(WithCapacity, WithClear, WithLength, WithSwap, WithTruncate)]
struct SomeCustomVector(Vec<i32>);

create_and_impl_marker_trait!(
  GenericVector: Capacity<Output = usize>,
  Clear<Output = ()>,
  Length<Output = usize>,
  Swap<Input = (usize, usize), Output = ()>,
  Truncate<Input = usize, Output = ()>
);

fn stuff<T>(_: &mut T)
where
  T: GenericVector,
{
}

fn main() {
  let mut v = SomeCustomVector(vec![1, 2, 3, 4]);
  stuff(&mut v);
}
