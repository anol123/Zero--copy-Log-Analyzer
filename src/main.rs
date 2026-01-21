use analyzer::analyze;
use io::reader::open_reader;
use log_analyzer::{analyzer, io};

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let reader = open_reader(path)?;
    let counts = analyze(reader)?;

    println!("INFO: {}", counts.info);
    println!("WARN: {}", counts.warn);
    println!("ERROR: {}", counts.error);
    println!("MALFORMED: {}", counts.malformed);

    Ok(())
}
