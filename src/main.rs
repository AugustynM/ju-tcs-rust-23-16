use std::path::Path;

use clap::{command, Arg, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand(Command::new("head").arg(Arg::new("path")).arg(Arg::new("n")))
        .subcommand(Command::new("tail").arg(Arg::new("path")).arg(Arg::new("n")))
        .get_matches();

    match matches.subcommand() {
        Some((name, args)) => match name {
            "head" => {
                let path = Path::new(args.get_one::<String>("path").unwrap());
                let n = args.get_one::<String>("n").unwrap().parse::<usize>().unwrap();
                println!("{:?}, {n}", path);
            }
            "tail" => {
                let path = Path::new(args.get_one::<String>("path").unwrap());
                let n = args.get_one::<String>("n").unwrap().parse::<usize>().unwrap();
                println!("{:?}, {n}", path);
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
