use crate::Input;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote as q;
use syn::Ident;

pub struct RustExternParams(Vec<TokenStream2>);
pub struct RustTransforms(Vec<TokenStream2>);
pub struct RustArgs(Vec<Ident>);

impl From<&Vec<Input>> for RustExternParams {
  fn from(inputs: &Vec<Input>) -> Self {
    let mut stream = vec![];

    for input in inputs {
      let variable = Ident::new(&input.variable, Span::call_site());
      let c_type = rust_c_type(&input.rust_type);
      stream.push(q!(#variable: #c_type))
    }

    Self(stream)
  }
}

impl From<&Vec<Input>> for RustTransforms {
  fn from(inputs: &Vec<Input>) -> Self {
    let mut stream = vec![];

    for input in inputs {
      let variable = Ident::new(&input.variable, Span::call_site());
      let cast = cast_c_type_to_rust(&input.rust_type, &input.variable);
      stream.push(q!(let #variable = #cast;))
    }

    Self(stream)
  }
}

impl From<&Vec<Input>> for RustArgs {
  fn from(inputs: &Vec<Input>) -> Self {
    let mut stream = vec![];

    for input in inputs {
      stream.push(Ident::new(&input.variable, Span::call_site()))
    }

    Self(stream)
  }
}

impl From<RustExternParams> for Vec<TokenStream2> {
  fn from(types: RustExternParams) -> Self {
    types.0
  }
}

impl From<RustTransforms> for Vec<TokenStream2> {
  fn from(types: RustTransforms) -> Self {
    types.0
  }
}

impl From<RustArgs> for Vec<Ident> {
  fn from(types: RustArgs) -> Self {
    types.0
  }
}

fn rust_c_type(ty: &str) -> TokenStream2 {
  match ty {
    "String" => q!(*const ::std::os::raw::c_char),
    "i64" => q!(::std::os::raw::c_long),
    "f64" => q!(::std::os::raw::c_double),
    "bool" => q!(::std::os::raw::c_char), // u8
    _ => panic!("C type {} not yet supported", ty),
  }
}

fn cast_c_type_to_rust(ty: &str, variable: &str) -> TokenStream2 {
  match ty {
    "String" => {
      let variable = Ident::new(variable, Span::call_site());
      q!(cstr!(#variable).to_string())
    }
    "i64" => {
      let variable = Ident::new(variable, Span::call_site());
      q!(#variable)
    }
    "f64" => {
      let variable = Ident::new(variable, Span::call_site());
      q!(#variable)
    }
    "bool" => {
      let variable = Ident::new(variable, Span::call_site());
      q!(#variable != 0)
    }

    _ => panic!("casting C type {} not yet supported", ty),
  }
}
