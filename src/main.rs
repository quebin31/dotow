use clap::{ErrorKind, clap_app};
use ansi_term::Colour::{
    Green
};

#[macro_use]
mod utility;

mod error;
mod create;
mod install;
mod remove;

use std::env::{var, current_dir};

fn main() {
    let mut dotow = clap_app!{ dotow =>
        (version: "0.1")
        (author: "Kevin Del Castillo <quebin31@gmail.com>")
        (about: "Manage your dotfiles easily")
        (@subcommand create =>
            (about: "Create a new dotow directory to manage your current dotfiles"))
        (@subcommand install => 
            (about: "Install dotfiles under some target directory")
            (@arg WORKING_DIR: "Working directory")
            (@arg TARGET_DIR: "Target directory"))
        (@subcommand remove => 
            (about: "Remove certain dotfiles"))
    };

    let matches = dotow.clone().get_matches_safe().unwrap_or_else(|e| {
        if e.kind == ErrorKind::HelpDisplayed || e.kind == ErrorKind::VersionDisplayed {
            e.exit()
        }

        if let Some(vec) = &e.info {
            let similar = utility::similar_command(&vec[0]);
            uiprint!(error format!("unknown command \'{}\'", vec[0]));
            println!("\nMaybe you wanted to type\n    {}", similar);
            println!("\nFor more information try {}", Green.paint("--help"));
        }

        std::process::exit(1);
    });

    match matches.subcommand() {
        ("create", Some(_create)) => {

        }

        ("install", Some(install)) => {
            let home_dir = var("HOME").unwrap();
            let current_dir = current_dir().unwrap().to_str().unwrap().to_owned();

            let working_dir = install.value_of("WORKING_DIR").unwrap_or(&current_dir);
            let target_dir = install.value_of("TARGET_DIR").unwrap_or(&home_dir);

            install::install(working_dir, target_dir).unwrap();
        }

        ("remove", Some(_remove)) => {

        }

        _ => { dotow.print_help().unwrap(); }
    }
}
