#![doc = include_str!("../README.md")]
use anchor_lang::prelude::*;
use solana_program::pubkey;

pub mod account;
pub mod constants;
pub mod errors;
pub mod utils;

#[cfg(feature = "mainnet")]
declare_id!("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD");
#[cfg(not(feature = "mainnet"))]
declare_id!("BG3oRikW8d16YjUEmX3ZxHm9SiJzrGtMhsSR8aCw1Cd7");

#[derive(Clone)]
pub struct ZetaProgram;

impl anchor_lang::Id for ZetaProgram {
    fn id() -> Pubkey {
        ID
    }
}

#[derive(Clone)]
pub struct Dex;

impl anchor_lang::Id for Dex {
    fn id() -> Pubkey {
        match cfg!(feature = "mainnet") {
            true => pubkey!("zDEXqXEG7gAyxb1Kg9mK5fPnUdENCGKzWrM21RMdWRq"),
            false => pubkey!("5CmWtUihvSrJpaUrpJ3H1jUa9DRjYz4v2xs6c3EgQWMf"),
        }
    }
}

anchor_gen::generate_cpi_interface!(idl_path = "idl.json",);

pub use crate::account::*;
pub use crate::constants::*;
pub use crate::errors::*;
pub use crate::utils::*;
