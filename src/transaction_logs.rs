use super::*;

use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq)]
pub struct TransactionLog {
    pub client_id: ClientId,
    pub amount: Amount,
}

#[derive(PartialEq)]
pub struct TransactionLogs(pub HashMap<TransactionId, TransactionLog>);

impl TransactionLogs {
    pub fn new() -> Self {
        TransactionLogs(HashMap::new())
    }

    pub fn add(&mut self, transaction: &Transaction) {
        self.insert(
            transaction.id,
            TransactionLog {
                client_id: transaction.client_id,
                amount: transaction.amount,
            },
        );
    }

    pub fn find(&mut self, transaction: &Transaction) -> Option<TransactionLog> {
        if let Some(transaction_log) = self.get(&transaction.id) {
            if transaction_log.client_id == transaction.client_id {
                return Some(*transaction_log);
            }
        }

        None
    }
}

impl Deref for TransactionLogs {
    type Target = HashMap<TransactionId, TransactionLog>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TransactionLogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for TransactionLogs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_new {
    use super::*;

    #[test]
    fn ok() {
        let transaction_logs = TransactionLogs::new();
        assert!(transaction_logs == TransactionLogs(HashMap::new()));
    }

    #[test]
    fn default() {
        let transaction_logs = TransactionLogs::new();
        assert!(transaction_logs == TransactionLogs(HashMap::new()));
    }
}

#[cfg(test)]
mod test_add {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn ok() {
        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        });

        let mut expected_transaction_logs = HashMap::new();

        expected_transaction_logs.insert(
            1,
            TransactionLog {
                client_id: 1,
                amount: dec!(1.0),
            },
        );

        assert!(*transaction_logs == expected_transaction_logs);
    }
}

#[cfg(test)]
mod test_find {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn does_not_exist() {
        let mut transaction_logs = TransactionLogs::new();

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        };

        assert!(transaction_logs.find(&transaction) == None);
    }

    #[test]
    fn exists_but_not_owned() {
        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        };

        assert!(transaction_logs.find(&transaction) == None);
    }

    #[test]
    fn ok() {
        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        });

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        };

        let expected_transaction_log = TransactionLog {
            client_id: 1,
            amount: dec!(1.0),
        };

        assert!(transaction_logs.find(&transaction) == Some(expected_transaction_log));
    }
}

#[cfg(test)]
mod test_derefs {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn derefed_ok() {
        let transaction_logs = TransactionLogs::new();
        assert!(*transaction_logs == HashMap::new());
    }

    #[test]
    fn derefedmut_ok() {
        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        });

        transaction_logs.remove(&1);

        assert!(*transaction_logs == HashMap::new());
    }
}
