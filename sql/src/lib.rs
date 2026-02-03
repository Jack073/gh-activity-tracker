extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;
use std::str::FromStr;

#[proc_macro]
pub fn load_queries(_: TokenStream) -> TokenStream {
    TokenStream::from_str(
        &fs::read_dir("./sql/queries/")
            .expect("SQL query folder read error")
            .map(|f| match f {
                Ok(f) => {
                    let body = fs::read_to_string(f.path())
                        .expect(&format!("sql read error: {}", f.path().to_str().unwrap()));

                    format!(
                        "pub const {}: &str = \"{}\";",
                        f.file_name()
                            .to_ascii_uppercase()
                            .to_str()
                            .unwrap()
                            .replace(" ", "_")
                            .trim_end_matches(".SQL"),
                        body.split_ascii_whitespace()
                            .into_iter()
                            .map(|s| s.to_owned())
                            .reduce(|acc, e| acc.to_owned() + " " + &e)
                            .unwrap_or("".to_owned())
                            .replace("\"", "\\\"")
                    )
                }
                Err(e) => panic!("unable to load SQL query: {}", e),
            })
            .collect::<String>(),
    )
    .unwrap()
}