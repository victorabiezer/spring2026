// 1. bank_account.rs 

#[derive(Debug)]
pub struct BankAccount
{
    balance: f64,
}

impl BankAccount
{
    pub fn new(initial_balance: f64) -> BankAccount
    {
        BankAccount
        {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64)
    {
        if amount > 0.0
        {
            self.balance += amount; 
        }
    }

    pub fn withdraw(&mut self, amount: f64)
    {
        if amount > 0.0 && amount <= self.balance
        {
            self.balance -= amount; 
        }
    }

    pub fn balance(&self) -> f64
    {
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64)
    {
        if rate > 0.0
        {
            self.balance += self.balance * rate; 
        }
    }
}


// tests 
#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn test_new_account()
    {
        let account = BankAccount::new(100.0); 
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit()
    {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw()
    {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0); 
        assert_eq!(account.balance(), 60.0);
    }

    #[test]
    fn test_negative_deposits()
    {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0);
    }
    
    #[test]
    fn test_negative_withdraw()
    {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-50.0);
        assert_eq!(account.balance(), 100.0); 
    }

    #[test]
    fn test_overdraft()
    {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert_eq!(account.balance(), 100.0); 
    }

    #[test]
    fn test_apply_interest()
    {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.10);
        assert!((account.balance() - 110.0).abs() < 1e-10);
    }
}
