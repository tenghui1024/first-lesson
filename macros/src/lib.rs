mod builder;
mod builder_with_attr;
mod raw_builder;

use proc_macro::TokenStream;
use raw_builder::BuilderContext;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    //println!("{:#?}", input);
    //TokenStream::default()
    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    // 解析输入的tokens到一个语法树
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    // TokenStream::default()
    builder::BuilderContext::from(input).render().into()
}

#[proc_macro_derive(BuilderWithAttr, attributes(builder))]
pub fn derive_builder_with_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder_with_attr::BuilderContext::from(input)
        .render()
        .into()
}
