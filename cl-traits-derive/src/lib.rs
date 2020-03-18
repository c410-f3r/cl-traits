extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
  parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DeriveInput,
  Field, Fields, Index,
};

macro_rules! create_trait {
  (
    $derive_name:ident,
    $derive_fn_name:ident,
    $trait_name:ident$(<$trait_generic:ident>)?,
    $trait_assoc_name:ident,
    $trait_fn_name:ident($trait_fn_self:ty $(, input: $trait_fn_input:ty)?) -> $trait_fn_ret:ty,
    $has_inputs:literal
  ) => {
    #[proc_macro_derive($derive_name)]
    pub fn $derive_fn_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
      let (data, data_name, trait_name, fields_fns) = common_properties(
        parse_macro_input!(input as DeriveInput),
        stringify!($trait_fn_name),
        stringify!($trait_name),
        $has_inputs
      );
      let input_fields_types = fields_types(&trait_name, stringify!($trait_assoc_name), &data);
      proc_macro::TokenStream::from(quote! {
        impl ::cl_traits::$trait_name$(<$trait_generic>)? for #data_name {
          type $trait_assoc_name = #input_fields_types;

          fn $trait_fn_name(
            self: $trait_fn_self
            $(, input: $trait_fn_input)?
          ) -> $trait_fn_ret {
            #fields_fns
          }
        }
      })
    }
  };
  (
    $derive_name:ident,
    $derive_fn_name:ident,
    $trait_name:ident$(<$trait_generic:ident>)?,
    $trait_assoc_input:ident,
    $trait_assoc_ret:ident,
    $trait_fn_name:ident($trait_fn_self:ty, Self::$trait_fn_input:ty) -> $trait_fn_ret:ty
  ) => {
    #[proc_macro_derive($derive_name)]
    pub fn $derive_fn_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
      let (data, data_name, trait_name, fields_fns) = common_properties(
        parse_macro_input!(input as DeriveInput),
        stringify!($trait_fn_name),
        stringify!($trait_name),
        true
      );
      let input_fields_types = fields_types(&trait_name, stringify!($trait_assoc_input), &data);
      let ret_fields_types = fields_types(&trait_name, stringify!($trait_assoc_ret), &data);
      proc_macro::TokenStream::from(quote! {
        impl ::cl_traits::$trait_name$(<$trait_generic>)? for #data_name {
          type $trait_assoc_input = #input_fields_types;
          type $trait_assoc_ret = #ret_fields_types;

          fn $trait_fn_name(
            self: $trait_fn_self,
            input: Self::$trait_fn_input
          ) -> $trait_fn_ret {
            #fields_fns
          }
        }
      })
    }
  }
}

fn aggregate_fields<I>(fields: I, len: usize) -> TokenStream
where
  I: Iterator<Item = TokenStream>,
{
  if len > 1 {
    quote!((#(#fields,)*))
  } else {
    quote!(#(#fields)*)
  }
}

fn common_properties(
  input: DeriveInput,
  trait_fn_name: &str,
  trait_name: &str,
  has_input: bool,
) -> (Data, Ident, Ident, TokenStream) {
  let data_name = input.ident;
  let trait_fn_name = Ident::new(trait_fn_name, Span::call_site());
  let trait_name = Ident::new(trait_name, Span::call_site());
  let fields_fns = fields_fns(&trait_fn_name, &input.data, has_input);
  (input.data, data_name, trait_name, fields_fns)
}

fn construct_fields_fns<F>(
  fields: &Punctuated<Field, Comma>,
  trait_fn_name: &Ident,
  has_input: bool,
  cb: F,
) -> TokenStream
where
  F: Fn(usize, &Field) -> TokenStream,
{
  if has_input {
    if fields.len() > 1 {
      let i = fields.iter().enumerate().map(|(idx, f)| {
        let syn_idx = Index::from(idx);
        let name = cb(idx, f);
        quote_spanned!(f.span()=> self.#name.#trait_fn_name(input.#syn_idx))
      });
      aggregate_fields(i, fields.len())
    } else {
      let i = fields.iter().enumerate().map(|(idx, f)| {
        let name = cb(idx, f);
        quote_spanned!(f.span()=> self.#name.#trait_fn_name(input))
      });
      aggregate_fields(i, fields.len())
    }
  } else {
    let i = fields.iter().enumerate().map(|(idx, f)| {
      let name = cb(idx, f);
      quote_spanned!(f.span()=> self.#name.#trait_fn_name())
    });
    aggregate_fields(i, fields.len())
  }
}

fn fields_fns(trait_fn_name: &Ident, data: &Data, has_input: bool) -> TokenStream {
  match *data {
    Data::Struct(ref data) => match data.fields {
      Fields::Named(ref fields) => {
        construct_fields_fns(&fields.named, trait_fn_name, has_input, |_, f| {
          f.ident.clone().unwrap().into_token_stream()
        })
      }
      Fields::Unnamed(ref fields) => {
        construct_fields_fns(&fields.unnamed, trait_fn_name, has_input, |idx, _| {
          Index::from(idx).into_token_stream()
        })
      }
      Fields::Unit => {
        unimplemented!();
      }
    },
    Data::Enum(_) | Data::Union(_) => unimplemented!(),
  }
}

fn fields_types(trait_name: &Ident, trait_assoc_name: &str, data: &Data) -> TokenStream {
  let trait_assoc_ret_ident = Ident::new(trait_assoc_name, Span::call_site());
  match *data {
    Data::Struct(ref data) => match data.fields {
      Fields::Named(ref fields) => aggregate_fields(
        fields.named.iter().map(|f| {
          let ty = &f.ty;
          quote_spanned!(f.span()=> <#ty as #trait_name>::#trait_assoc_ret_ident)
        }),
        fields.named.len(),
      ),
      Fields::Unnamed(ref fields) => aggregate_fields(
        fields.unnamed.iter().map(|f| {
          let ty = &f.ty;
          quote_spanned!(f.span()=> <#ty as #trait_name>::#trait_assoc_ret_ident)
        }),
        fields.unnamed.len(),
      ),
      Fields::Unit => {
        unimplemented!();
      }
    },
    Data::Enum(_) | Data::Union(_) => unimplemented!(),
  }
}

create_trait!(
  WithCapacity,
  derive_with_capacity,
  Capacity,
  Output,
  capacity(&Self) -> Self::Output,
  false
);
create_trait!(
  WithClear,
  derive_with_clear,
  Clear,
  Output,
  clear(&mut Self) -> Self::Output,
  false
);
create_trait!(
  WithLength,
  derive_with_length,
  Length,
  Output,
  length(&Self) -> Self::Output,
  false
);
create_trait!(
  WithPush,
  derive_with_push,
  Push,
  Input,
  Output,
  push(&mut Self, Self::Input) -> Self::Output
);
create_trait!(
  WithSwap,
  derive_with_swap,
  Swap,
  Input,
  Output,
  swap(&mut Self, Self::Input) -> Self::Output
);
create_trait!(
  WithTruncate,
  derive_with_truncate,
  Truncate,
  Input,
  Output,
  truncate(&mut Self, Self::Input) -> Self::Output
);
