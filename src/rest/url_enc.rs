use std::fmt::Display;

pub fn encode_url_param<'v>(value: &'v str) -> impl Display + 'v {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
    const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
    const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');

    utf8_percent_encode(value, PATH_SEGMENT)
}

#[cfg(test)]
mod tests {
    #[test]
    fn non_alphanumerics() {
        assert_eq!(
            "group%20%2F%20element%20-+%3F*",
            super::encode_url_param("group / element -+?*").to_string()
        )
    }
}
