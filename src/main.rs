use std::fs;

use anyhow::Result;
use clap::Parser;
use file_cmp::compare_lines;

#[derive(Parser)]
struct Args {
    #[clap(required = true, min_values = 2)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let files = read_files(args.files)?;

    let result = compare_lines(files);
    if result {
        println!("Equal");
    } else {
        println!("Not equal");
        std::process::exit(1);
    }

    Ok(())
}

fn read_files(filepaths: Vec<String>) -> Result<Vec<String>> {
    let mut files = Vec::with_capacity(filepaths.len());

    for filepath in filepaths {
        let file = fs::read_to_string(filepath)?;
        files.push(file);
    }

    Ok(files)
}
