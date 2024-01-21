use text_processing::regex::{self, extract_hashtags};

fn main() {
    assert_eq!(
        regex::extract_login(r"I❤email@example.com"),
        Some(r"I❤email")
    );
    assert_eq!(
        regex::extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(regex::extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(regex::extract_login(r"Not an email@email"), None);

    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);

    let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";
    let res = regex::extract_us_phone_numbers(phone_text);
    assert_eq!(
        res.map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "1 (505) 881-9292",
            "1 (505) 778-2212",
            "1 (505) 881-9297",
            "1 (202) 991-9534",
            "1 (555) 392-0011",
            "1 (800) 233-2010",
            "1 (299) 339-1020",
        ]
    );

    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = regex::reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}
