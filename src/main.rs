extern crate termion;
extern crate hyper;
extern crate clap;
extern crate url;
extern crate rustc_serialize;

mod afk;

use clap::{Arg, App};

fn main(){
    let matches = App::new("afk-ent")
        .about("A cli-ent for arlefreak.com")
        .version("0.1.0")
        .author("Marioc hi@arlefreak.com")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("setup")
             .short("S")
             .long("setup")
             .takes_value(false)
             .help("Get api Token"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    afk::afk();
    afk::projects::projects();
    afk::about::about();
    afk::diary::diary();
    println!("{:?}", afk::diary::get_post(7).title);
}
