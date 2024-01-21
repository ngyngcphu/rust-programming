use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)+
            [[:word:]]+$
            "
        )
        .unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

pub fn extract_hashtags(input: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
    }
    HASHTAG_REGEX
        .find_iter(input)
        .map(|mat| mat.as_str())
        .collect()
}

pub struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

impl<'a> fmt::Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

pub fn extract_us_phone_numbers(input: &str) -> impl Iterator<Item = PhoneNumber> {
    lazy_static! {
        static ref PHONE_REGEX: Regex = Regex::new(
            r#"(?x)
                (?:\+?1)?                           # Country Code Optional
                [\s\.]?
                (([2-9]\d{2}) | \(([2-9]\d{2})\))   # Area Code
                [\s\.\-]?
                ([2-9]\d{2})                        # Exchange Code
                [\s\.\-]?
                (\d{4})                             # Subscriber Number
            "#
        )
        .unwrap();
    }
    PHONE_REGEX.captures_iter(input).filter_map(|cap| {
        let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    })
}

pub fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX: Regex =
            Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}
