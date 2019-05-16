use std::path::Path;
use std::collections::BTreeSet;
use std::error::Error as StdError;

use crate::error::{self, Error};

static KNOWN_COMMANDS: &[&'static str] = &["create", "install", "remove", "help"];

fn jaccard_index(a: &str, b: &str) -> f64 {
    let set_of_a: BTreeSet<_> = a.chars().collect();
    let set_of_b: BTreeSet<_> = b.chars().collect();
    let intersection: BTreeSet<_> = set_of_a.intersection(&set_of_b).collect();

    let union_len = set_of_a.len() + set_of_b.len();
    let intersection_len = intersection.len();

    intersection_len as f64 / union_len as f64
}

pub fn similar_command(cmd: &str) -> String {
    let mut max_index = 0;
    let mut max_jindex = 0.0;

    for (index, known_cmd) in KNOWN_COMMANDS.iter().enumerate() {
        let jindex = jaccard_index(known_cmd, cmd);
        if jindex > max_jindex {
            max_index = index;
            max_jindex = jindex;
        }
    }

    KNOWN_COMMANDS[max_index].to_owned()
}

pub fn check_directory(path: &Path) -> error::Result<()> {
    if !path.is_dir() {
        return Err(Error::NotDirectory(
            path.to_str().unwrap_or("unknown").to_owned(),
        ));
    }

    if !path.exists() {
        return Err(Error::DirectoryDoesNotExists(
            path.to_str().unwrap_or("unknown").to_owned(),
        ));
    }

    Ok(())
}


#[macro_export]
macro_rules! uiprint {
    (info $print:expr) => {
        print!("{} ", ansi_term::Color::White.bold().paint("info:"));
        println!("{}", $print);
    };

    (warning $print:expr) => {
        print!("{} ", ansi_term::Color::Yellow.bold().paint("warning:"));
        println!("{}", $print);
    };

    (error $print:expr) => {
        print!("{} ", ansi_term::Color::Red.bold().paint("error:"));
        println!("{}", $print);
    };

}

#[macro_export]
macro_rules! flush {
    () => {
        use std::io::Write;
        std::io::stdout().flush().expect("Failed to flush stdout!");
    }
}

#[cfg(test)]
mod test {
    use std::error::Error as StdError;

    #[test]
    fn command_suggestions() -> Result <(), Box<dyn StdError>>{
        let cmd = "crate";
        let suggested = super::similar_command(&cmd);
        assert_eq!(suggested, "create");

        let cmd = "intal";
        let suggested = super::similar_command(&cmd);
        assert_eq!(suggested, "install");

        let cmd = "rmv";
        let suggested = super::similar_command(&cmd);
        assert_eq!(suggested, "remove");

        Ok(())
    }
}
