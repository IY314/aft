use std::process::exit;

use clap::Parser;

mod lexer;

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let lex_res = lexer::lex(
        &std::fs::read_to_string(&args.file)
            .expect(format!("Could not read file {}", args.file).as_str()),
    );

    if let Err(lex_err) = lex_res {
        eprintln!("{:#?}", lex_err);
        exit(1);
    }

    println!("{:#?}", lex_res.unwrap());
}
