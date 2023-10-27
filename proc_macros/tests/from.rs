#[cfg(test)]
mod test {
    use proc_macros::From;

    #[test]
    fn simple_expansion() {
        #[derive(From, Debug, PartialEq)]
        struct Foo {
            name: String,
        }

        assert_eq!(
            Foo::from(String::from("Hello")),
            Foo {
                name: String::from("Hello")
            }
        );
    }
}
