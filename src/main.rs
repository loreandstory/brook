use brook::*;

fn main() {
    let funds = vec![
        Fund::budget(String::from("Groceries"), Amount(200.00)),
        Fund::budget(String::from("Gas"), Amount(50.00)),
        Fund::savings(String::from("Emergency Fund"), 500.00),
    ];

    let transactions = vec![
        Transaction::deposit(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::withdrawal(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::deposit(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::withdrawal(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
    ];

    let future_transactions = vec![
        Transaction::deposit(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::withdrawal(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::deposit(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
        Transaction::withdrawal(18.34, String::from("Groceries"), String::from("HEB"), String::from("For kishik")),
    ];

    let mut checking = Account::new(String::from("BOA Checking"), Amount(823.00), funds, transactions, future_transactions);
    checking.setup();
    checking.print();

}
