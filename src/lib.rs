extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[allow(non_snake_case)]
/*
pub fn mimpl2(token_stream: TokenStream) -> TokenStream {
    struct TokenIter {
        stream: String,
    }

    impl TokenIter {
        fn get_trait(&mut self) -> String {
            self.stream = self.stream.trim_start().into();
            self.stream.split_off(self.stream.find(';').unwrap_or(0))
        }
        fn get_next(&mut self) -> String {
            self.stream = self.stream.trim_start().into();
            self.stream.split_off(self.stream.find(',').unwrap_or(0))
        }
        fn get_rest(self) -> String {
            self.stream.trim_start().into()
        }
    }

    let mut iter = TokenIter {
        stream: token_stream.to_string().trim().into(),
    };

    let expand = match &*iter.get_trait() {
        "Default" => {
            let TYPE = iter.get_next();
            let CLOSURE = iter.get_rest();
            quote! {
                impl Default for #TYPE {
                    fn default() -> Self {
                        #CLOSURE()
                    }
                }
            }
        },
        "From" => {
            let FROM = iter.get_next();
            let TO = iter.get_next();
            let CLOSURE = iter.get_rest();
            quote! {
                impl From<#FROM> for #TO {
                    fn from(from: #FROM) -> Self {
                        #CLOSURE(from)
                    }
                }
            }
        }
        (TryFrom; $from:path, $type:path, $error:ty, $closure:expr) => {
            impl TryFrom<$from> for $type {
                type Error = $error;
                fn try_from(from: $from) -> Result<Self, Self::Error> {
                    $closure(from)
                }
            }
        };
        (Ord; $type:path, $closure:expr) => {
            impl Ord for $type {
                fn cmp(&self, other: &Self) -> Ordering {
                    $closure(self, other)
                }
            }
        };
        (PartialOrd; $type:path, USE_CMP) => {
            impl PartialOrd for $type {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }
        };
        (PartialOrd; $type:path, $closure:expr) => {
            impl PartialOrd for $type {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    $closure(self, other)
                }
            }
        };
        _ => quote! {compile_error!("Bad!")},
    };

    TokenStream::from(expand)
}*/
#[macro_export]

macro_rules! mimpl {
    (Derive; $( $derivable:expr ),+ ) => {
        #[derive( $($derivable),+) ]
    };
    (Default; $type:path, $closure:expr) => {
        impl std::default::Default for $type {
            fn default() -> Self {
                $closure()
            }
        }
    };
    (Display; $type:path, $closure:expr) => { // Bad
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $closure(self, f)
            }
        }
    }
    (From; $from:path, $type:path, $closure:expr) => {
        impl std::convert::From<$from> for $type {
            fn from(from: $from) -> Self {
                $closure(from)
            }
        }
    };
    (TryFrom; $from:path, $type:path, $error:ty, $closure:expr) => {
        impl std::convert::TryFrom<$from> for $type {
            type Error = $error;
            fn try_from(from: $from) -> Result<Self, Self::Error> {
                $closure(from)
            }
        }
    };
    (Iterator; $type:path, $item:path, $closure:expr) => {
        impl std::iter::Iterator for $type {
            type Item = $item;

            fn next(&mut self) -> Self::Item {
                $closure(self)
            }
        }
    };
    (Ord; $type:path, $closure:expr) => {
        impl std::cmp::Ord for $type {
            fn cmp(&self, other: &Self) -> Ordering {
                $closure(self, other)
            }
        }
    };
    (PartialOrd; $type:path) => {
        impl std::cmp::PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
    (PartialOrd; $type:path, $closure:expr) => {
        impl std::cmp::PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                $closure(self, other)
            }
        }
    };
}
