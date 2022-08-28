/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId};

type TaskId = String;

#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct Task {
    id: TaskId,
    task_name: String,
    task_status: String,
}

// Define the contract structure
#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tasks_by_account: LookupMap<AccountId, Vec<TaskId>>,
    tasks: LookupMap<TaskId, Task>,
}

// Define the default, which automatically initializes the contract
#[near_bindgen]
impl Default for Contract {
    fn default() -> Self {
        Self {
            tasks_by_account: LookupMap::new(b"ta".to_vec()),
            tasks: LookupMap::new(b"t"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the current tasks list
    pub fn get_tasks(&self) -> Vec<Task> {
        let owner = env::predecessor_account_id();
        match self.tasks_by_account.get(&owner) {
            Some(tasks) => {
                return tasks
                    .clone()
                    .into_iter()
                    .map(|t| self.tasks.get(&(t as TaskId)).unwrap())
                    .collect::<Vec<Task>>()
            }
            None => return Vec::new(),
        };
    }

    // Public method - insert new task to tasks list
    pub fn insert_task(&mut self, task_name: String) {
        log!("Insert new task {}", task_name);
        let owner = env::predecessor_account_id();
        let task_id = format!("{}.{}", owner, task_name);
        let task_obj = Task {
            id: task_id.clone(),
            task_name,
            task_status: "TODO".to_owned(),
        };
        let task_id_converted = (task_id as TaskId).clone();
        self.tasks.insert(&task_id_converted, &task_obj);
        let mut new_task_lists = match self.tasks_by_account.get(&owner) {
            Some(tasks) => tasks.clone(),
            _ => Vec::new(),
        };
        new_task_lists.push(task_id_converted);
        self.tasks_by_account.insert(&owner, &new_task_lists);
    }

    // Public method - update task status in tasks list
    pub fn update_task(&mut self, task_name: String, task_status: String) {
        log!("Update task {} to {}", task_name, task_status);
        let owner = env::predecessor_account_id();
        let task_id = format!("{}.{}", owner, task_name);
        let task_obj = Task {
            id: task_id.clone(),
            task_name: task_name,
            task_status: task_status,
        };
        self.tasks.insert(&task_id, &task_obj);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn insert_then_get_task() {
        let mut context = get_context(false);
        let alice: AccountId = accounts(0);

        context
            .account_balance(1000)
            .predecessor_account_id(alice.clone())
            .attached_deposit(1000)
            .signer_account_id(alice.clone());

        testing_env!(context.build());
        let mut contract = Contract::default();
        let mut output_tasks = Vec::new();
        output_tasks.push(Task {
            id: format!("{}.{}", alice, "task_a"),
            task_name: String::from("task_a"),
            task_status: String::from("TODO"),
        });
        contract.insert_task(String::from("task_a"));
        assert_eq!(contract.get_tasks(), output_tasks);
    }

    #[test]
    fn insert_update_then_get_task() {
        let mut context = get_context(false);
        let john: AccountId = accounts(1);

        context
            .account_balance(1000)
            .predecessor_account_id(john.clone())
            .attached_deposit(1000)
            .signer_account_id(john.clone());

        testing_env!(context.build());
        let mut contract = Contract::default();
        let mut output_tasks = Vec::new();
        output_tasks.push(Task {
            id: format!("{}.{}", john, "task_a"),
            task_name: String::from("task_a"),
            task_status: String::from("DONE"),
        });
        contract.insert_task(String::from("task_a"));
        contract.update_task(String::from("task_a"), String::from("DONE"));
        assert_eq!(contract.get_tasks()[0], output_tasks[0]);
    }
}