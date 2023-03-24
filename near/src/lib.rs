use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    owner_id: AccountId,
    value: u64,
}

impl Default for Counter {
    fn default() -> Self {
        env::panic_str("Counter should be initialized before usage")
    }
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
     * Resets the counter to 0 and can only be called by the owner.
     */
    pub fn reset(&mut self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can reset the counter."
        );
        self.value = 0;
    }

    /**
     * Increment the counter by 1 and reset to 0 if it exceeds 10.
     */
    pub fn increment(&mut self) {
        match self.value {
            0..=9 => self.value += 1,
            _ => self.value = 0,
        }
    }

    /**
     * Decrement the counter by 1 and reset to 10 if it goes below 0.
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

    /**
     * Get the owner of the contract.
     */
    pub fn get_owner_id(&self) -> &str {
        self.owner_id.as_str()
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
            .predecessor_account_id(precedessor_account_id)
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_stores_owner() {
        let context = get_context(accounts(2), false);
        testing_env!(context.build());
        let contract = Counter::new();
        assert_eq!(contract.get_owner_id(), accounts(2).as_str());
    }

    #[test]
    fn test_resets_after_10() {
        let context = get_context(accounts(2), false);
        testing_env!(context.build());
        let mut contract = Counter::new();
        for _ in 0..=10 {
            contract.increment();
        }
        assert_eq!(contract.get_count(), 0);
    }

    #[test]
    fn test_resets_after_0() {
        let context = get_context(accounts(2), false);
        testing_env!(context.build());
        let mut contract = Counter::new();
        contract.value = 10;
        for _ in 0..=10 {
            contract.decrement();
        }
        assert_eq!(contract.get_count(), 10);
    }

    #[test]
    fn test_can_be_reset_by_owner() {
        let context = get_context(accounts(2), false);
        testing_env!(context.build());
        let mut contract = Counter::new();
        contract.increment();
        contract.reset();
        assert_eq!(contract.get_count(), 0);
    }

    #[test]
    #[should_panic(expected = "Only owner can reset the counter.")]
    fn test_can_not_be_reset_by_non_owner() {
        let context = get_context(accounts(2), false);
        testing_env!(context.build());
        let mut contract = Counter::new();
        contract.increment();

        let context = get_context(accounts(3), false);
        testing_env!(context.build());
        contract.reset();
    }
}
