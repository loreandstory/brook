use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Clone, Debug)]
enum TransactionKind {
    Withdrawal,
    Deposit
}

#[derive(Clone, Debug)]
struct Transaction {
    date: DateTime<Utc>,
    kind: TransactionKind,
    amount: f32,
    fund: String,
    entity: String,
    description: String
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transaction_kind = match &self.kind {
            TransactionKind::Withdrawal => "-",
            TransactionKind::Deposit => "+"
        };

        write!(f, "{}\t{}{}\t{}\t{}\t{}",
            self.date.format("%a %b %e %Y"),
            transaction_kind,
            self.amount,
            self.fund,
            self.entity,
            self.description,
        )
    }
}

impl Transaction {
    fn new(date: DateTime<Utc>, kind: TransactionKind, amount: f32, fund: String, entity: String, description: String)  -> Self {
        Transaction {
            date,
            kind,
            amount,
            fund,
            entity,
            description,
        }
    }

    fn withdrawal(amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(Utc::now(), TransactionKind::Withdrawal, amount, fund, entity, description)
    }

    fn deposit(amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(Utc::now(), TransactionKind::Deposit, amount, fund, entity, description)
    }

}

struct Funds(f32);

impl Funds {
    fn deposit(&mut self, amount: f32) {
        // normal add, but done with proper rounding for money
        self.0 = ((100.0 * self.0).round() + (100.0 * amount).round()) / 100.0;
    }

    fn withdraw(&mut self, amount: f32) {
        // normal subtract, but done with proper rounding for money
        self.0 = ((100.0 * self.0).round() - (100.0 * amount).round()) / 100.0;

    }
}

struct Fund {
    name: String,
    amount: Funds
}

struct Account {
    balance: Funds,
    funds: Vec<Fund>,
    transactions: Vec<Transaction>
}

impl Account {
    fn setup_fund(&mut self, fund: Fund) {
        self.funds.push(fund);
    }

    fn setup_funds(&mut self, funds: Vec<Fund>) {
        for fund in funds {
            self.setup_fund(fund);
        }
    }

    fn index_of_fund_with_name(&self, name: &String) -> usize {
        let mut counter: usize = 0;
        for fund in &self.funds {
             if fund.name == *name {
                break;
            }

            counter += 1;
        };

        counter
    }

    fn process_transaction(&mut self, transaction: Transaction) {
        let funds_index = self.index_of_fund_with_name(&transaction.fund);

        match &transaction.kind {
            TransactionKind::Withdrawal => {
                self.balance.withdraw(transaction.amount);
                self.funds[funds_index].amount.withdraw(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            },

            TransactionKind::Deposit => {
                self.balance.deposit(transaction.amount);
                self.funds[funds_index].amount.deposit(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            }
        }

        self.transactions.push(transaction);
    }

    fn process_transactions(&mut self, transactions: Vec<Transaction>) {
        for transaction in transactions {
            self.process_transaction(transaction);
        }
    }

    fn transfer(&mut self, to_transfer: Transaction, to_account: &mut Account) {
        let clone_t = to_transfer.clone();
        self.process_transaction(to_transfer);
        to_account.process_transaction(clone_t);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_withdrawal() {
        let withdrawal = Transaction::withdrawal(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik"));
        // TODO: use matches and Result so ensure date and kind are correct as well.
        assert_eq!(18.34, withdrawal.amount);
        assert_eq!(String::from("Groceries"), withdrawal.fund);
        assert_eq!(String::from("HEB"), withdrawal.entity);
        assert_eq!(String::from("For kishik"), withdrawal.description);
    }

    #[test]
    fn can_create_new_deposit() {
        let deposit = Transaction::deposit(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik"));
        // TODO: use matches and Result so ensure date and kind are correct as well.
        assert_eq!(18.34, deposit.amount);
        assert_eq!(String::from("Groceries"), deposit.fund);
        assert_eq!(String::from("HEB"), deposit.entity);
        assert_eq!(String::from("For kishik"), deposit.description);
    }
}
