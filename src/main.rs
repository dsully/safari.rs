#[macro_use]
extern crate clap;
#[macro_use]
extern crate tera;
extern crate urlparse;

use std::process;

use clap::App;

mod safari;
mod urls;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).usage("furl <subcommand>");
    let matches = app.get_matches();
    let url: String;

    if let Some(_) = matches.subcommand_matches("furl") {
        url = safari::safari_furl();
        print!("{}", urls::tidy_url(url));
    } else if let Some(_) = matches.subcommand_matches("2url") {
        url = safari::safari_2url();
        print!("{}", urls::tidy_url(url));
    } else if let Some(_) = matches.subcommand_matches("clean-tabs") {
        let urls = vec!["https://twitter.com", "https://www.facebook.com"];
        safari::safari_closetabs(urls);
    } else {
        App::from_yaml(yaml)
            .usage("furl <subcommand>")
            .print_help()
            .ok();
        process::exit(1);
    }
}
