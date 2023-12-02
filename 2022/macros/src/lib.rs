use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn include_str_arr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let file = File::open(input).expect("file could not be found");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.expect("could not parse line"))
        .map(|line| {
            if line.is_empty() {
                format!("Option::None")
            } else {
                format!("Option::Some(\"{}\")", line)
            }
        })
        .collect::<Vec<_>>();
    let arr_str = format!("&[{}]", lines.join(", "));
    arr_str.parse().expect("could not parse token stream")
}
