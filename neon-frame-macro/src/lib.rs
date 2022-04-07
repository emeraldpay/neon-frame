#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{parse_macro_input, ReturnType};
use syn::{ItemFn, Ident};
use syn::parse::{ParseStream, Parse};
use syn::punctuated::{Punctuated};
use proc_macro::{TokenStream };
use proc_macro2::{ Span };

struct Args {
    use_channel: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            use_channel: false
        }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let use_channel = vars.iter().any(|v| v.to_string().eq(&"channel".to_string()));
        Ok(Args {
            use_channel,
            ..Args::default()
        })
    }
}

#[proc_macro_attribute]
pub fn neon_frame_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args= parse_macro_input!(attr as Args);
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // here we change the original function to an internal name and replace with our own which actually follows the Neon requrements
    let orignal_name = input_fn.sig.ident;
    let new_name = Ident::new(&format!("{}_nfi", orignal_name), Span::call_site());
    input_fn.sig.ident = new_name.clone();

    let process_handle = if args.use_channel {
        quote! {
            let arg = &cx.argument::<neon::types::JsFunction>(0)?;
            let handler = neon::prelude::JsFunction::root(arg, &mut cx);
            let queue = neon::prelude::Context::channel(&mut cx);

            let internal_handler = move |result| {
                let result = neon_frame::StatusResult::from(result).as_json();
                queue.send(move |mut cx| {
                    let callback = handler.into_inner(&mut cx);
                    let this = neon::prelude::Context::undefined(&mut cx);
                    let args: Vec<neon::prelude::Handle<neon::types::JsValue>> = vec![
                        neon::prelude::Context::string(&mut cx, result).upcast()
                    ];
                    callback.call(&mut cx, this, args)?;
                    Ok(())
                });
            };
        }
    } else {
        quote! {}
    };

    let get_result = if args.use_channel {
        quote! {
            let result = #new_name(&mut cx, internal_handler);
        }
    } else {
        quote! {
            let result = #new_name(&mut cx);
        }
    };

    TokenStream::from(quote!{
        #input_fn

        pub fn #orignal_name(mut cx: neon::prelude::FunctionContext) -> neon::prelude::JsResult<neon::types::JsString> {
            #process_handle
            #get_result

            let result = neon_frame::StatusResult::from(result);

            Ok(neon::prelude::Context::string(&mut cx, result.as_json()))
        }
    })
}
