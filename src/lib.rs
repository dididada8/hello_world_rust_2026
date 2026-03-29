//! Core greeting logic for the helloworld binary.

/// Returns the default greeting string.
pub fn greeting() -> &'static str {
    "Hello, world!"
}

/// Prints the concrete Rust type name of a value with an optional prefix.
pub fn print_type_of<T>(_: &T, prefix: Option<&str>) {
    match prefix {
        Some(p) => println!("{p}: {}", std::any::type_name::<T>()),
        None => println!("{}", std::any::type_name::<T>()),
    }
}

pub fn print_line_separator() {
    println!("{}\n","-".repeat(60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_expected() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
