use brook::*;

fn main() {
    let funds = vec![
        Fund::new(String::from("Groceries"), Amount(200.00)),
        Fund::new(String::from("Gas"), Amount(50.00)),
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

    let checking = Account::new(String::from("BOA Checking"), Amount(823.00), funds, transactions, future_transactions);
    checking.print();

}
