use anyhow::{bail, Result};
use structure::DBLayout;
use std::fs::File;
use std::io::prelude::*;


mod commands;
mod database;
mod error;
mod structure;
mod io; 


#[tokio::main]
async fn main() -> Result<()> {

    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];

    match command.as_str() {

        ".dbinfo" => {

            let mut header = [0; 100];

            // let mut file = File::open(&args[1])?;
            // file.read_exact(&mut header)?;

            // Uncomment this block to pass the first stage
            // println!("database page size: {}", page_size);

            let mut db_layout = DBLayout::init(&args[1]).await.unwrap();

            db_layout.print_data().await;
            
        }

        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
