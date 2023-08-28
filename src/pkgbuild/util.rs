use std::fmt;

#[macro_export]
macro_rules! string_vec {
    ( $( $str:expr ),* ) => {
        {
            vec![ $( $str.to_string(), )* ]
        }
    };
}

//TODO clean up and make robust because bash is evil

pub fn bash_val(f: &mut fmt::Formatter, key: &str, val: &str) -> fmt::Result {
    writeln!(f, "{key}={val:?}")
}

pub fn bash_val_opt(f: &mut fmt::Formatter, key: &str, val: Option<String>) -> fmt::Result {
    match val {
        Some(val) => bash_val(f, key, &val),
        None => Ok(()),
    }
}

pub fn bash_arr(f: &mut fmt::Formatter, key: &str, vals: Option<Vec<String>>) -> fmt::Result {
    writeln!(
        f,
        "{key}=({})",
        vals.unwrap_or(vec![])
            .iter()
            .map(|val| format!("{val:?}"))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

pub fn bash_assign(key: &str, val: &str) -> String {
    format!("{key}={val:?}")
}
