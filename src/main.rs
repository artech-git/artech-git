
#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::{bail, Result};
use structure::DBLayout;
use std::fs::File;
use std::io::prelude::*;
use std::process::ExitCode;


mod commands;
mod database;
mod error;
mod structure;
mod io; 
mod btree; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];

    // header buffer in the layout section
    let mut header = [0; 100];
    let mut db_layout = DBLayout::init(&args[1]).await.unwrap();


    match command.as_str() {

        ".dbinfo" => {
            db_layout.print_dbinfo().await;
        }, 

        ".tables" => { 
            db_layout.print_tables().await;
        },

        _ => bail!("Missing or invalid command passed: {}", command),
    }

    return Ok(());
}
