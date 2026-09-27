#[derive(Debug, PartialEq)]
enum TransactionStatus {
    Success,
    Failed,
}

struct Wallet {
    owner: String,
    balance: f64,
    transactions: u32,
}

impl Wallet {
    fn transfer(&mut self, amount: f64) -> Result<TransactionStatus, String> {
        if amount > self.balance {
            return Err(String::from("Not enough SOL"));
        }

        self.balance -= amount;
        self.transactions += 1;

        Ok(TransactionStatus::Success)
    }
}

fn print_wallet(wallet: &Wallet) {
    println!("--- Solana Wallet ---");
    println!("Owner: {}", wallet.owner);
    println!("Balance: {:.6} SOL", wallet.balance);
    println!("Transactions: {}", wallet.transactions);
}

fn main() {
    let mut wallet = Wallet {
        owner: String::from("Balnur"),
        balance: 0.5,
        transactions: 0,
    };

    print_wallet(&wallet);

    println!("\nSending 0.1 SOL...");

    match wallet.transfer(0.1) {
        Ok(status) => println!("Transaction status: {:?}", status),
        Err(error) => println!("Error: {}", error),
    }

    print_wallet(&wallet);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_transfer() {
        let mut wallet = Wallet {
            owner: String::from("Balnur"),
            balance: 0.5,
            transactions: 0,
        };

        let result = wallet.transfer(0.1);

        assert_eq!(result, Ok(TransactionStatus::Success));
        assert_eq!(wallet.transactions, 1);
    }

    #[test]
    fn failed_transfer() {
        let mut wallet = Wallet {
            owner: String::from("Balnur"),
            balance: 0.5,
            transactions: 0,
        };

        let result = wallet.transfer(1.0);

        assert_eq!(result, Err(String::from("Not enough SOL")));
    }
}