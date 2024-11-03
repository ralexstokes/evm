use crate::U256;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

type StorageBacking = HashMap<U256, U256>;

#[derive(Debug, Default)]
pub struct Storage(StorageBacking);

impl Deref for Storage {
    type Target = StorageBacking;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
