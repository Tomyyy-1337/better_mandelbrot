extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, FnArg, ItemFn, ReturnType, Stmt, Token, Type};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn worker_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let state_param: FnArg = syn::parse_quote! {
        state: &State
    };
    input_fn.sig.inputs.push(state_param);

    if let ReturnType::Type(_, ref mut ty) = input_fn.sig.output {
        let original_type = ty.as_ref();
        let wrapped_type: Type = syn::parse_quote! {
            Option<#original_type>
        };
        *ty = Box::new(wrapped_type);
    }
    
    if let Some(last_stmt) = input_fn.block.stmts.last_mut() {
        match last_stmt {
            Stmt::Expr(expr, None) => {
                *last_stmt = Stmt::Expr(wrap_in_some(expr.clone()), None);
            }
            Stmt::Expr(expr, Some(_)) => {
                *last_stmt = Stmt::Expr(wrap_in_some(expr.clone()), Some(Token![;](expr.span())));
            }
            _ => {}
        }
    }

    let output = quote! {
        #input_fn
    };

    output.into()
}

fn wrap_in_some(expr: Expr) -> Expr {
    syn::parse_quote! {
        Some(#expr)
    }
}