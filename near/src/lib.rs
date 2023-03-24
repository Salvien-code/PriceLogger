use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    owner_id: AccountId,
    value: u64,
}

#[near_bindgen]
impl Counter {
    /**
     * Initializes the contract with the given `owner_id`.
     */
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            value: 0,
        }
    }

    /**
     * Only the owner of the contract can call this method.
     */
    pub fn reset(&mut self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can reset the counter."
        );
        self.value = 0;
    }

    /**
     * Increment the counter by 1 and reset to 0 if it reaches 10.
     */
    pub fn increment(&mut self) {
        match self.value {
            0..=9 => self.value += 1,
            _ => self.value = 0,
        }
    }

    /**
     * Decrement the counter by 1 and reset to 10 if it reaches 0.
     */
    pub fn decrement(&mut self) {
        match self.value {
            1..=10 => self.value -= 1,
            _ => self.value = 10,
        }
    }

    /**
     * Get the current value of the counter.
     */
    pub fn get_count(&self) -> u64 {
        self.value
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #![allow(unused_imports)]

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    use super::*;

    fn get_context(precedessor_account_id: AccountId, is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .signer_account_id(precedessor_account_id.clone())
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_stores_owner() {
        let context = get_context(accounts(1), false);
        testing_env!(context.build());
        let contract = Counter::new();
        assert_eq!(contract.owner_id, accounts(1));
    }
}
