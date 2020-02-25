#![allow(dead_code)]
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path);
    match content {
        Ok(_) => {
            let u_content = &content.unwrap();
            for line in u_content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
