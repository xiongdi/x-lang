//! X Language LSP Server entry point

#![allow(dead_code)] // symbol table, diagnostics publish, workspace helpers — wired incrementally

use anyhow::Result;
use log::info;

mod analysis;
mod handlers;
mod server;
mod state;
mod utils;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting X Language LSP Server");

    let mut server = server::LspServer::new()?;
    server.run()?;

    Ok(())
}
