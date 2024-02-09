use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Block, FnArg, ItemFn, Lifetime, Meta, MetaList, Pat, Path, Type,
};

fn find_validate_attr(attributes: &Vec<Attribute>) -> Option<usize> {
    attributes
        .iter()
        .enumerate()
        .find(|(_ind, attr)| {
            let Meta::List(MetaList { path, .. }) = &attr.meta else {
                return false;
            };
            let name: Vec<_> = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            let name = name.join("::");
            name == "validate" || name == "validator::validate"
        })
        .map(|(ind, _attr)| ind)
}

#[proc_macro_attribute]
pub fn auto_validate(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func: ItemFn = parse_macro_input!(input);
    let mut args = Vec::new();
    let mut types = Vec::new();
    let mut validations = Vec::new();
    for input in &mut func.sig.inputs {
        let FnArg::Typed(arg) = input else { continue };
        let Pat::Ident(ident) = &*arg.pat else {
            continue;
        };
        let Some(index) = find_validate_attr(&arg.attrs) else {
            continue;
        };
        let attr = arg.attrs.remove(index);
        args.push(ident.clone());
        if let Type::Reference(inner) = &*arg.ty {
            let mut inner = inner.clone();
            inner.lifetime = Some(Lifetime::new("'a", Span::call_site()));
            types.push(Type::Reference(inner));
        } else {
            types.push(*(arg.ty).clone());
        }
        validations.push(attr);
    }
    let body = &func.block;
    let validation = quote! {
        {
            #[derive(validator::Validate)]
            struct __validate<'a> {
                #(
                    #validations
                    #args: #types
                ),*
            };
            let __validation = __validate { #(#args),* };
            validator::Validate::validate(&__validation)?;
            let __validate { #(#args),* } = __validation;
            #body
        }
    };
    let block: Block = syn::parse(validation.into()).unwrap();
    func.block = Box::new(block);
    quote! {
        #func
    }
    .into()
}
