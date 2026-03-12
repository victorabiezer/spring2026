mod bank_account; 
use bank_account::BankAccount; 

fn main() 
{
    let mut account = BankAccount::new(500.0);
    println!("Opened account with: ${}", account.balance());

    account.deposit(200.0);
    println!("Got paid! Deposited $500: ${}", account.balance());

    account.withdraw(100.0);
    println!("Bought groceries.. withdrew $200: ${}", account.balance());

    account.withdraw(9999.0); 
    println!("Bough a super expensive watch.. withdrew $99999: ${}", account.balance());

    account.deposit(-100.0);
    println!("Negative deposit.. ignore that: ${}", account.balance());

    account.apply_interest(0.05);
    println!("Bank added 5% interest: ${:.2}", account.balance()); 

    println!("Final Balance: ${:.2}", account.balance());
}