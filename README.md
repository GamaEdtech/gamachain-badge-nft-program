# GamaChain Badge NFT Program

**Solana Anchor program for minting and upgrading educational Badge NFTs within the GamaChain credential system.**

---

## Overview

The **GamaChain Badge NFT Program** is an on-chain Solana smart contract (built with the Anchor framework) that powers GamaChain’s verifiable learning credentials.  
Each badge represents a learner’s achievement and can be minted, upgraded, or verified as the student progresses through their educational journey.

This project enables:
- Tamper-proof educational records  
- NFT-based learning badges  
- Level-up functionality to represent learner progression  
- Integration with GamaChain’s off-chain exam data and EdTech platform  

---

## Features

- **Badge Minting:** Create NFTs tied to verified learning achievements  
- **Level Up:** Upgrade existing badge NFTs as learners gain new credentials  
- **Metadata Management:** Store and update educational details  
- **Anchor-based Program:** Built using [Anchor](https://www.anchor-lang.com/) for Solana smart contract development  
- **Open Source:** Licensed under Apache 2.0 for community collaboration  

---

## Tech Stack

- **Language:** Rust  
- **Framework:** [Anchor](https://www.anchor-lang.com/)  
- **Blockchain:** [Solana](https://solana.com/)  
- **Version Control:** GitHub  

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/chapter_2/installation.html)

### Build and Deploy
```bash
# Build the program
anchor build
```



##About GamaChain

GamaChain is an open, blockchain-powered credentialing network for education.
It transforms verified learning outcomes into NFT badges, enabling transparent, portable, and gamified academic credentials.

Learn more: https://github.com/GamaEdtech

# Deploy to localnet
anchor deploy
