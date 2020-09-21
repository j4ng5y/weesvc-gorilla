#[macro_use] extern crate clap;
mod migrate;
mod serve;

fn main() {
    let matches = clap_app!(weesvc =>
        (version: "0.1.0")
        (author: "Jordan Gregory <gregory.jordan.m@gmail.com>")
        (about: "An implementation of the WeeSVC in Rust with the Rocket web framework")
        (@subcommand migrate =>
            (about: "run database migrations")
        )
        (@subcommand serve =>
            (about: "serve the application")
        )
    ).get_matches();

    if let Some(_matches) = matches.subcommand_matches("migrate") {
        println!("migrating the database")
    }
    if let Some(_matches) = matches.subcommand_matches("serve") {
        println!("serving the application")
    }
}