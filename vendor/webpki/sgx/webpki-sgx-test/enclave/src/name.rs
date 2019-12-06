use webpki::name::*;

const PRESENTED_MATCHES_REFERENCE: &[(&[u8], &[u8], Option<bool>)] = &[
    (b"", b"a", None),
    (b"a", b"a", Some(true)),
    (b"b", b"a", Some(false)),
    (b"*.b.a", b"c.b.a", Some(true)),
    (b"*.b.a", b"b.a", Some(false)),
    (b"*.b.a", b"b.a.", Some(false)),
    // Wildcard not in leftmost label
    (b"d.c.b.a", b"d.c.b.a", Some(true)),
    (b"d.*.b.a", b"d.c.b.a", None),
    (b"d.c*.b.a", b"d.c.b.a", None),
    (b"d.c*.b.a", b"d.cc.b.a", None),
    // case sensitivity
    (
        b"abcdefghijklmnopqrstuvwxyz",
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        Some(true),
    ),
    (
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        b"abcdefghijklmnopqrstuvwxyz",
        Some(true),
    ),
    (b"aBc", b"Abc", Some(true)),
    // digits
    (b"a1", b"a1", Some(true)),
    // A trailing dot indicates an absolute name, and absolute names can match
    // relative names, and vice-versa.
    (b"example", b"example", Some(true)),
    (b"example.", b"example.", None),
    (b"example", b"example.", Some(true)),
    (b"example.", b"example", None),
    (b"example.com", b"example.com", Some(true)),
    (b"example.com.", b"example.com.", None),
    (b"example.com", b"example.com.", Some(true)),
    (b"example.com.", b"example.com", None),
    (b"example.com..", b"example.com.", None),
    (b"example.com..", b"example.com", None),
    (b"example.com...", b"example.com.", None),
    // xn-- IDN prefix
    (b"x*.b.a", b"xa.b.a", None),
    (b"x*.b.a", b"xna.b.a", None),
    (b"x*.b.a", b"xn-a.b.a", None),
    (b"x*.b.a", b"xn--a.b.a", None),
    (b"xn*.b.a", b"xn--a.b.a", None),
    (b"xn-*.b.a", b"xn--a.b.a", None),
    (b"xn--*.b.a", b"xn--a.b.a", None),
    (b"xn*.b.a", b"xn--a.b.a", None),
    (b"xn-*.b.a", b"xn--a.b.a", None),
    (b"xn--*.b.a", b"xn--a.b.a", None),
    (b"xn---*.b.a", b"xn--a.b.a", None),
    // "*" cannot expand to nothing.
    (b"c*.b.a", b"c.b.a", None),
    // --------------------------------------------------------------------------
    // The rest of these are test cases adapted from Chromium's
    // x509_certificate_unittest.cc. The parameter order is the opposite in
    // Chromium's tests. Also, they some tests were modified to fit into this
    // framework or due to intentional differences between mozilla::pkix and
    // Chromium.
    (b"foo.com", b"foo.com", Some(true)),
    (b"f", b"f", Some(true)),
    (b"i", b"h", Some(false)),
    (b"*.foo.com", b"bar.foo.com", Some(true)),
    (b"*.test.fr", b"www.test.fr", Some(true)),
    (b"*.test.FR", b"wwW.tESt.fr", Some(true)),
    (b".uk", b"f.uk", None),
    (b"?.bar.foo.com", b"w.bar.foo.com", None),
    (b"(www|ftp).foo.com", b"www.foo.com", None), // regex!
    (b"www.foo.com\0", b"www.foo.com", None),
    (b"www.foo.com\0*.foo.com", b"www.foo.com", None),
    (b"ww.house.example", b"www.house.example", Some(false)),
    (b"www.test.org", b"test.org", Some(false)),
    (b"*.test.org", b"test.org", Some(false)),
    (b"*.org", b"test.org", None),
    // '*' must be the only character in the wildcard label
    (b"w*.bar.foo.com", b"w.bar.foo.com", None),
    (b"ww*ww.bar.foo.com", b"www.bar.foo.com", None),
    (b"ww*ww.bar.foo.com", b"wwww.bar.foo.com", None),
    (b"w*w.bar.foo.com", b"wwww.bar.foo.com", None),
    (b"w*w.bar.foo.c0m", b"wwww.bar.foo.com", None),
    (b"wa*.bar.foo.com", b"WALLY.bar.foo.com", None),
    (b"*Ly.bar.foo.com", b"wally.bar.foo.com", None),
    // Chromium does URL decoding of the reference ID, but we don't, and we also
    // require that the reference ID is valid, so we can't test these two.
    //     (b"www.foo.com", b"ww%57.foo.com", Some(true)),
    //     (b"www&.foo.com", b"www%26.foo.com", Some(true)),
    (b"*.test.de", b"www.test.co.jp", Some(false)),
    (b"*.jp", b"www.test.co.jp", None),
    (b"www.test.co.uk", b"www.test.co.jp", Some(false)),
    (b"www.*.co.jp", b"www.test.co.jp", None),
    (b"www.bar.foo.com", b"www.bar.foo.com", Some(true)),
    (b"*.foo.com", b"www.bar.foo.com", Some(false)),
    (b"*.*.foo.com", b"www.bar.foo.com", None),
    // Our matcher requires the reference ID to be a valid DNS name, so we cannot
    // test this case.
    //     (b"*.*.bar.foo.com", b"*..bar.foo.com", Some(false)),
    (b"www.bath.org", b"www.bath.org", Some(true)),
    // Our matcher requires the reference ID to be a valid DNS name, so we cannot
    // test these cases.
    // DNS_ID_MISMATCH("www.bath.org", ""),
    //     (b"www.bath.org", b"20.30.40.50", Some(false)),
    //     (b"www.bath.org", b"66.77.88.99", Some(false)),

    // IDN tests
    (
        b"xn--poema-9qae5a.com.br",
        b"xn--poema-9qae5a.com.br",
        Some(true),
    ),
    (
        b"*.xn--poema-9qae5a.com.br",
        b"www.xn--poema-9qae5a.com.br",
        Some(true),
    ),
    (
        b"*.xn--poema-9qae5a.com.br",
        b"xn--poema-9qae5a.com.br",
        Some(false),
    ),
    (b"xn--poema-*.com.br", b"xn--poema-9qae5a.com.br", None),
    (b"xn--*-9qae5a.com.br", b"xn--poema-9qae5a.com.br", None),
    (b"*--poema-9qae5a.com.br", b"xn--poema-9qae5a.com.br", None),
    // The following are adapted from the examples quoted from
    //   http://tools.ietf.org/html/rfc6125#section-6.4.3
    // (e.g., *.example.com would match foo.example.com but
    // not bar.foo.example.com or example.com).
    (b"*.example.com", b"foo.example.com", Some(true)),
    (b"*.example.com", b"bar.foo.example.com", Some(false)),
    (b"*.example.com", b"example.com", Some(false)),
    (b"baz*.example.net", b"baz1.example.net", None),
    (b"*baz.example.net", b"foobaz.example.net", None),
    (b"b*z.example.net", b"buzz.example.net", None),
    // Wildcards should not be valid for public registry controlled domains,
    // and unknown/unrecognized domains, at least three domain components must
    // be present. For mozilla::pkix and NSS, there must always be at least two
    // labels after the wildcard label.
    (b"*.test.example", b"www.test.example", Some(true)),
    (b"*.example.co.uk", b"test.example.co.uk", Some(true)),
    (b"*.example", b"test.example", None),
    // The result is different than Chromium, because Chromium takes into account
    // the additional knowledge it has that "co.uk" is a TLD. mozilla::pkix does
    // not know that.
    (b"*.co.uk", b"example.co.uk", Some(true)),
    (b"*.com", b"foo.com", None),
    (b"*.us", b"foo.us", None),
    (b"*", b"foo", None),
    // IDN variants of wildcards and registry controlled domains.
    (
        b"*.xn--poema-9qae5a.com.br",
        b"www.xn--poema-9qae5a.com.br",
        Some(true),
    ),
    (
        b"*.example.xn--mgbaam7a8h",
        b"test.example.xn--mgbaam7a8h",
        Some(true),
    ),
    // RFC6126 allows this, and NSS accepts it, but Chromium disallows it.
    // TODO: File bug against Chromium.
    (b"*.com.br", b"xn--poema-9qae5a.com.br", Some(true)),
    (b"*.xn--mgbaam7a8h", b"example.xn--mgbaam7a8h", None),
    // Wildcards should be permissible for 'private' registry-controlled
    // domains. (In mozilla::pkix, we do not know if it is a private registry-
    // controlled domain or not.)
    (b"*.appspot.com", b"www.appspot.com", Some(true)),
    (b"*.s3.amazonaws.com", b"foo.s3.amazonaws.com", Some(true)),
    // Multiple wildcards are not valid.
    (b"*.*.com", b"foo.example.com", None),
    (b"*.bar.*.com", b"foo.bar.example.com", None),
    // Absolute vs relative DNS name tests. Although not explicitly specified
    // in RFC 6125, absolute reference names (those ending in a .) should
    // match either absolute or relative presented names.
    // TODO: File errata against RFC 6125 about this.
    (b"foo.com.", b"foo.com", None),
    (b"foo.com", b"foo.com.", Some(true)),
    (b"foo.com.", b"foo.com.", None),
    (b"f.", b"f", None),
    (b"f", b"f.", Some(true)),
    (b"f.", b"f.", None),
    (b"*.bar.foo.com.", b"www-3.bar.foo.com", None),
    (b"*.bar.foo.com", b"www-3.bar.foo.com.", Some(true)),
    (b"*.bar.foo.com.", b"www-3.bar.foo.com.", None),
    // We require the reference ID to be a valid DNS name, so we cannot test this
    // case.
    //     (b".", b".", Some(false)),
    (b"*.com.", b"example.com", None),
    (b"*.com", b"example.com.", None),
    (b"*.com.", b"example.com.", None),
    (b"*.", b"foo.", None),
    (b"*.", b"foo", None),
    // The result is different than Chromium because we don't know that co.uk is
    // a TLD.
    (b"*.co.uk.", b"foo.co.uk", None),
    (b"*.co.uk.", b"foo.co.uk.", None),
];

//#[test]
pub fn presented_matches_reference_test() {
    for &(presented, reference, expected_result) in PRESENTED_MATCHES_REFERENCE {

        #[cfg(not(target_env = "sgx"))]
        use std::string::String;

        let actual_result = presented_dns_id_matches_reference_dns_id(
            untrusted::Input::from(presented),
            untrusted::Input::from(reference),
        );
        assert_eq!(
            actual_result,
            expected_result,
            "presented_dns_id_matches_reference_dns_id(\"{}\", IDRole::ReferenceID, \"{}\")",
            String::from_utf8_lossy(presented),
            String::from_utf8_lossy(reference)
        );
    }
}
