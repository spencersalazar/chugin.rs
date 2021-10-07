
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
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
