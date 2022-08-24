/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

// import HashMap
use std::collections::HashMap;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// enum TaskStatus {
//     TODO,
//     IN_PROGRESS,
//     DONE
// }

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Contract {
    message: String,
    tasks: HashMap<String, String>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            message: DEFAULT_MESSAGE.to_string(),
            tasks: HashMap::new()
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    // Public method - returns the current tasks list
    pub fn get_tasks(&self) -> HashMap<String, String>{
        return self.tasks.clone();
    }

    // Public method - insert new task to tasks list
    pub fn insert_task(&mut self, task_name: String) {
        log!("Insert new task {}", task_name);
        self.tasks.insert(task_name, "TODO".to_owned());
    }

    // Public method - update task status in tasks list
    pub fn update_task(&mut self, task_name: String, task_status: String) {
        log!("Update task {} to {}", task_name, task_status);
        *self.tasks.get_mut(&task_name).unwrap() = task_status
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }

    #[test]
    fn insert_then_get_task() {
        let mut contract = Contract::default();
        let mut output_tasks = HashMap::new();
        output_tasks.insert(String::from("task_a"), String::from("TODO"));
        contract.insert_task(String::from("task_a"));
        assert_eq!(
            contract.get_tasks(),
            output_tasks
        );
    }

    #[test]
    fn insert_update_then_get_task() {
        let mut contract = Contract::default();
        let mut output_tasks = HashMap::new();
        output_tasks.insert(String::from("task_a"), String::from("DONE"));
        contract.insert_task(String::from("task_a"));
        contract.update_task(String::from("task_a"), String::from("DONE"));
        assert_eq!(
            contract.get_tasks(),
            output_tasks
        );
    }
}
