extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn json_type(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let s = s
        .split('.')
        .map(|s| {
            let mut s = s.chars().collect::<Vec<_>>();

            s[0] = s[0].to_uppercase().nth(0).unwrap();
            s.into_iter().collect::<String>()
        })
        .collect::<String>();

    s.parse().unwrap()
}

#[proc_macro]
pub fn n_try(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let s = s.split(',').collect::<Vec<_>>();

    let n = s[1].trim().parse().unwrap();
    let t = vec!['?'; n].into_iter().collect::<String>();

    format!("{}{}", s[0], t).parse().unwrap()
}
