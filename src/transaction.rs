use super::*;

use csv::StringRecord;

#[derive(Clone, PartialEq)]
pub struct Transaction {
    pub id: TransactionId,
    pub client_id: ClientId,
    pub amount: Amount,
}

impl Transaction {
    pub fn parse_record(record: &StringRecord) -> Option<Transaction> {
        if record.len() < 3 {
            return None;
        }

        Some(Transaction {
            id: match String::from(&record[2]).parse() {
                Ok(id) => id,
                _ => return None,
            },
            client_id: match String::from(&record[1]).parse() {
                Ok(client_id) => client_id,
                _ => return None,
            },
            amount: if record.len() < 4 {
                Decimal::ZERO
            } else {
                match Decimal::from_str(&record[3]) {
                    Ok(amount) => {
                        if amount <= Decimal::ZERO {
                            return None;
                        }

                        match Decimal::from_str(&format(amount)) {
                            Ok(a) => a,
                            _ => return None,
                        }
                    }
                    _ => return None,
                }
            },
        })
    }
}

#[cfg(test)]
mod test_parse_record {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn two_cols() {
        let record = StringRecord::from(vec!["desposit", "1"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        assert!(transaction == None);
    }

    #[test]
    fn invalid_id() {
        let record = StringRecord::from(vec!["desposit", "invalid-id", "1", "1.0"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        assert!(transaction == None);
    }

    #[test]
    fn invalid_client_id() {
        let record = StringRecord::from(vec!["desposit", "1", "invalid-client-id", "1.0"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        assert!(transaction == None);
    }

    #[test]
    fn three_cols() {
        let record = StringRecord::from(vec!["despute", "1", "1"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        let expected_transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(0.0),
        };

        assert!(transaction == Some(expected_transaction));
    }

    #[test]
    fn invalid_amount() {
        let record = StringRecord::from(vec!["desposit", "1", "1", "invalid-amount"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        assert!(transaction == None);
    }

    #[test]
    fn negative_amount() {
        let record = StringRecord::from(vec!["despute", "1", "1", "-5.0"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        assert!(transaction == None);
    }

    #[test]
    fn ok() {
        let record = StringRecord::from(vec!["despute", "1", "1", "1.0"]);
        let transaction = Transaction::parse_record(&StringRecord::from(record));

        let expected_transaction = Transaction {
            id: 1,
            client_id: 1,
            amount: dec!(1.0),
        };

        assert!(transaction == Some(expected_transaction));
    }
}
