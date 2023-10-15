## NEAR-cli-new
near-cli-new is a command-line tool that simplifies the process of creating new NEAR projects based on a chosen template. This Rust-based NEAR CLI extension aims to streamline project setup and assist developers in kickstarting their NEAR smart contract development.


## Table of Contents

- [NEAR-cli-new](#near-cli-new)
- [Table of Contents](#table-of-contents)
- [Introduction](#introduction)
- [Problem Statement](#problem-statement)
- [Previous Setup Steps](#previous-setup-steps)
    - [Building from Scratch](#building-from-scratch)
    - [Forking a Repository](#forking-a-repository)
- [Solution](#solution)
- [Build](#build)
- [Installation](#installation)
- [Value Proposition](#value-proposition)
- [future work](#future-work)

## Introduction

Creating a new NEAR project from scratch or forking existing repositories can be a time-consuming and challenging task, especially for newcomers. NEAR-cli-new simplifies this process and guides developers through the setup of NEAR smart contracts by offering various project templates.

## Problem Statement

Developers often face several difficulties when setting up NEAR projects:

- Discovering and incorporating the required dependencies, which are not always easy to find in the NEAR documentation.
- Understanding the architecture and structure of NEAR smart contracts.
- Avoiding common setup mistakes and issues.

## Previous Setup Steps

Before NEAR-cli-new, developers had to perform a series of manual setup steps:

#### Building from Scratch

1. Create a new Cargo project: `cargo new --lib my-project`.
2. Search for dependencies in NEAR docs (which is not always straightforward) and manually add them to the Cargo.toml file.
3. Update the lib.rs file with the necessary `using` statements.
4. Begin developing the NEAR smart contract from scratch.

#### Forking a Repository

1. Search for an existing repository and fork it.
2. Clean up the forked repository, removing unwanted files and customizing it to serve as a project template.

## Solution

NEAR-cli-new offers a user-friendly solution to the challenges faced by developers. With this tool, you can:

- Install the CLI extension locally with ease (see [Installation](#installation)).
- Execute `near-cli-new` and follow the on-screen instructions to create a new NEAR project using a template of your choice (e.g., `nft`, `empty`, `hello_world`).
  ![Alt text](demo.gif)

## Build 
- `cargo  build --release`

## Installation
install the cli extension locally 
```bash
cargo install --path .
```
## Value Proposition
- Easy to use
- Helps new developers to get started quickly
- Helps new developers to learn how to develop NEAR smart contract using rust by showing them the required dependencies and contract architecture.
- Supports multiple templates for different use cases


## future work
- Add more templates . This project will be the hub of all NEAR Examples
- Add support for new features e.g generate tests , contract abi 


