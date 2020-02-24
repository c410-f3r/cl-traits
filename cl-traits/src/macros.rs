/// Creates a marker trait `$trait_name` with or without bounds.
///
/// A default implementation is also provided for anything, regardless of the presence of
/// the given bounds.
#[macro_export]
macro_rules! create_and_impl_marker_trait {
  ($v:vis $trait_name:ident $(:)? $($bound:path),*) => {
    $v trait $trait_name: $($bound +)* {}

    impl<T> $trait_name for T where T: $($bound +)* {}
  }
}

/// Creates a marker trait `$trait_name` with or without bounds.
#[macro_export]
macro_rules! create_marker_trait {
  ($v:vis $trait_name:ident $(:)? $($bound:path),*) => {
    $v trait $trait_name: $($bound +)* {}
  }
}
