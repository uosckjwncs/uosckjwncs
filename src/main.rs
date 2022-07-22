use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{env, io};

type ClientId = u16;
type TransactionId = u32;
type Amount = f64;

#[derive(Debug)]
struct Account {
    available: Amount,
    held: Amount,
    locked: bool,
}

type Accounts = HashMap<ClientId, Account>;

#[derive(Debug)]
struct Transaction {
    tx_id: TransactionId,
    client_id: ClientId,
    amount: Option<Amount>,
}

type TransactionLogs = HashMap<(TransactionId, ClientId), Amount>;
type Disputes = HashSet<(TransactionId, ClientId)>;

fn main() {
    let input_filename = env::args().nth(1).unwrap_or_else(|| String::from("-"));
    let input: Box<dyn io::Read + 'static> = match input_filename.as_str() {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input_filename).unwrap()),
    };

    let mut reader = ReaderBuilder::new().flexible(true).from_reader(input);

    let mut accounts: Accounts = HashMap::new();
    let mut disputes: Disputes = HashSet::new();
    let mut transaction_logs: TransactionLogs = HashMap::new();

    for string_record in reader.records() {
        let record = match string_record {
            Ok(mut r) => {
                if r.len() < 3 {
                    continue;
                }

                r.trim();
                r
            }
            _ => continue,
        };

        let transaction = Transaction {
            tx_id: match String::from(&record[2]).parse() {
                Ok(tx_id) => tx_id,
                _ => continue,
            },
            client_id: match String::from(&record[1]).parse() {
                Ok(client_id) => client_id,
                _ => continue,
            },
            amount: if record.len() < 4 {
                None
            } else {
                match String::from(&record[3]).parse::<Amount>() {
                    Ok(a) => Some(clamp(a)),
                    _ => continue,
                }
            },
        };

        match &record[0] {
            "deposit" => deposit(&mut accounts, &mut transaction_logs, &transaction),
            "withdrawal" => withdrawal(&mut accounts, &mut transaction_logs, &transaction),
            "dispute" => dispute(
                &mut accounts,
                &mut disputes,
                &mut transaction_logs,
                &transaction,
            ),
            "resolve" => resolve(
                &mut accounts,
                &mut disputes,
                &mut transaction_logs,
                &transaction,
            ),
            "chargeback" => chargeback(
                &mut accounts,
                &mut disputes,
                &mut transaction_logs,
                &transaction,
            ),
            _ => continue,
        }
    }

    println!("client,available,held,total,locked");

    for (client_id, account) in accounts.iter() {
        println!(
            "{},{},{},{},{}",
            client_id,
            clamp(account.available),
            clamp(account.held),
            clamp(account.available + account.held),
            account.locked
        )
    }
}

fn deposit(
    accounts: &mut Accounts,
    transaction_logs: &mut TransactionLogs,
    transaction: &Transaction,
) {
    let mut account = match accounts.get_mut(&transaction.client_id) {
        Some(a) => {
            // Assuming we don't want to accept future deposits if locked
            if a.locked {
                return;
            }

            a
        }
        None => {
            accounts.insert(
                transaction.client_id,
                Account {
                    available: 0.0,
                    held: 0.0,
                    locked: false,
                },
            );

            accounts.get_mut(&transaction.client_id).unwrap()
        }
    };

    let amount = transaction.amount.unwrap();

    transaction_logs.insert((transaction.tx_id, transaction.client_id), amount);

    account.available += amount;
}

fn withdrawal(
    accounts: &mut Accounts,
    transaction_logs: &mut TransactionLogs,
    transaction: &Transaction,
) {
    if let Some(account) = accounts.get_mut(&transaction.client_id) {
        // Assuming we don't want frozen accounts to withdraw
        if account.locked {
            return;
        }

        let amount = transaction.amount.unwrap();

        if account.available - amount >= 0.0 {
            transaction_logs.insert((transaction.tx_id, transaction.client_id), amount);

            account.available -= amount;
        }
    }
}

fn dispute(
    accounts: &mut Accounts,
    disputes: &mut Disputes,
    transaction_logs: &mut TransactionLogs,
    transaction: &Transaction,
) {
    if let Some(account) = accounts.get_mut(&transaction.client_id) {
        if let Some(amount) = transaction_logs.get(&(transaction.tx_id, transaction.client_id)) {
            disputes.insert((transaction.tx_id, transaction.client_id));

            account.available -= amount;
            account.held += amount;
        }
    }
}

fn resolve(
    accounts: &mut Accounts,
    disputes: &mut Disputes,
    transaction_logs: &mut TransactionLogs,
    transaction: &Transaction,
) {
    if let Some(account) = accounts.get_mut(&transaction.client_id) {
        if let Some(amount) = transaction_logs.get(&(transaction.tx_id, transaction.client_id)) {
            if !disputes.remove(&(transaction.tx_id, transaction.client_id)) {
                return;
            }

            account.available += amount;
            account.held -= amount;
        }
    }
}

fn chargeback(
    accounts: &mut Accounts,
    disputes: &mut Disputes,
    transaction_logs: &mut TransactionLogs,
    transaction: &Transaction,
) {
    if let Some(mut account) = accounts.get_mut(&transaction.client_id) {
        if let Some(amount) = transaction_logs.get(&(transaction.tx_id, transaction.client_id)) {
            if !disputes.remove(&(transaction.tx_id, transaction.client_id)) {
                return;
            }

            account.held -= amount;
            account.locked = true;
        }
    }
}

fn clamp(f: f64) -> f64 {
    (f * 10_000.0).round() / 10_000.0
}
