mod new_url;
mod parse_url;

fn main() {
    if let Err(err) = parse_url::parse_string_to_base_url() {
        eprintln!("Error: {}", err);
    }

    let path = "/rust-lang/cargo";
    let gh = new_url::build_github_url(path).unwrap();
    println!("The joined URL is: {}", gh);
}
