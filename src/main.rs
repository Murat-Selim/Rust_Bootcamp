// Define the Account Trait
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Define the BankAccount Struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account Trait for BankAccount
impl Account for BankAccount {
    // Implement the deposit method
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {} into account {}. New balance is: {}", amount, self.account_number, self.balance);
    }

    // Implement the withdraw method
    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew {} from account {}. New balance is: {}", amount, self.account_number, self.balance);
        } else {
            println!("Insufficient funds in account {} for withdrawal of {}. Current balance is: {}", self.account_number, amount, self.balance);
        }
    }

    // Implement the balance method
    fn balance(&self) -> f64 {
        self.balance
    }
}

// Main function to test the implementation
fn main() {
    // Create two BankAccount instances with different account numbers and holder names
    let mut account1 = BankAccount {
        account_number: 101,
        holder_name: String::from("Alice"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 102,
        holder_name: String::from("Bob"),
        balance: 300.0,
    };

    // Call the deposit method on the first account
    account1.deposit(150.0);

    // Call the withdraw method on the second account
    account2.withdraw(100.0);

    // Call the balance method on both accounts and print the result
    println!("Balance of {}'s account: {}", account1.holder_name, account1.balance());
    println!("Balance of {}'s account: {}", account2.holder_name, account2.balance());
}
