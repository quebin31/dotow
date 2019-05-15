use crate::error::{self, Error};
use std::env;
use std::path::Path;
use std::process::Command;

use rayon::prelude::*;
use ansi_term::Colour::{
    Cyan,
    Yellow,
    Green,
};

use io::*;

fn check_directory(target_path: &Path) -> error::Result<()> {
    if !target_path.is_dir() {
        return Err(Error::NotDirectory(
            target_path.to_str().unwrap_or("unknown").to_owned(),
        ));
    }

    if !target_path.exists() {
        return Err(Error::DirectoryDoesNotExists(
            target_path.to_str().unwrap_or("unknown").to_owned(),
        ));
    }

    Ok(())
}

fn read_ignore(working_path: &Path) -> Vec<String> {
    let mut ignores = Vec::new();

    ignores
}

fn get_dotfiles(working_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dots = Vec::new();

    for dot in working_path.read_dir().expect("Failed to read directory") {
        let dot = dot?.path();
        if dot.is_dir() {
            let dot = dot.file_name().ok_or()?.to_str().unwrap();
            if dot.starts_with(".") {
                continue;
            }

            dots.push(dot.to_owned());
        }
    }

    Ok(dots)
}


//fn get_dotfiles(working_path: &Path) -> Vec<String> {
    //let mut dots = Vec::new();

    //for dot in working_path.read_dir().expect("Failed to read directory") {
        //let dot = dot.unwrap().path();
        //if dot.is_dir() {
            //let dot = dot.file_name().unwrap().to_str().unwrap();
            //if dot.starts_with(".") {
                //continue;
            //}

            //dots.push(dot.to_owned());
        //}
    //}

    //dots
//}

pub fn install(working: &str, target: &str) -> error::Result<()> {
    let target_path = Path::new(target).canonicalize().unwrap();
    check_directory(&target_path)?;

    if target_path.to_str().unwrap() != env::var("HOME").unwrap() {
        uiprint!(warning format!("target directory is not $HOME ({})", env::var("HOME").unwrap()));
        uiprint!(warning format!("target directory is {}", target_path.to_str().unwrap()));
    }

    let working_path = Path::new(working).canonicalize().unwrap();
    check_directory(&working_path)?;

    let dotfiles = get_dotfiles(&working_path);

    println!("{} I've found the following dotfiles: ", Yellow.paint("::"));
    for (index, dot) in dotfiles.iter().enumerate() {
        println!("  {} {}", Green.paint(format!("{:>3}", index + 1)), dot);
    } 
    println!("{} Dotfiles to not install: (e.g: 1 2 3)", Cyan.paint("=>"));
    print!("{} ", Cyan.paint("=>"));
    flush!();

    let ignored: String = read!("{}\n");
    let ignored = ignored.trim();
    let ignored: Vec<_> = ignored.split(" ").map(|opt| opt.to_owned()).collect();

    println!("{} Starting installation: ", Yellow.paint("::"));
    dotfiles.par_iter().enumerate().for_each(|(index, dot)| {
        if ignored.contains(&(index + 1).to_string()) {
            return;
        }

        println!("  {} Installing {}", Cyan.paint("=>"), dot);
        Command::new("stow")
            .current_dir(&working_path)
            .args(&["-t", target_path.to_str().unwrap(), dot])
            .spawn()
            .expect("Failed to execute stow");
    });

    Ok(())
}
