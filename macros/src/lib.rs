
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
    
#[proc_macro_derive(Chugin)]
pub fn chugin_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_chugin_macro(&ast)
}

fn impl_chugin_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                let b: chuck::t_CKBOOL = chuck::CK_TRUE;
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn query_fn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    
    let input_fn = parse_macro_input!(input as ItemFn);
    let input_fn_name = input_fn.sig.ident;
    
    output.extend(TokenStream::from(quote! {
            #[no_mangle]
            pub extern "C" fn ck_version() -> chuck::t_CKUINT {
                chugin::version()
            }

            #[no_mangle]
            pub extern "C" fn ck_query(query: *mut chuck::DL_Query) -> chuck::t_CKBOOL {    
                match #input_fn_name (query) {
                    Ok(_) => chuck::CK_TRUE,
                    Err(_) => chuck::CK_FALSE,
                }
            }
        })
    );
    
    output
}
