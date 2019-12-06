pub mod key {
    use log::kv::*;

    pub fn key_from_string() {
        assert_eq!("a key", Key::from_str("a key").as_str());
    }
}

pub mod source {
    use log::kv::*;

    pub fn source_is_object_safe() {
        fn _check(_: &Source) {}
    }

    pub fn visitor_is_object_safe() {
        fn _check(_: &Visitor) {}
    }
}

pub mod value {
    use log::kv::*;
    //use log::kv::Error;
    //use log::kv::value::Visitor;

    use std::prelude::v1::*;
    use std::fmt;
    use std::fmt::Write;
    use std::str::{self, Utf8Error};

    // A quick-and-dirty no-std buffer
    // to write strings into
    struct Buffer {
        buf: [u8; 16],
        len: usize,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer {
                buf: [0; 16],
                len: 0,
            }
        }

        fn as_str(&self) -> Result<&str, Utf8Error> {
            str::from_utf8(&self.buf[0..self.len])
        }
    }

    impl Write for Buffer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let bytes = s.as_bytes();

            let end = self.len + bytes.len();

            if end > 16 {
                panic!("`{}` would overflow", s);
            }

            let buf = &mut self.buf[self.len..end];
            buf.copy_from_slice(bytes);
            self.len = end;

            Ok(())
        }
    }

    pub fn test_to_value_display() {
        // Write a value into our buffer using `<Value as Display>::fmt`
        fn check(value: Value, expected: &str) {
            let mut buf = Buffer::new();
            write!(&mut buf, "{}", value).unwrap();

            assert_eq!(expected, buf.as_str().unwrap());
        }

        check(42u64.to_value(), "42");
        check(42i64.to_value(), "42");
        check(42.01f64.to_value(), "42.01");
        check(true.to_value(), "true");
        check('a'.to_value(), "'a'");
        check(format_args!("a {}", "value").to_value(), "a value");
        check("a loong string".to_value(), "\"a loong string\"");
        check(Some(true).to_value(), "true");
        check(().to_value(), "None");
        check(Option::None::<bool>.to_value(), "None");
    }

    //pub fn test_to_value_structured() {
    //    #[derive(Debug, PartialEq)]
    //    enum Token<'a> {
    //        U64(u64),
    //        I64(i64),
    //        F64(f64),
    //        Char(char),
    //        Bool(bool),
    //        Str(&'a str),
    //        None,
    //    }

    //    struct TestVisitor<F>(F);

    //    impl<F> Visitor for TestVisitor<F>
    //    where
    //        F: Fn(Token),
    //    {
    //        fn debug(&mut self, v: &fmt::Debug) -> Result<(), Error> {
    //            let mut buf = Buffer::new();
    //            write!(&mut buf, "{:?}", v)?;

    //            let s = buf.as_str().map_err(|_| Error::msg("invalid UTF8"))?;
    //            (self.0)(Token::Str(s));
    //            Ok(())
    //        }

    //        fn u64(&mut self, v: u64) -> Result<(), Error> {
    //            (self.0)(Token::U64(v));
    //            Ok(())
    //        }

    //        fn i64(&mut self, v: i64) -> Result<(), Error> {
    //            (self.0)(Token::I64(v));
    //            Ok(())
    //        }

    //        fn f64(&mut self, v: f64) -> Result<(), Error> {
    //            (self.0)(Token::F64(v));
    //            Ok(())
    //        }

    //        fn bool(&mut self, v: bool) -> Result<(), Error> {
    //            (self.0)(Token::Bool(v));
    //            Ok(())
    //        }

    //        fn char(&mut self, v: char) -> Result<(), Error> {
    //            (self.0)(Token::Char(v));
    //            Ok(())
    //        }

    //        fn str(&mut self, v: &str) -> Result<(), Error> {
    //            (self.0)(Token::Str(v));
    //            Ok(())
    //        }

    //        fn none(&mut self) -> Result<(), Error> {
    //            (self.0)(Token::None);
    //            Ok(())
    //        }
    //    }

    //    // Check that a value retains the right structure
    //    fn check(value: Value, expected: Token) {
    //        let mut visitor = TestVisitor(|token: Token| assert_eq!(expected, token));
    //        value.visit(&mut visitor).unwrap();
    //    }

    //    check(42u64.to_value(), Token::U64(42));
    //    check(42i64.to_value(), Token::I64(42));
    //    check(42.01f64.to_value(), Token::F64(42.01));
    //    check(true.to_value(), Token::Bool(true));
    //    check('a'.to_value(), Token::Char('a'));
    //    check(format_args!("a {}", "value").to_value(), Token::Str("a value"));
    //    check("a loong string".to_value(), Token::Str("a loong string"));
    //    check(Some(true).to_value(), Token::Bool(true));
    //    check(().to_value(), Token::None);
    //    check(Option::None::<bool>.to_value(), Token::None);
    //}
}
