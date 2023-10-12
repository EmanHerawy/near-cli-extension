
use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/**When writing smart contracts, the pattern is to have a struct with an associated impl
where you write the core logic into functions. It’s actually common in Rust to have this
pattern elsewhere. */
// the trait `BorshSerialize` is not implemented for `Contract`
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract{
// define contract fields here
}



// optional, use trait to define contract functions
pub trait ContractTraits{

}
#[near_bindgen]
impl ContractTraits for Contract{
    // constructor function
    // #[init]
    // fn init( /*args goes here  */)->Self{
    //    let this = Self{
    //     // initialize contract fields here
    //    };
    //    this
    // }

}