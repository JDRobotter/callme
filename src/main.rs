#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use errors::*;

extern crate daemonize;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use daemonize::Daemonize;
use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {

    // handle arguments
    let args = App::new("Call Me")
        .version("0.1")
        .author("JD <jeandamien.brossillon@gmail.com>")
        .about("Call Me Maybe: an automated, ssh-call-me-back daemon")
        .subcommand(
            SubCommand::with_name("server")
            .about("start call-me daemon")
            .arg(
                Arg::with_name("foreground")
                .short("-f")
                .long("foreground")
                .help("start call-me daemon foreground")
            )
            .arg(
                Arg::with_name("config")
                .short("-c")
                .long("config")
                .help("specify a configuration file")
            )
        )
        .get_matches();

    if args.is_present("server") {
        // binary will run as server 
        if let Err(ref e) = run_server(args) {
            
            use std::io::Write;
            use error_chain::ChainedError;
            let stderr = &mut ::std::io::stderr();
            writeln!(stderr, "{}", e.display_chain())
                .expect("Error writing to stderr");

            ::std::process::exit(1);
        }
    }
    else {
        // binary will run as client

    }

/*
    let daemon = Daemonize::new()
        .pid_file("/tmp/this.pid")
        .chown_pid_file(true)
        .working_directory("/")
        .user("nobody")
        .group("daemon");
  */  
}

#[derive(Deserialize)]
struct Config {
    server_address: String,
}

fn read_config(filename: &str) -> Result<Config> {


    let mut file = File::open(filename)
        .chain_err(|| "unable to open file");

    Ok(Config {server_address:String::from("a")})
}

fn run_server(args:ArgMatches) -> Result<()> {

    // open and parse configuration file
    let config = args.value_of("config").unwrap_or("config.toml");

    let config = read_config(config)
        .chain_err(|| "unable to read configuration file");

    let mut contents = String::new();

    // daemonize
    let foreground = args.is_present("foreground");

    Ok(())
}
