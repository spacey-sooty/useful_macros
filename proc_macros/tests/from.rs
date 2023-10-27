#[cfg(test)]
mod test {
    use proc_macros::From;

    #[test]
    fn small_expansion() {
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

    #[test]
    fn large_expansion() {
        #[derive(PartialEq, Debug)]
        enum Bar {
            Foo(String),
            Bar(bool),
        }
        #[derive(From, Debug, PartialEq)]
        struct Foo {
            name: String,
            foo: i32,
            bar: Bar,
        }

        assert_eq!(
            Foo::from(String::from("Hello"), 7, Bar::Bar(true)),
            Foo {
                name: String::from("Hello"),
                foo: 7,
                bar: Bar::Bar(true)
            }
        );
        assert_eq!(
            Foo::from(String::from("Hello"), 7, Bar::Foo(String::from("FooBar"))),
            Foo {
                name: String::from("Hello"),
                foo: 7,
                bar: Bar::Foo(String::from("FooBar"))
            }
        );
    }
}
