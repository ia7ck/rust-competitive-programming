use proc_macro::TokenStream;
use quote::quote;

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn insert_scanner(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut main_func = match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(func) => {
            if func.sig.ident == "main" {
                func
            } else {
                return token_stream_with_error(
                    item,
                    syn::Error::new(func.sig.ident.span(), "expected `main`"),
                );
            }
        }
        Err(e) => return token_stream_with_error(item, e),
    };
    let top = match syn::parse2::<syn::Block>(quote! {
        {
            let __stdin = std::io::stdin();
            let mut __scanner = scanner::Scanner::from(__stdin.lock());

            #[allow(unused_macros)]
            macro_rules! scan {
                (($($t: ty),+)) => {
                    ($(scan!($t)),+)
                };
                ($t: ty) => {
                    __scanner.scan::<$t>() as $t
                };
                (($($t: ty),+); $n: expr) => {
                    std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
                };
                ($t: ty; $n: expr) => {
                    std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
                };
            }
        }
    }) {
        Ok(block) => block,
        Err(e) => return token_stream_with_error(item, e),
    };
    let mut stmts = Vec::new();
    stmts.extend(top.stmts);
    stmts.extend(main_func.block.stmts);
    main_func.block.stmts = stmts;
    let output = quote! {
        #main_func
    };
    output.into()
}
