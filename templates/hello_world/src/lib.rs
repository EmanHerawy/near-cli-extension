
use near_sdk::{AccountId};
use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/**When writing smart contracts, the pattern is to have a struct with an associated impl
where you write the core logic into functions. It’s actually common in Rust to have this
pattern elsewhere. */
// the trait `BorshSerialize` is not implemented for `Contract`
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract{
// storage 
pub last_caller:AccountId,
pub last_message:String,
}
impl Default for Contract{
    fn default()->Self{
        panic!("Contract should be initialized before usage")
    }
}



pub trait ContractTraits{

    fn init(msg: String)->Self;
    fn send_message(&mut self, msg: String);
    fn get_last_caller(&self) ->AccountId;
    fn get_last_message(&self) ->String;
}
#[near_bindgen]
impl ContractTraits for Contract{
    #[init]
    fn init( msg: String)->Self{
       let this = Self{
         last_caller : env::signer_account_id(),
        last_message : msg,
       };
       this
    }
    fn send_message(&mut self, msg: String){
        self.last_caller = env::signer_account_id();
        self.last_message = msg;
    }
    fn get_last_caller(&self) ->AccountId{
        self.last_caller.clone()
    }
    fn get_last_message(&self) ->String{
        self.last_message.clone()
    }

}