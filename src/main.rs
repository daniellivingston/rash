use clap::{Arg, ArgAction, Command};
use colored::*;
use sha2::{Digest, Sha256};
use std::{fs, io};

fn hash(filename: &str) -> String {
    let mut file = fs::File::open(filename).expect("Could not read file. Aborting!");

    let mut hasher = Sha256::new();
    let _ = io::copy(&mut file, &mut hasher).expect("hashing failed");

    let hash = hasher.finalize();
    hex::encode(hash)
}

fn cli() -> Command {
    Command::new("hash")
        .about("Simple file hashing program")
        .version("0.1.0")
        .author("DRL")
        .arg(
            Arg::new("FILES")
                .help("Path to file(s) for hashing")
                .action(ArgAction::Set)
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("short")
                .long("short")
                .short('s')
                .help("Returns a short hash")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sort")
                .long("sort")
                .help("Sorts the output by hash value")
                .action(ArgAction::SetTrue),
        )
}

fn main() {
    let matches = cli().get_matches();
    let filenames: Vec<_> = matches
        .get_many::<String>("FILES")
        .expect("missing required filename")
        .map(|s| s.to_string())
        .collect();

    let mut hashes: Vec<(String, String)> = filenames
        .into_iter()
        .map(|filename| (hash(&filename), filename))
        .collect();

    if matches.get_flag("sort") {
        hashes.sort_by(|(a, _), (b, _)| a.cmp(b));
    }

    for (hash, filename) in hashes {
        let hash = if matches.get_flag("short") {
            &hash[..8]
        } else {
            &hash[..]
        };

        println!("{}  {}", hash.bold().bright_blue(), filename.bright_green());
    }
}
