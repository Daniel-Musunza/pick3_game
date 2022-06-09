use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, log};
//use near_sdk::VMContext;
//near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Pick2 {
    // PICK2_GAME STATE
val: u8,
no_of_participants: u8,
name: String,

}

#[near_bindgen]
impl Pick2 {
    // PICK2_GAME METHODS HERE 

    pub fn parts(&mut self, no_of_participants: u8) {

    //     //the host to enter number of participants(n)
    
   log!("Enter number of Participants");
    self.obtain_parts();

        
          for _i in 1..no_of_participants{

         //each user to enter his/her name
       log!("Enter your Name/ID: ");
          self.name();

        //asks each user to enter his/her two lucky numbers
         log!("Enter your two Lucky Numbers between 1 and 9. \n e.g 12\n HINT: do not repeat a number ");
         self.obtain_no();
         self.secret_no_generate();
         self.guess(self.val);

            
           
       }
        
       //After all the participants guess their lucky numbers it prints the secret number
       log!("The Lucky number is {:?}", self.secret_no_generate());
        
        // Reset guess number to zero.
        self.reset();

        }


    pub fn obtain_parts(&self) -> u8 {
        return self.no_of_participants;

       }
    pub fn name(&self) -> String{
        return self.name.to_string();

       }

    pub fn obtain_no(&self) -> u8 {
        return self.val;

    }
    //function to generate the secret number
    pub fn secret_no_generate(&mut self){

        let num= env::random_seed();
        self.val=num[0];
    }

    //function for matching the input with the secret number
    pub fn guess(&mut self, input: u8){
    
        if input>10&&input<99&&input!=10&&input!=11&&input!=20&&input!=22&&input!=30&&input!=33&&input!=40&&input!=44&&input!=50&&input!=55&&input!=60&&input!=66&&input!=70&&input!=77&&input!=80&&input!=88&&input!=90&&input!=99 {
        
            if input==self.val{
                log!("WON");
                self.val =0;
            }else{
                log!("LOST");
            }
        }else{
            log!("INVALID INPUT! (please do not repeat a digit");
        }

    }
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        log!("Reset guess to zero"); 
    }
 


}
/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

//  attribute for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    //use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    //use near_sdk::test_utils::{get_logs, VMContextBuilder};
    //use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    //fn get_context(input: Vec<u8>, is_view: bool) -> VMContextBuilder {
     //   let mut builder = VMContextBuilder::new();
      //  builder.predecessor_account_id(input);
        //builder
    //}
    
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "festo.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    // TESTS HERE
    #[test]
    fn obtain_participants(){
         // The mock context set into the testing environment here
         let context = get_context(vec![], false);
         testing_env!(context);
         // a contract variable instantiated with the counter at zero
         //mut
         let contract = Pick2 { no_of_participants: 5,name: "Dan".to_string(),val: 14 };
         contract.obtain_parts();

         println!("The number of participants should be: {}", contract.obtain_parts());
         // confirming that we receive 5 after calling obtain_parts
         assert_eq!(5, contract.obtain_parts());

    }

    #[test]
    fn obtain_name(){

        let context = get_context(vec![], false);
        testing_env!(context);
        //mut
        let contract = Pick2 { no_of_participants: 5,name: "Dan".to_string(),val: 14 };
        contract.name();

        println!("The name should be: {}", contract.name());
    
        assert_eq!("Dan", contract.name());
    }
    #[test]
    fn generate_random_and_test() {
        
        let context = get_context(vec![], false);
        testing_env!(context);
        
        let mut contract = Pick2 { no_of_participants: 5,name: "Dan".to_string(),val: 0};
        contract.secret_no_generate();


        //guessing the correct number 
        contract.guess(contract.obtain_no());

        println!("Value after guess should be 0: {}", contract.obtain_no());
        // confirming that we receive 1 after calling obtain_no
        assert_eq!(0, contract.obtain_no());
    }

    #[test]
    fn generate_random_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Pick2 { no_of_participants: 5,name: "Dan".to_string(),val: 0};
        contract.secret_no_generate();
        contract.reset();
        println!("Value after guess reset: {}", contract.obtain_no());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.obtain_no());
    }
}
