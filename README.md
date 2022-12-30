Popula Community Genesis
==================

Community code holder and factory.

Exploring The Code
==================

## Terminology

* `owner_id`: The owner of this contract.
* `args`: Some customized arguments to pass on communities.
* `accounts`: Map of owner and communities.
* `codes`: Variety types of community code.  
* `account_storage_usage`: Estimation storage usage for a single account.

## Function Specification

### Deploy Community
Users can deploy and update their own communities and also owner of this contract can deploy and update communities for others. 

### Code and Arguments Management
Owner have a set of methods to update different types of code and pass on arguments for initialization or migration of a community.

## Build

Run `RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release` to build the project.
