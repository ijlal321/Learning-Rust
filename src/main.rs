use std::fmt::Error;
use std::fs;
use std::io::{self, stdin};
use std::process;

fn main() {
    let res = write_to_file();
    match res {
        Ok(..) => println!("Successfully done"),
        Err(error) => {
            eprintln!("Some error occured: {}", error);
            process::exit(1);
        }
    }
}

fn write_to_file() -> Result<String, io::Error> {
    println!("What file would you like to write to, Master -?");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write to the file, Master -?");
    let mut data_to_write = String::new();
    stdin().read_line(&mut data_to_write)?;

    fs::write(file_name.trim(), data_to_write.trim())?;

    Ok(file_name)
}
