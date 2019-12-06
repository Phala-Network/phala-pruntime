//use serde_test::{assert_tokens, Configure, Token};

use std::prelude::v1::*;
use std::net;

use serde_test::*;

//#[test]
pub fn ip_addr_roundtrip() {
    assert_tokens(
        &net::IpAddr::from(*b"1234").compact(),
        &seq![
            Token::NewtypeVariant { name: "IpAddr", variant: "V4" },

            Token::Tuple { len: 4 },
            seq b"1234".iter().map(|&b| Token::U8(b)),
            Token::TupleEnd,
        ],
    );
}

//#[test]
pub fn socket_addr_roundtrip() {
    assert_tokens(
        &net::SocketAddr::from((*b"1234567890123456", 1234)).compact(),
        &seq![
            Token::NewtypeVariant { name: "SocketAddr", variant: "V6" },

            Token::Tuple { len: 2 },

            Token::Tuple { len: 16 },
            seq b"1234567890123456".iter().map(|&b| Token::U8(b)),
            Token::TupleEnd,

            Token::U16(1234),
            Token::TupleEnd,
        ],
    );
}
