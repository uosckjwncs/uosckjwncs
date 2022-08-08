use super::*;

use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq)]
pub struct Accounts(HashMap<ClientId, Account>);

impl Accounts {
    pub fn new() -> Self {
        Accounts(HashMap::new())
    }

    pub fn find_or_create(&mut self, client_id: ClientId) -> Option<Account> {
        match self.get(&client_id) {
            Some(account) => {
                if account.locked {
                    // Assuming we don't want to act on frozen accounts
                    return None;
                }

                Some(*account)
            }
            None => {
                self.insert(
                    client_id,
                    Account {
                        available: Decimal::ZERO,
                        held: Decimal::ZERO,
                        locked: false,
                    },
                );

                self.get(&client_id).copied()
            }
        }
    }
}

impl Deref for Accounts {
    type Target = HashMap<ClientId, Account>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accounts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Accounts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    #[test]
    fn ok() {
        let accounts = Accounts::new();
        assert!(accounts == Accounts(HashMap::new()));
    }

    #[test]
    fn default() {
        let accounts = Accounts::new();
        assert!(accounts == Accounts(HashMap::new()));
    }
}

#[cfg(test)]
mod test_find_or_create {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new() {
        let mut accounts = Accounts::new();
        let account = accounts.find_or_create(1);

        let expected_account = Account {
            available: dec!(0.0),
            held: dec!(0.0),
            locked: false,
        };

        assert!(account == Some(expected_account));
    }

    #[test]
    fn locked() {
        let mut accounts = Accounts::new();

        accounts.insert(
            1,
            Account {
                available: dec!(5.0),
                held: dec!(10.0),
                locked: true,
            },
        );

        let account = accounts.find_or_create(1);

        assert!(account == None);
    }

    #[test]
    fn ok() {
        let mut accounts = Accounts::new();

        accounts.insert(
            1,
            Account {
                available: dec!(5.0),
                held: dec!(10.0),
                locked: false,
            },
        );

        let expected_account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let account = accounts.find_or_create(1);

        assert!(account == Some(expected_account));
    }
}

#[cfg(test)]
mod test_derefs {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn derefed_ok() {
        let accounts = Accounts::new();
        assert!(*accounts == HashMap::new());
    }

    #[test]
    fn derefedmut_ok() {
        let mut accounts = Accounts::new();

        accounts.insert(
            1,
            Account {
                available: dec!(5.0),
                held: dec!(10.0),
                locked: false,
            },
        );

        accounts.remove(&1);

        assert!(*accounts == HashMap::new());
    }
}
