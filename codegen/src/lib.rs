extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    eprintln!("_item1:{:?}", _item);
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    eprintln!("_item:{:?}", _item.to_string());
    "fn answer1() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    eprintln!("_item:{:?}", _item.to_string());
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn show_streams(attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr_ts.to_string());
    let mut fn_ts_str = fn_ts.to_string();
    println!("item: \"{}\"", &fn_ts_str);
    let start_fn_pos = fn_ts_str.find('{').unwrap_or(0);

    fn_ts_str.insert_str(start_fn_pos + 1, " let now:u32 = before();");

    println!("item: \"{}\"", fn_ts_str);

    // let fn_name = &fn_ts.clone().to_string()[0..fn_ts.clone().to_string().find('{').unwrap_or(0usize)];
    // println!("{:?}", fn_name);

    let size = fn_ts_str.rfind('}').unwrap_or(fn_ts_str.len() - 1);

    fn_ts_str.insert_str(size, "  println!(\"{}--\", now);after();");

    let s2 = "pub fn abc() { println!(\"{}\", \"khuyen cheat\");}".to_string();

    let result = format!("{}\n{}", s2, fn_ts_str);
    println!("new_fn----->>>>>>>>>>>>>>>>{:?}", result);
    let a = TokenStream::from_str(fn_ts_str.as_ref()).unwrap();

    a
}

#[proc_macro_attribute]
pub fn validate_request(_attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream {
    let mut fn_ts_str = fn_ts.to_string();

    let check = r#"
        if !cfg!(test) {
            let mut request: &str = "reject";
            if let Some(value) = req.headers().get("valid_request") {
                request = value.to_str().unwrap_or("");
            }
            
            if request != "accepted" {
                return Err(ApiError::new(
                    403,
                    Messages::USER_NOT_PERMISSION.to_string(),
                    ErrorCodes::USER_NOT_PERMISSION,
                    Some(Messages::USER_NOT_PERMISSION.to_string()),
                    None,
                ));
            }
        }
        "#;
    // let check = r#"println!("{:?}", req.headers().get("valid_request"));"#;

    let start_fn_pos = fn_ts_str.find('{').unwrap_or(0);
    fn_ts_str.insert_str(start_fn_pos + 1, check);

    TokenStream::from_str(fn_ts_str.as_ref()).unwrap()
}
