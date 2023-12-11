use std::path::Path;

use clap::{command, Arg, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand(Command::new("head").arg(Arg::new("path")))
        .subcommand(Command::new("tail").arg(Arg::new("path")))
        .get_matches();

    match matches.subcommand() {
        Some((name, args)) => match name {
            "head" => {
                let path = Path::new(args.get_one::<String>("path").unwrap());
                println!("{:?}", path);
            }
            "tail" => {
                let path = Path::new(args.get_one::<String>("path").unwrap());
                println!("{:?}", path);
            }
            _ => panic!(),
        },
        None => panic!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_math() {
        assert_eq!(1 + 1, 3);
    }
}
