use clap::Parser;
use std::process::Command;
use std::fs::File;
use std::io::Write;


#[derive(Parser, Debug)]
#[command(author,version)]
struct Args{
    #[arg(short)]
    contest_id: String,
    #[arg(short)]
    problem_id: String,
}
fn main() -> std::io::Result<()>{
    let args = Args::parse();
    let mut scraper = Command::new("python3");
    scraper.arg("src/scrape.py")
           .arg(args.contest_id)
           .arg(args.problem_id);
    let output = scraper.output().unwrap();
    let mut file = File::create("out.txt")?;
    file.write_all(&output.stdout).unwrap();
    Ok(())
}
