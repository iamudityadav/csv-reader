use std::error::Error;
use std::path::Path;
use csv;

fn read_csv_file(fname: &Path) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(fname)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let file_name = Path::new(&*args[1]);
    if let Err(e) = read_csv_file(&file_name) {
        eprintln!("{}", e);
    }
}