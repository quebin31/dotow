use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::collections::BTreeSet;
use std::error::Error as StdError;

use io::*;
use rayon::prelude::*;
use ansi_term::Colour::{
    Cyan,
    Yellow,
    Green,
};

use crate::utility;
use crate::error::Error;

fn get_ignores(working_path: &Path) -> Result<BTreeSet<String>, Box<dyn StdError>> {
    let mut dotow_ignore_found = false;

    for file in working_path.read_dir()? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }

        let file_name = file.file_name();
        let file_name = file_name.to_str().ok_or(Error::FailToConvertFileName(file_name.clone()))?;

        if file_name == ".dotowignore" {
            dotow_ignore_found = true;
            break;
        }
    }

    let mut ignores = BTreeSet::new();
    if !dotow_ignore_found {
        return Ok(ignores);
    }

    let dotow_ignore = File::open(working_path.join(".dotowignore"))?;
    for line in BufReader::new(dotow_ignore).lines().map(|l| l.unwrap()) {
        if line.contains(" ") || line.contains("\t") {
            return Err(Box::new(Error::BadString(line)));
        }

        ignores.insert(line);
    }

    Ok(ignores)
}

fn get_dotfiles(working_path: &Path) -> Vec<String> {
    let mut dots = Vec::new();

    for dot in working_path.read_dir().expect("Failed to read directory") {
        let dot = dot.unwrap().path();
        if dot.is_dir() {
            let dot = dot.file_name().unwrap().to_str().unwrap();
            if dot.starts_with(".") {
                continue;
            }

            dots.push(dot.to_owned());
        }
    }

    dots
}

pub fn install(working: &str, target: &str) -> Result<(), Box<dyn StdError>> {
    let target_path = Path::new(target).canonicalize().unwrap();
    utility::check_directory(&target_path)?;

    if target_path.to_str().unwrap() != env::var("HOME").unwrap() {
        uiprint!(warning format!("target directory is not $HOME ({})", env::var("HOME").unwrap()));
        uiprint!(warning format!("target directory is {}", target_path.to_str().unwrap()));
    }

    let working_path = Path::new(working).canonicalize().unwrap();
    utility::check_directory(&working_path)?;

    let dotfiles = get_dotfiles(&working_path);
    let mut ignores = get_ignores(&working_path)?;

    println!("{} I've found the following dotfiles: ", Yellow.paint("::"));
    for (index, dot) in dotfiles.iter().enumerate() {
        println!("  {} {}", Green.paint(format!("{:>3}", index + 1)), dot);
    } 
    println!("{} Dotfiles to not install: (e.g: 1 2 3)", Cyan.paint("=>"));
    print!("{} ", Cyan.paint("=>"));
    flush!();

    let ignored: String = read!("{}\n");
    let ignored: Vec<_> = if ignored.is_empty() { 
        Vec::new() 
    } else {
        ignored.trim().split(" ").map(|opt| opt.to_owned()).collect()
    };

    for index in ignored {
        let index: usize = index.parse()?;
        if (index - 1) < dotfiles.len() {
            ignores.insert(dotfiles[index - 1].clone());
        }
    }

    println!("{} Starting installation: ", Yellow.paint("::"));
    dotfiles.par_iter().for_each(|dot| {
        if ignores.contains(dot) {
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

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::error::Error as StdError;

    #[test]
    fn get_ignores() -> Result<(), Box<dyn StdError>> {
        let ignores = super::get_ignores(Path::new("test/dotfiles"))?;
        println!("{:#?}", ignores);

        Ok(())
    }
}
