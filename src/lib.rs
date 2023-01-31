pub mod process;

#[macro_export]
macro_rules! Abc {
    (say $e:expr) => {
        println!("{}", stringify!($e));
    };

    (say $e:expr, $(say $es:expr),+) => {
        Abc! {say $e}
        Abc! { $(say $es),+ }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_test() {
        Abc! {
            say 1,
            say 2,
            say "three"
        };
    }
}
