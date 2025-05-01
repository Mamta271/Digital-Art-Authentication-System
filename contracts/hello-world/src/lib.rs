#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address};

// Structure to represent an Art Piece
#[contracttype]
#[derive(Clone)]
pub struct ArtPiece {
    pub id: u64,
    pub creator: Address,
    pub title: String,
    pub description: String,
    pub creation_date: u64, // Timestamp when the art was created
    pub is_verified: bool,
}

// For mapping art piece id to the ArtPiece
#[contracttype]
pub enum ArtPieceKey {
    ArtPiece(u64)
}

// Constants
const ART_COUNT: Symbol = symbol_short!("A_COUNT");

#[contract]
pub struct DigitalArtContract;

#[contractimpl]
impl DigitalArtContract {
    // Register a new art piece
    pub fn register_art(
        env: Env,
        creator: Address,
        title: String,
        description: String,
    ) -> u64 {
        // Authenticate the creator
        creator.require_auth();

        // Get current art piece count and increment
        let mut art_count: u64 = env.storage().instance().get(&ART_COUNT).unwrap_or(0);
        art_count += 1;

        // Create a new ArtPiece
        let art = ArtPiece {
            id: art_count,
            creator: creator.clone(),
            title,
            description,
            creation_date: env.ledger().timestamp(),
            is_verified: false,
        };

        // Store the art piece
        env.storage().instance().set(&ArtPieceKey::ArtPiece(art_count), &art);

        // Update art piece count
        env.storage().instance().set(&ART_COUNT, &art_count);

        log!(&env, "Art piece registered with ID: {}", art_count);

        art_count
    }

    // Verify the authenticity of an art piece (only the creator can verify)
    pub fn verify_art(env: Env, creator: Address, art_id: u64) -> bool {
        // Authenticate the creator
        creator.require_auth();

        // Get the art piece
        let mut art = Self::get_art(env.clone(), art_id);

        // Ensure the caller is the creator of the art piece
        if art.creator != creator {
            log!(&env, "Only the creator can verify this art piece");
            return false;
        }

        // Mark the art as verified
        art.is_verified = true;

        // Store the updated art piece
        env.storage().instance().set(&ArtPieceKey::ArtPiece(art_id), &art);

        log!(&env, "Art piece {} has been verified", art_id);

        true
    }

    // Get art piece details
    pub fn get_art(env: Env, art_id: u64) -> ArtPiece {
        let key = ArtPieceKey::ArtPiece(art_id);

        env.storage().instance().get(&key).unwrap_or(ArtPiece {
            id: 0,
            creator: Address::from_str(&env, "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            title: String::from_str(&env, ""),
            description: String::from_str(&env, ""),
            creation_date: 0,
            is_verified: false,
        })
    }
}
