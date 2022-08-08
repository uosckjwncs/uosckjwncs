pub mod account;
pub mod accounts;
pub mod libs;
pub mod transaction;
pub mod transaction_logs;

use crate::libs::*;

use csv::{ReaderBuilder, Trim};
use rust_decimal::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::process::exit;

fn main() {
    let mut accounts: Accounts = Accounts::new();
    let mut disputes: Disputes = HashSet::new();
    let mut transaction_logs = TransactionLogs::new();

    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!(
            "Usage: {} <filename>",
            env::args()
                .next()
                .unwrap_or_else(|| String::from("uosckjwncs"))
        );
        exit(1)
    });

    let file = File::open(&filename).unwrap_or_else(|err| {
        eprintln!("Error: could not open '{filename}' ({err})");
        exit(1)
    });

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file);

    for record in reader.records().flatten() {
        let transaction = match Transaction::parse_record(&record) {
            Some(t) => t,
            None => continue,
        };

        let mut account = match accounts.find_or_create(transaction.client_id) {
            Some(a) => a,
            None => continue,
        };

        match &record[0] {
            "deposit" => account.deposit(&transaction, &mut transaction_logs),
            "withdrawal" => account.withdrawal(&transaction, &mut transaction_logs),
            "dispute" => account.dispute(&transaction, &mut transaction_logs, &mut disputes),
            "resolve" => account.resolve(&transaction, &mut transaction_logs, &mut disputes),
            "chargeback" => account.chargeback(&transaction, &mut transaction_logs, &mut disputes),
            _ => continue,
        }

        accounts.insert(transaction.client_id, account);
    }

    println!("client,available,held,total,locked");

    for (client_id, account) in accounts.iter() {
        println!(
            "{},{},{},{},{}",
            client_id,
            format(account.available),
            format(account.held),
            format(account.available + account.held),
            account.locked
        )
    }
}
