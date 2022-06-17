use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, log, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct User {
    // USER STATE
guessnumber: u8,
name: String,

}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct Pick2 {
    // PICK2_GAME STATE
users: Vec<User>,
val: u8,
}

#[near_bindgen]
impl Pick2 {
    // PICK2_GAME METHODS HERE 


    //function to obtain number of participants
    pub fn obtain_participants(&self) -> usize{
       log!("Enter number of Participants");
       return self.users.len();
    }

    //function to let participants enter their names and lucky numbers
    pub fn add_guess (&mut self, name: String, guessnumber: u8) {

        let names = name.clone();
        let new_user = User {
            name: names,
            guessnumber: guessnumber,
        };

        self.users.push(new_user);
        // log!("Enter your Name: ");
        // return &name.to_string();

        // log!("Enter your two Lucky Numbers between 1 and 9.\n e.g 12 \n HINT: do not repeat a number ");
        // return guessnumber.to_string();

    }


    //function to generate the secret number
    pub fn secret_no_generate(&mut self){
        let mut gen_num: u8 = 0;
        while !gen_num>10&&gen_num<99&&gen_num!=10&&gen_num!=11&&gen_num!=20&&gen_num!=22&&gen_num!=30&&gen_num!=33&&gen_num!=40&&gen_num!=44&&gen_num!=50&&gen_num!=55&&gen_num!=60&&gen_num!=66&&gen_num!=70&&gen_num!=77&&gen_num!=80&&gen_num!=88&&gen_num!=90&&gen_num!=99 {
   // Generate random number from 10 to 100 based on the above conditions
        gen_num = *env::random_seed().get(0).unwrap()%100+10

        };
        self.val=gen_num;
                
    }
    
    //function of matching the guessnumber with the secret number across each participant
    pub fn determine_winner(&mut self){
        //obtain_participants
        //loop self.users ->each guess=val;
        for i in &self.users{
            let guessnumber = i.guessnumber;
            let name = &i.name;
            //checking whether the guessnumber is in [10,99] and not with 0s(eg 20) or with repeated digits(eg 77) 
            if guessnumber>10&&guessnumber<99&&guessnumber!=10&&guessnumber!=11&&guessnumber!=20&&guessnumber!=22&&guessnumber!=30&&guessnumber!=33&&guessnumber!=40&&guessnumber!=44&&guessnumber!=50&&guessnumber!=55&&guessnumber!=60&&guessnumber!=66&&guessnumber!=70&&guessnumber!=77&&guessnumber!=80&&guessnumber!=88&&guessnumber!=90&&guessnumber!=99 {
        
                if guessnumber==self.val{
                    log!("{}, WON", name);
                }else{
                    log!("{}, LOST", name);
                }
            }else{
                log!("INVALID INPUT! (please do not repeat a digit");
            }
    

        }
       
        //After all the participants guess their lucky numbers it prints the secret number
       log!("The Lucky number is {:?}", self.secret_no_generate());
       //calls the function reset
       self.reset();
        
    }
     // function to reset guess number to zero and partcipants to null
    pub fn reset(&mut self) {
        self.val = 0;
        self.users= Vec::new();
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
    use near_sdk::{testing_env, VMContext};
    
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


   

    // // TESTS HERE
    
    #[test]
    //testing the add_guess function
    fn add_guess_test() {
        
        let context = get_context(vec![], false);
        testing_env!(context);
        
        let mut contract = Pick2 { users: Vec::new(),val: 0};
            contract.add_guess ("name".to_string(),12);
        
        println!("Value after guess should be 0: {}", contract.val);
        assert_eq!(1, contract.users.len());
    }

    #[test]
    //testing the secret number generated
    fn secret_no_generate_test() {

        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Pick2 { users: Vec::new(),val: 0};
        contract.secret_no_generate();
        assert_ne!(0, contract.val);
        println!("Value generated is: {:?}", contract.secret_no_generate());
    }

    #[test]
    //testing the reset function for users
    fn reset_users_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Pick2 { users: Vec::new(),val: 0};
        contract.add_guess ("name".to_string(),12);
        assert_eq!(1, contract.users.len());
        contract.reset();
        println!("Value after guess reset: {}", contract.val);
        assert_eq!(0, contract.users.len());
    }
    #[test]
    //testing the reset function for value
    fn reset_value_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Pick2 { users: Vec::new(),val: 0};
        contract.secret_no_generate();
        assert_ne!(0, contract.val);
        contract.reset();
        println!("Value after guess reset: {}", contract.val);
        assert_eq!(0, contract.val);
    }
}
