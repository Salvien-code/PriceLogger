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
