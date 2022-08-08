use super::*;

use rust_decimal::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Account {
    pub available: Amount,
    pub held: Amount,
    pub locked: bool,
}

impl Account {
    pub fn deposit(&mut self, transaction: &Transaction, transaction_logs: &mut TransactionLogs) {
        if !transaction_logs.contains_key(&transaction.id) {
            transaction_logs.add(transaction);

            // Assuming we don't want to skip/panic on overflow, let's saturate
            self.available = self.available.saturating_add(transaction.amount);
        }
    }

    pub fn withdrawal(
        &mut self,
        transaction: &Transaction,
        transaction_logs: &mut TransactionLogs,
    ) {
        if !transaction_logs.contains_key(&transaction.id) {
            transaction_logs.add(transaction);

            // Assuming we don't want to skip/panic on underflow, let's saturate
            if self.available.saturating_sub(transaction.amount) >= Decimal::ZERO {
                self.available = self.available.saturating_sub(transaction.amount);
            }
        }
    }

    pub fn dispute(
        &mut self,
        transaction: &Transaction,
        transaction_logs: &mut TransactionLogs,
        disputes: &mut Disputes,
    ) {
        if let Some(transaction_log) = transaction_logs.find(transaction) {
            // Assuming we don't want to skip/panic on underflow/overflow, let's saturate
            self.available = self.available.saturating_sub(transaction_log.amount);
            self.held = self.held.saturating_add(transaction_log.amount);

            disputes.insert(transaction.id);
        }
    }

    pub fn resolve(
        &mut self,
        transaction: &Transaction,
        transaction_logs: &mut TransactionLogs,
        disputes: &mut Disputes,
    ) {
        if let Some(transaction_log) = transaction_logs.find(transaction) {
            if disputes.remove(&transaction.id) {
                // Assuming we don't want to skip/panic on underflow/overflow, let's saturate
                self.held = self.held.saturating_sub(transaction_log.amount);
                self.available = self.available.saturating_add(transaction_log.amount);
            }
        }
    }

    pub fn chargeback(
        &mut self,
        transaction: &Transaction,
        transaction_logs: &mut TransactionLogs,
        disputes: &mut Disputes,
    ) {
        if let Some(transaction_log) = transaction_logs.find(transaction) {
            if disputes.remove(&transaction.id) {
                // Assuming we don't want to skip/panic on underflow, let's saturate
                self.held = self.held.saturating_sub(transaction_log.amount);

                self.locked = true;
            }
        }
    }
}

#[cfg(test)]
mod test_deposit {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn transaction_already_exists() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        });

        account.deposit(&transaction, &mut transaction_logs);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(5.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn ok() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(7.0),
        };

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        account.deposit(&transaction, &mut transaction_logs);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(12.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(7.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }
}

#[cfg(test)]
mod test_withdrawal {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn transaction_already_exists() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        });

        account.withdrawal(&transaction, &mut transaction_logs);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(5.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn ok() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        };

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        account.withdrawal(&transaction, &mut transaction_logs);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }
}

#[cfg(test)]
mod test_dispute {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn transaction_does_not_exist() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(2);

        //
        // action
        //

        let mut transaction_logs = TransactionLogs::new();

        account.dispute(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(5.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        assert!(transaction_logs == TransactionLogs::new());
    }

    #[test]
    fn transaction_exists_but_not_owned() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        account.dispute(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(5.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn ok() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(5.0),
            held: dec!(10.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        //
        // action
        //

        account.dispute(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(1);
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }
}

#[cfg(test)]
mod test_resolve {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn transaction_does_not_exist() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(2);

        //
        // action
        //

        let mut transaction_logs = TransactionLogs::new();

        account.resolve(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        assert!(transaction_logs == TransactionLogs::new());
    }

    #[test]
    fn transaction_exists_but_not_owned() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        account.resolve(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(1);
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn dispute_does_not_exist() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        //
        // action
        //

        disputes.remove(&1);

        account.resolve(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn ok() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        //
        // action
        //

        account.resolve(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(5.0),
                    held: dec!(10.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }
}

#[cfg(test)]
mod test_chargeback {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn transaction_does_not_exist() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(2);

        //
        // action
        //

        let mut transaction_logs = TransactionLogs::new();

        account.chargeback(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        assert!(transaction_logs == TransactionLogs::new());
    }

    #[test]
    fn transaction_exists_but_not_owned() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        //
        // action
        //

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        account.chargeback(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(1);
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 2,
            amount: dec!(1.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn dispute_does_not_exist() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        //
        // action
        //

        disputes.remove(&1);

        account.chargeback(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(13.0),
                    locked: false,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }

    #[test]
    fn ok() {
        //
        // setup
        //

        let mut account = Account {
            available: dec!(2.0),
            held: dec!(13.0),
            locked: false,
        };

        let transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        let mut disputes = HashSet::new();
        disputes.insert(1);
        disputes.insert(2);

        let mut transaction_logs = TransactionLogs::new();

        transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        //
        // action
        //

        account.chargeback(&transaction, &mut transaction_logs, &mut disputes);

        //
        // test what we expect
        //

        assert!(
            account
                == Account {
                    available: dec!(2.0),
                    held: dec!(10.0),
                    locked: true,
                }
        );

        let mut expected_disputes = HashSet::new();
        expected_disputes.insert(2);
        assert!(disputes == expected_disputes);

        let mut expected_transaction_logs = TransactionLogs::new();

        expected_transaction_logs.add(&Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(3.0),
        });

        assert!(transaction_logs == expected_transaction_logs);
    }
}
