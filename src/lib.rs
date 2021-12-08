use std::fmt;

#[derive(Clone, Debug)]
enum TransactionKind {
    Withdrawal,
    Deposit
}

#[derive(Clone, Debug)]
pub struct Transaction {
    date: String,
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

        write!(f, "{}  {:>9}  {:<15}  {:<15}  {:<25}",
            self.date,
            format!("{}{:.2}", transaction_kind, self.amount),
            self.fund,
            self.entity,
            self.description,
        )
    }
}

impl Transaction {
    fn new(date: String, kind: TransactionKind, amount: f32, fund: String, entity: String, description: String)  -> Self {
        Transaction {
            date,
            kind,
            amount,
            fund,
            entity,
            description,
        }
    }

    pub fn withdrawal(date: String, amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(date, TransactionKind::Withdrawal, amount, fund, entity, description)
    }

    pub fn deposit(date: String, amount: f32, fund: String, entity: String, description: String) -> Self {
        Transaction::new(date, TransactionKind::Deposit, amount, fund, entity, description)
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
  Income,
}

pub struct Fund {
    name: String,
    kind: FundKind,
    current_amount: Amount,
    begin_with_sum: f32,
    end_with_sum: f32,
}

impl Fund {
    fn new(name: String, kind: FundKind, begin_with_sum: f32, end_with_sum: f32) -> Self {
        Fund {
            name,
            kind,
            current_amount: Amount(begin_with_sum),
            begin_with_sum,
            end_with_sum,
        }
    }

    fn sum_status_out_of_20(&self) -> usize {
        let fraction = match self.kind {
            FundKind::Budget => {
                20.0 * (self.begin_with_sum - self.current_amount.0) / self.begin_with_sum
            },

            FundKind::Savings => {
                20.0 - 20.0 * (self.end_with_sum - self.current_amount.0) / (self.end_with_sum - self.begin_with_sum)
            },

            FundKind::Income => {
                20.0 * self.current_amount.0 / self.end_with_sum
            },
        };

        fraction as usize
    }

    pub fn budget(name: String, begin_with_sum: f32) -> Self {
        Fund::new(name, FundKind::Budget, begin_with_sum, 0.00)
    }

    pub fn savings(name: String, begin_with_sum: f32, increase_by_sum: f32) -> Self {
        Fund::new(name, FundKind::Savings, begin_with_sum, begin_with_sum + increase_by_sum)
    }

    pub fn income(name: String, end_with_sum: f32) -> Self {
        Fund::new(name, FundKind::Income, 0.00, end_with_sum)
    }
}

impl fmt::Display for Fund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<15}{:>12.2} {} {:<12.2}\n{:>5$.2}",
            self.name,
            self.begin_with_sum,
            // print balance
            format!("[{0:=>1$}{2:>3$}",
                 ">", self.sum_status_out_of_20(),
                 "]", 20 -self.sum_status_out_of_20()),
            self.end_with_sum,
            self.current_amount.0,
            35 + self.sum_status_out_of_20(),
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
        let fund_index = self.index_of_fund_with_name(&transaction.fund);

        match &transaction.kind {
            TransactionKind::Withdrawal => {
                self.balance.withdraw(transaction.amount);
                self.funds[fund_index].current_amount.withdraw(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            },

            TransactionKind::Deposit => {
                self.balance.deposit(transaction.amount);
                self.funds[fund_index].current_amount.deposit(transaction.amount);
                //TODO: implement handling funds: Vec<Funds>
            }
        }
    }

    pub fn process_transactions(&mut self) {
        for transaction in &self.transactions {
            let fund_index = self.index_of_fund_with_name(&transaction.fund);

            match &transaction.kind {
                TransactionKind::Withdrawal => {
                    self.balance.withdraw(transaction.amount);
                    self.funds[fund_index].current_amount.withdraw(transaction.amount);
                    //TODO: implement handling funds: Vec<Funds>
                },

                TransactionKind::Deposit => {
                    self.balance.deposit(transaction.amount);
                    self.funds[fund_index].current_amount.deposit(transaction.amount);
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
      println!("Current Balance: {:.2}", self.balance.0);

      println!("\n\n    Funds");
      println!("    -----");

      for fund in &self.funds {
          println!("\n    {}", fund);
      }

      println!("\n\n    Transactions");
      println!("    ------------");

      for transaction in &self.transactions {
          println!("    {}", transaction);
      }

      println!("\n---\n");
    }
}


