# Solana Builders Course — Weeks 1–7

Student: Balnur  
Network: Solana Devnet

This repository contains my practical work for Weeks 1–7 of the Solana Builders course.

## Week 1 — Blockchain & Solana Basics

- Set up Solana CLI and wallet
- Connected to Solana Devnet
- Received test SOL
- Completed first on-chain SOL transfer

Transaction:
`2vp5MrpVSe3237odpskktGG6dQVsZDCU7KS4vfx9zxatMF2Tj2Vuh9fHu8UbSNMEoz4Qc2LrJTFAdRyF3mffjrZx`

Transferred: `0.1 SOL`

## Week 2 — Solana Architecture & Transaction Analysis

Analyzed the Week 1 transaction.

- Program: System Program
- Transfer: 0.1 SOL
- Fee: 0.000005 SOL
- Compute Units: 150
- Studied Transactions, Instructions, Programs and Accounts

## Week 3 — Rust Fundamentals

Created a Rust CLI mini-project demonstrating:

- Variables and types
- Functions
- Structs and enums
- Ownership and borrowing
- Result/error handling
- Cargo tests

Tests: `2 passed, 0 failed`

Project: `week-03-rust`

## Week 4 — First Solana Program

Created, built and deployed my first Rust Solana Program to Devnet.

Program ID:
`68cqLcC7LeE2gABRd1AdcCP1r9ApwwJZ7D4Kq783eAEg`

Project: `week-04-solana-program`

## Week 5 — Anchor Framework & Counter Program

Created an Anchor Counter Program with:

- Anchor Accounts
- Constraints
- Initialize instruction
- Increment instruction
- Counter state
- IDL

Deployed to Devnet.

Program ID:
`Cxvh5d9yE4GY3MF7Cse7QULNF144V7AHRoHQVMxkkPS`

Project: `week-05-anchor-counter`

Note: the program builds and deploys successfully. The generated LiteSVM local test had a binary compatibility issue during program loading.

## Week 6 — PDA & Accounts Model

Created an on-chain Profile Program using a Program Derived Address (PDA).

Profile stores:

- Owner
- Name
- Bio

PDA seeds:

`["profile", user public key]`

The program uses seeds and bump to derive a unique profile account for each user.

Deployed to Devnet.

Program ID:
`63LK3qMWZVJ8tseoUyhS9v5miZK9U829ExyQcGHJZQyw`

Project: `week-06-pda-profile`

## Week 7 — SPL Token & Token-2022

Created my own SPL token on Devnet.

SPL Token Mint:
`BKygQbZ65u7B6VpmDVxynSYQxwPoBfPfKnP6f8bW7zPP`

- Created Associated Token Account
- Minted 1000 tokens
- Transferred 100 tokens to another wallet
- Final balances verified: 900 / 100
- Checked Mint Authority

I also created a Token-2022 mint with Metadata Extension.

Token-2022 Mint:
`ENYMR5Rn46xJgtmvyyPVJhxoAU5XceHm3X4d8zSdFDaX`

Metadata:

- Name: Balnur Token
- Symbol: BLNR
- Metadata Pointer enabled
- Update Authority configured

## Completed

Weeks 1–7 practical work completed on Solana Devnet.
