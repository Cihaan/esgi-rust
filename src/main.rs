use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new() -> Self {
        BankAccount { balance: 0.0 }
    }

    fn load_from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error lors de la lecture du fichier de solde: {}", e))?;
        let balance: f64 = content
            .trim()
            .parse()
            .map_err(|_| "Balance invalide dans le fichier de solde".to_string())?;
        Ok(BankAccount { balance })
    }

    fn save_to_file(&self, path: &Path) -> Result<(), String> {
        fs::write(path, self.balance.to_string())
            .map_err(|e| format!("Erreur lors de la sauvegarde du solde: {}", e))
    }

    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Le montant de dépôt doit être positif".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Le montant de retrait doit être positif".to_string());
        }
        if amount > self.balance {
            return Err("Solde insuffisant".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let account_file = "account.txt";
    let path = Path::new(account_file);

    let mut account = match BankAccount::load_from_file(path) {
        Ok(acc) => acc,
        Err(_) => {
            println!(
                "Aucun compte existant trouvé, création d'un nouveau compte avec un solde de 0"
            );
            BankAccount::new()
        }
    };

    loop {
        println!("\nMenu du compte bancaire :");
        println!("1. Voir le solde");
        println!("2. Déposer de l'argent");
        println!("3. Retirer de l'argent");
        println!("4. Quitter");
        print!("Choisissez une option : ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Solde actuel : {:.2}", account.get_balance());
            }
            "2" => {
                print!("Entrez le montant à déposer : ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();

                match amount.trim().parse::<f64>() {
                    Ok(amt) => match account.deposit(amt) {
                        Ok(_) => println!("Dépôt réussi"),
                        Err(e) => println!("Erreur : {}", e),
                    },
                    Err(_) => println!("Montant saisi invalid"),
                }
            }
            "3" => {
                print!("Entrez le montant à retirer : ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();

                match amount.trim().parse::<f64>() {
                    Ok(amt) => match account.withdraw(amt) {
                        Ok(_) => println!("Retrait réussi"),
                        Err(e) => println!("Erreur : {}", e),
                    },
                    Err(_) => println!("Montant saisi invalid"),
                }
            }
            "4" => {
                match account.save_to_file(path) {
                    Ok(_) => println!("Compte sauvegardé avec succès"),
                    Err(e) => println!("Erreur lors de la sauvegarde du compte : {}", e),
                }
                println!("Fermeture...");
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}
