# Using Enums in Rust

In Visual Studio Code, create a folder called **Enums** and then open it. Now open a new Terminal window.

## Exercises

Complete the following exercises.

## 09-0:  Traffic Light System

   - Create an `enum` named `TrafficLight` with three variants: `Red`, `Yellow`, and `Green`.
   - Implement a method `duration()` that returns the duration each light stays on (e.g., `Red` for 30 seconds, `Yellow` for 5 seconds, and `Green` for 60 seconds).

## 09-1:  Shape Calculation

   - Define an `enum` `Shape` with variants: `Circle`, `Square`, and `Rectangle`.
   - Each variant should **hold data** relevant to that shape (e.g., radius for `Circle`, side length for `Square`, and width and height for `Rectangle`).
   - Implement methods to calculate the area for each shape.

## 09-2:  Card Deck Management

   - Create an `enum` `Suit` for the four suits in a deck of cards: `Hearts`, `Diamonds`, `Clubs`, and `Spades`.
   - Create another `enum` `Rank` for the ranks in a deck (e.g., `Two`, `Three`, `Four`, up to `Ace`).
   - Define a `struct` `Card` that uses these [enums](/notes/13-enums/enums.md) to represent a playing card.
   - **Write a function** to create a full deck of cards (52 cards) and **another function** to shuffle this deck.
   - **Note:**  In order to complete this task, look at this [note](/notes/05-vectors/shuffle.md) on how to shuffle a vector in Rust.

# Enums with Data

As we have already seen, an enum in Rust is a custom data type that allows you to define a type by enumerating its possible variants. Each variant can optionally have data associated with it. Enums are useful when you need to work with a value that can be one of several possible types.  Here is an [example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8ee4ca084ef758fca098d857c353504) that models a simple bank account.

```rust
#[derive(Debug)]
enum Transaction {
    Deposit(f64),
    Withdrawal(f64),
}

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String) -> Self {
        BankAccount {
            owner,
            balance: 0.0,
        }
    }
  
    fn apply_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::Deposit(amount) => {
                self.balance += amount;
                println!("Deposited ${:.2}. New balance: ${:.2}.", amount, self.balance);
            }

            Transaction::Withdrawal(amount) => {
                if self.balance >= amount {
                    self.balance -= amount;
                    println!("Withdrew ${:.2}. New balance: ${:.2}.", amount, self.balance);
                } else {
                    println!("Insufficient funds! Current balance: ${:.2}.", self.balance);
                }
            }
        }
    }
}

  

fn main() {
    let mut account = BankAccount::new("Paul".to_string());

    println!("Account created for: {}. Initial balance: ${:.2}.", account.owner, account.balance);

    account.apply_transaction(Transaction::Deposit(100.0));
    account.apply_transaction(Transaction::Withdrawal(30.0));
    account.apply_transaction(Transaction::Withdrawal(80.0));
  
    println!("{:?}", account);

}
```

## 09-3:  Transaction History
Extend the **BankAccount** program to store a **history of transactions**. Use a `Vec<Transaction>` to log every transaction applied to the account. Add a **method** to display the transaction history in a user-friendly way.

## 09-4:  Account Types
Modify the program to support different account types using an **enum**. For example, you could add `enum AccountType { Savings, Checking, Business }` and store it in the **BankAccount** `struct`. Add logic to enforce different rules based on account type _(e.g., withdrawal limits for Savings accounts)_.

## 09-5:  Interest Calculation
Add a **method** to calculate interest on the account balance. Use an additional field in the **BankAccount** `struct` to store an interest rate. The method should apply the interest based on the account type or balance amount.