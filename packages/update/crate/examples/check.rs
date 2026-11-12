//! `cargo run --example check -- owner/repo 1.0.0`: asks GitHub for the latest release.

use nodalia_update::{check, App};

fn main() {
    let mut args = std::env::args().skip(1);
    let repo: &'static str = Box::leak(args.next().unwrap_or_default().into_boxed_str());
    let version: &'static str = Box::leak(
        args.next()
            .unwrap_or_else(|| "0.0.0".into())
            .into_boxed_str(),
    );
    let app = App {
        name: "Example",
        repo,
        bundle: "Example.app",
        version,
        edition: None,
    };
    match check(&app) {
        Ok(result) => println!("{result:#?}"),
        Err(error) => println!("error: {error}"),
    }
}
