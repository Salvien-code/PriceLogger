use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Counter {
    value: u64,
}

#[near_bindgen]
impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_count(&self) -> u64 {
        self.value
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn my_test() {
        let context = get_context(false);
        testing_env!(context);
        // ... Write test here
    }
}
