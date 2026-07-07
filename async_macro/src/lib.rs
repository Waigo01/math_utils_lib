use proc_macro::{TokenStream};
#[cfg(feature = "async_attr")]
use proc_macro::{Delimiter, Group, TokenTree};
#[cfg(feature = "async_attr")]
use quote::ToTokens;
#[cfg(feature = "async_attr")]
use proc_macro2::Span;
#[cfg(feature = "async_attr")]
use syn::{Block, Expr, ExprCall, ExprMethodCall, Ident, ItemFn, parse, parse_macro_input, token::Async};

#[proc_macro]
pub fn async_call(item: TokenStream) -> TokenStream {
    item
}

#[cfg(feature = "async_attr")]
fn replace_async_calls(stream: TokenStream) -> TokenStream {
    let mut new_stream = vec![];
    let mut state = 0;
    for token in stream {
        match (state, token) {
            (0, TokenTree::Ident(ident)) if ident.to_string() == "async_call" => {
                state = 1;
                new_stream.push(TokenTree::Ident(ident));
            },
            (1, TokenTree::Punct(punct)) if punct.as_char() == '!' => {
                state = 2;
                new_stream.push(TokenTree::Punct(punct));
            },
            (2, TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                if let Ok(mut call) = parse::<ExprCall>(group.stream()) && let Expr::Path(mut path) = *call.func && let Some(segment) = path.path.segments.last_mut() {
                    segment.ident = Ident::new(&format!("{}_async", segment.ident), Span::call_site());
                    call.func = Box::new(Expr::Path(path));
                    for arg in call.args.iter_mut() {
                        *arg = parse::<Expr>(replace_async_calls(TokenStream::from(arg.to_token_stream()))).unwrap()
                    }
                    new_stream.append(&mut format!("(Box::pin({}).await)", call.to_token_stream().to_string()).parse::<TokenStream>().unwrap().into_iter().collect());
                } else if let Ok(mut call) = parse::<ExprMethodCall>(group.stream()) {
                    call.method = Ident::new(&format!("{}_async", call.method), Span::call_site());
                    for arg in call.args.iter_mut() {
                        *arg = parse::<Expr>(replace_async_calls(TokenStream::from(arg.to_token_stream()))).unwrap()
                    }
                    new_stream.append(&mut format!("(Box::pin({}).await)", call.to_token_stream().to_string()).parse::<TokenStream>().unwrap().into_iter().collect());
                } else {
                    new_stream.push(TokenTree::Group(Group::new(group.delimiter(), replace_async_calls(group.stream()))));
                }
                state = 0;
            },
            (0, TokenTree::Group(group)) => {
                new_stream.push(TokenTree::Group(Group::new(group.delimiter(), replace_async_calls(group.stream()))));
            },
            (0, t) => new_stream.push(t),
            _ => state = 0
        }
    }

    TokenStream::from_iter(new_stream)
}

#[cfg(feature = "async_attr")]
#[proc_macro_attribute]
pub fn function_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut return_item = item.clone();

    let item = replace_async_calls(item);

    let mut func = parse_macro_input!(item as ItemFn);

    if func.sig.asyncness.is_none() {
        func.sig.asyncness = Some(Async::default());
        func.sig.ident = Ident::new(&format!("{}_async", func.sig.ident), Span::call_site());
    }

    let async_yield = r"{
        async fn yield_async() {
            let mut yielded = false;
            std::future::poll_fn(|cx| {
                if yielded {
                    return std::task::Poll::Ready(());
                }

                yielded = true;

                cx.waker().wake_by_ref();

                std::task::Poll::Pending
            }).await;
        }

        yield_async().await;
    }";

    let new_block = parse::<Block>(async_yield.parse().unwrap()).unwrap();

    for stmt in new_block.stmts.iter().rev() {
        func.block.stmts.insert(0, stmt.clone());
    }

    #[cfg(feature = "wasm")]
    {
        let async_yield_wasm = r#"{
            async fn yield_async_wasm() {
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_name = queueMicrotask)]
                    fn queue_microtask(callback: &js_sys::Function);
                }
                let promise = js_sys::Promise::new(&mut |resolve, _reject| {
                    setTimeout(&resolve, 0.);
                });
                let _ = promise.await;
            }

            yield_async_wasm().await;
        }"#;

        let new_block = parse::<Block>(async_yield_wasm.parse().unwrap()).unwrap();

        for stmt in new_block.stmts.iter().rev() {
            func.block.stmts.insert(0, stmt.clone());
        }
    }

    return_item.extend(TokenStream::from(func.to_token_stream()));

    return_item
}
