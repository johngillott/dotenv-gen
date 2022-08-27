#[macro_use]
extern crate serde;

use std::fmt;

macro_rules! gen {
    ($name:ident {$($field:ident : $t:ty),+}) => {
        struct $name { $($field: $t),+ }
        impl $name {
            fn field_count(&self) -> usize {
                gen!(@count $($field),+)
            }
        }
    };
    (@count $t1:tt, $($t:tt),+) => { 1 + gen!(@count $($t),+) };
    (@count $t:tt) => { 1 };
}

gen! {Test1 { _num: i32 }}
// TODO add default mod
// https://github.com/NandosUK/nandos-platform/pull/1859/files#diff-57e28444be09e7bd07913c5965f1595d9cf75cb02e6584b9dec345a0980c5b3d

#[derive(Deserialize)]
pub struct Config {
    // #[serde(default = "defaults::port")]
    pub port: u16,

    pub api_url: String,
    pub api_token: Option<String>,
    pub api_active: bool,

    #[serde(
        rename = "foobar_expiry",
        // default = "defaults::foobar_expiry_in_minutes"
    )]
    pub foobar_expiry_in_minutes: i64,
}

macro_rules! MyDisplay {
    ($struct:ident {$( $field:ident:$type:ty ),*,}) => {
        #[derive(Debug)]
        pub struct $struct { pub $($field: $type),*}

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $(
                    write!(f, "{}: {}\n",
                        stringify!($field).to_string(),
                        match &self.$field {
                            None => "-".to_string(),
                            Some(x) => format!("{:#?}", x)
                        }
                    )?;
                )*
                Ok(())
            }
        }
    };
}

macro_rules! MyEnv {
    ($struct:ident {$( $field:ident:$type:ty ),*,}) => {
        #[derive(Debug)]
        pub struct $struct { pub $($field: $type),*}

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $(
                    write!(f, "{}: {}\n",
                        stringify!($field).to_string(),
                        match &self.$field {
                            None => "-".to_string(),
                            Some(x) => format!("{:#?}", x)
                        }
                    )?;
                )*
                Ok(())
            }
        }
    };
}

MyDisplay! {
    Example {
        a: Option<String>,
        b: Option<i64>,
        c: Option<String>,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;

        let e = Example {
            a: Some("test".to_string()),
            b: Some(123),
            c: None,
        };

        println!("{}", e);

        assert_eq!(result, 4);
    }
}
