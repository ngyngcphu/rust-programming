use error_chain::error_chain;
use url::{Host, Url};

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
    errors {
        CannotBeABase
    }
}

pub fn parse_string_to_base_url() -> Result<()> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    assert_eq!(parsed.scheme(), "https");
    assert_eq!(parsed.host(), Some(Host::Domain("github.com")));
    assert_eq!(parsed.port_or_known_default(), Some(443));
    let base = base_url(parsed)?;
    println!("The base of the URL is: {}", base);
    Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }
    url.set_query(None);

    Ok(url)
}
