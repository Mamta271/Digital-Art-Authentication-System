# Digital Art Authentication System

## Table of Contents:
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

---

## Project Title
**Digital Art Authentication System**

## Project Description
The Digital Art Authentication System is a decentralized platform designed to register, verify, and authenticate digital art pieces on the blockchain. It ensures that digital art creators can prove the authenticity of their work through a verifiable record on the blockchain.

---

## Project Vision
The vision of this project is to provide digital artists with an immutable and secure platform to authenticate and register their artwork. This will help in tackling issues of digital art piracy, uncredited work, and help establish trust between creators and potential buyers.

---

## Key Features
- **Art Registration**: Digital artists can register their artwork on the blockchain by submitting details such as title, description, and creator information.
- **Art Verification**: Only the original creator of the artwork can verify its authenticity, marking it as verified.
- **Immutable Records**: The entire process is transparent and tamper-proof, with all art pieces registered and verified on the blockchain.
- **Secure Ownership**: This system provides a secure way to prove ownership of digital art pieces.

---

## Contract Details

### Contract Address: CCNHWPIUOEBLJRTFFE7LHRMCAYXMGD4GTKKPUCYUJ5XC2H3KEKNSWMNU

![image](https://github.com/user-attachments/assets/b0271021-5d1c-4878-8eab-9489ae1919d5)

The smart contract contains the following core functions:
1. **register_art**: Allows a creator to register a new art piece with details such as title, description, and creation date.
2. **verify_art**: Allows the creator to verify the authenticity of their registered artwork, marking it as verified.
3. **get_art**: Allows anyone to view the details of a registered art piece, including its title, creator, description, and verification status.
4. **Immutable Data Storage**: Art pieces are stored in a decentralized manner, ensuring authenticity and preventing unauthorized changes.

---

### Contract Code Walkthrough
1. **ArtPiece Structure**: Each piece of art is represented by the `ArtPiece` struct which includes details like `id`, `creator`, `title`, `description`, `creation_date`, and `is_verified`.
2. **Register Art**: The `register_art` function allows an artist to register their artwork with unique details and increments the art piece count.
3. **Verify Art**: The `verify_art` function allows the creator to verify their artwork, marking it as verified on the blockchain.
4. **Get Art**: The `get_art` function retrieves the details of an art piece based on its ID.

---

### Requirements
- Soroban SDK setup
- A Soroban-enabled environment for deploying the contract

---

### How to Use
1. Deploy the contract to a Soroban-compatible blockchain.
2. Artists can use `register_art` to register their art pieces.
3. After registration, the creator can verify their work using `verify_art`.
4. Anyone can query the artwork's details using `get_art`.

---

## Future Enhancements
- Implement a marketplace for buying and selling verified digital art.
- Introduce a reward system for verified creators.
- Enable collectors to add metadata such as artwork provenance and transaction history.

---

