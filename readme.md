## NEAR-cli-new
NEAR-cli-new is a command line tool that helps you create a new NEAR project based on a chosen template.
This is a NEAR Cli extension written in Rust.
### Problem
When I started to learn how to develop NEAR smart contract using rust, I found that there is no easy way to create a new project. I have to manually create a new project, check required dependencies , and modify the Cargo.toml file by adding these required dependencies. This is a tedious process and make it hard for new developers to get started.

### Steps to setup the project previously
#### Build from scratch
- Create a new cargo project `cargo new --lib my-project`
- search for dependencies in NEAR docs (**which is not easy to find in the docs** )or any project to add them
- update your lib.rs file with the required `using` statements
- develop your new contract from scratch

#### Fork a repository
- Look for a repository and fork it
- Clean up the forked repository so that you can use it as template

## Solution
- install the cli extension [check installation section](#installation)
- `near-cli-new` and follow the instructions and choose the template you want to use e.g `nft`, `empty`, `hello_world` etc

## Build 
- `cargo  build --release`

## Installation
install the cli extension locally 
```bash
cargo install --path .
```
## Value Proposition
- easy to use
- helps new developers to get started quickly
- helps new developers to learn how to develop NEAR smart contract using rust by showing them the required dependencies and contract architecture.
- supports multiple templates for different use cases


## future work
- Add more templates . This project will be the hub of all NEAR Examples
- Add support for new features e.g generate tests , contract abi 