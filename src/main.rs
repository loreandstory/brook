use brook::*;

fn main() {
    let funds = vec![
        Fund::income("Quality Logic".to_string(), 4111.60),
        Fund::budget("Groceries".to_string(), 250.00),
        Fund::budget("Car/Gas".to_string(), 275.00),
        Fund::budget("House/Clothing".to_string(), 150.00),
        Fund::budget("Entertainment".to_string(), 50.00),
        Fund::budget("Dining Out".to_string(), 150.00),
        Fund::budget("Christmas".to_string(), 100.00),
        Fund::savings("Lebanon".to_string(), 0.00, 500.00),
        Fund::savings("Emergency Fund".to_string(), 400.00, 1000.00),
    ];

    let transactions = vec![
        Transaction::withdrawal("02 Dec 21".to_string(), 45.00, "Dining Out".to_string(), "Osaka Sushi".to_string(), "Sushi date night".to_string()),
        Transaction::withdrawal("02 Dec 21".to_string(), 12.00, "Groceries".to_string(), "Target".to_string(), "Cake and supplies".to_string()),
        Transaction::withdrawal("02 Dec 21".to_string(), 100.00, "House/Clothing".to_string(), "Target".to_string(), "Winter clothes".to_string()),
        Transaction::withdrawal("02 Dec 21".to_string(), 44.64, "Groceries".to_string(), "Pasand".to_string(), "Arabic and Indian food supplies".to_string()),
        Transaction::deposit("03 Dec 21".to_string(), 822.32, "Quality Logic".to_string(), "Dec 3".to_string(), "Payday!".to_string()),
        Transaction::withdrawal("03 Dec 21".to_string(), 18.79, "Groceries".to_string(), "Target".to_string(), "General".to_string()),
        Transaction::withdrawal("03 Dec 21".to_string(), 4.86, "House/Clothing".to_string(), "Michael's".to_string(), "Brown wrapping paper".to_string()),
        Transaction::withdrawal("03 Dec 21".to_string(), 47.27, "Dining Out".to_string(), "Cedro".to_string(), "Italian date night".to_string()),
        Transaction::withdrawal("03 Dec 21".to_string(), 129.01, "Groceries".to_string(), "HEB".to_string(), "Groceries for meal plan".to_string()),
        Transaction::withdrawal("03 Dec 21".to_string(), 75.00, "Car/Gas".to_string(), "TX DMV".to_string(), "CR-V registration".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 31.87, "Car/Gas".to_string(), "HEB Gas".to_string(), "For Inifiniti".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 38.05, "House/Clothing".to_string(), "Target".to_string(), "Winter clothes".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 47.27, "Dining Out".to_string(), "Starbucks".to_string(), "Pepermint Mocha".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 4.18, "Groceries".to_string(), "HEB".to_string(), "Water gallons".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 16.25, "Groceries".to_string(), "Target".to_string(), "Groceries".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 10.00, "Christmas".to_string(), "Target".to_string(), "Christmas Candy".to_string()),
        Transaction::withdrawal("05 Dec 21".to_string(), 12.00, "Christmas".to_string(), "Dollar Tree".to_string(), "Christas candy packaging".to_string()),
    ];

    let future_transactions = vec![
        Transaction::deposit("2021-01-01".to_string(), 18.34, "Groceries".to_string(), "HEB".to_string(), "For kishik".to_string()),
        Transaction::withdrawal("2021-01-01".to_string(), 18.34, "Groceries".to_string(), "HEB".to_string(), "For kishik".to_string()),
        Transaction::deposit("2021-01-01".to_string(), 18.34, "Groceries".to_string(), "HEB".to_string(), "For kishik".to_string()),
        Transaction::withdrawal("2021-01-01".to_string(), 18.34, "Groceries".to_string(), "HEB".to_string(), "For kishik".to_string()),
    ];

    let mut checking = Account::new("BOA Checking".to_string(), Amount(823.00), funds, transactions, future_transactions);
    checking.print();

}
