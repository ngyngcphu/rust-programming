use url::{ParseError, Url};

pub fn build_github_url(path: &str) -> Result<Url, ParseError> {
    const GITHUB: &str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}