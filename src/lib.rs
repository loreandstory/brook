use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Clone, Debug)]
enum TransactionKind {
    Withdrawal,
    Deposit
}

#[derive(Clone, Debug)]
pub struct Transaction {
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

    pub fn withdrawal(amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(Utc::now(), TransactionKind::Withdrawal, amount, fund, entity, description)
    }

    pub fn deposit(amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(Utc::now(), TransactionKind::Deposit, amount, fund, entity, description)
    }
}

pub struct Amount(pub f32);

impl Amount {
    pub fn deposit(&mut self, amount: f32) {
        // normal add, but done with proper rounding for money
        self.0 = ((100.0 * self.0).round() + (100.0 * amount).round()) / 100.0;
    }

    pub fn withdraw(&mut self, amount: f32) {
        // normal subtract, but done with proper rounding for money
        self.0 = ((100.0 * self.0).round() - (100.0 * amount).round()) / 100.0;

    }
}

enum FundKind {
  Budget,
  Savings,
}

impl fmt::Display for FundKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fund_type: String = match self {
            FundKind::Budget => String::from("Budget"),
            FundKind::Savings => String::from("Savings"),
        };

        write!(f, "{}", fund_type)
    }
}

pub struct Fund {
    name: String,
    kind: FundKind,
    current_amount: Amount,
    start_and_end_amounts: (f32, f32),
}

impl Fund {
    fn new(name: String, kind: FundKind, starting_amount: f32, ending_amount: f32) -> Self {
        Fund {
          name,
          kind,
          current_amount: Amount(starting_amount),
          start_and_end_amounts: (starting_amount, ending_amount),
        }
    }

    pub fn budget(name: String, starting_amount: f32) -> Self {
        Fund::new(name, FundKind::Budget, starting_amount, 0.00)
    }

    pub fn savings(name: String, starting_amount: f32, amount_to_add: f32) -> Self {
        Fund::new(name, FundKind::Savings, starting_amount, starting_amount + amount_to_add)
    }
}

impl fmt::Display for Fund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>9.2} {} {:<9.2} {} {}\n{:.2}",
            self.start_and_end_amounts.0,
            // print balance
            format!("[{0:=>1$}{2:>3$}", ">", 10, "]", 5),
            self.start_and_end_amounts.1,
            self.kind,
            self.name,
            self.current_amount.0,
        )
    }
}

pub struct Account {
    name: String,
    starting_amount: Amount,
    balance: Amount,
    funds: Vec<Fund>,
    transactions: Vec<Transaction>,
    future_transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(name: String, starting_amount: Amount, funds: Vec<Fund>, transactions: Vec<Transaction>, future_transactions: Vec<Transaction>) -> Account {
        let mut new_account = Account {
          name,
          starting_amount,
          balance: Amount(0.00),
          funds,
          transactions,
          future_transactions,
        };

        new_account.setup();
        new_account
    }

    pub fn add_fund(&mut self, fund: Fund) {
        self.funds.push(fund);
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

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    fn process_transaction(&mut self, transaction: &Transaction) {
        let funds_index = self.index_of_fund_with_name(&transaction.fund);

        match &transaction.kind {
            TransactionKind::Withdrawal => {
                self.balance.withdraw(transaction.amount);
                self.funds[funds_index].current_amount.withdraw(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            },

            TransactionKind::Deposit => {
                self.balance.deposit(transaction.amount);
                self.funds[funds_index].current_amount.deposit(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            }
        }
    }

    pub fn process_transactions(&mut self) {
        for transaction in &self.transactions {
            let funds_index = self.index_of_fund_with_name(&transaction.fund);

            match &transaction.kind {
                TransactionKind::Withdrawal => {
                    self.balance.withdraw(transaction.amount);
                    self.funds[funds_index].current_amount.withdraw(transaction.amount);
                    //TODO: implement handling funds: Vec<Funds>
                },

                TransactionKind::Deposit => {
                    self.balance.deposit(transaction.amount);
                    self.funds[funds_index].current_amount.deposit(transaction.amount);
                    //TODO: implement handling funds: Vec<Funds>
                }
            }
        }
    }

    pub fn setup(&mut self) {
        self.process_transactions();
    }

    pub fn transfer(&mut self, to_transfer: Transaction, to_account: &mut Account) {
        self.process_transaction(&to_transfer);
        to_account.process_transaction(&to_transfer);
    }

    pub fn print(&self) {
      println!("\n# {}", self.name);
      for fund in &self.funds {
          println!("\n {}", fund);
      }
      println!("\n---\n");
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
