#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! love_rust {
        ($s: expr) => {
            match $s {
                "" => format!("powered with <3 by Rust"),
                _  => format!("{}: powered with <3 by Rust", $s)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(love_rust!(""), "powered with <3 by Rust");
    }

    #[test]
    fn not_empty() {
        assert_eq!(love_rust!("Rust"), "Rust: powered with <3 by Rust");
        assert_eq!(love_rust!("oxy"), "oxy: powered with <3 by Rust");
        assert_eq!(love_rust!("rust"), "rust: powered with <3 by Rust");
    }
}
