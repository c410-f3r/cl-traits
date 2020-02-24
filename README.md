# Collection Traits (cl-traits)

Yet another library that provides traits that describe collections. This is a best effort without GAT.

Many data structures have unique features that make it difficult or even impossible to create a single `trait` that fits all. This crate tries to circumvent such behaviour by providing a single method for each `trait` to achieve maximum flexibility and freedom.

## Examples


```rust
use cl_traits::*;

struct SomeCustomVector(Vec<i32>, Vec<i32>);

impl Length for SomeCustomVector {
  type Output = usize;

  fn length(&self) -> Self::Output {
    self.0.length() + self.1.length()
  }
}

fn main() {
  let v = SomeCustomVector(vec![1, 2], vec![3, 4, 5, 6]);
  assert_eq!(v.length(), 6);
}
```

You can see more complete examples in the `examples` subdirectories.

## Derives

Derives are somewhat limited because they aggregate every single attribute, threfore, should be used with caution.

```rust
use cl_traits::*;
use cl_traits_derive::*;

#[derive(WithLength)]
struct SomeCustomVector(Vec<i32>, Vec<i32>);

fn main() {
  let v = SomeCustomVector(vec![1, 2], vec![3, 4, 5, 6]);
  assert_eq!(v.length(), (2, 4));
}
```