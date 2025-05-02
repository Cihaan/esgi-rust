use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Produit {
    nom: String,
    quantite: u32,
}

fn ajouter_produit(inventaire: &mut Vec<Produit>) {
    print!("Nom du produit: ");
    io::stdout().flush().unwrap();
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).unwrap();
    let nom = nom.trim().to_string();

    print!("Quantité du produit: ");
    io::stdout().flush().unwrap();
    let mut quantite_str = String::new();
    io::stdin()
        .read_line(&mut quantite_str)
        .expect("Échec de la lecture de la ligne");
    let quantite: u32 = match quantite_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Quantité invalide. Veuillez entrer un nombre.");
            return;
        }
    };

    let produit = Produit { nom, quantite };
    inventaire.push(produit);
    println!("Produit ajouté.");
}

fn lister_produits(inventaire: &Vec<Produit>) {
    if inventaire.is_empty() {
        println!("L'inventaire est vide.");
    } else {
        println!("Inventaire:");
        for produit in inventaire {
            println!("{:?}", produit);
        }
    }
}

fn supprimer_produit(inventaire: &mut Vec<Produit>) {
    print!("Nom du produit à supprimer: ");
    io::stdout().flush().unwrap();
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).unwrap();
    let nom = nom.trim().to_string();

    let mut index = None;
    for (i, produit) in inventaire.iter().enumerate() {
        if produit.nom == nom {
            index = Some(i);
            break;
        }
    }

    match index {
        Some(i) => {
            inventaire.remove(i);
            println!("Produit supprimé.");
        }
        None => println!("Produit non trouvé."),
    }
}

fn modifier_produit(inventaire: &mut Vec<Produit>) {
    print!("Nom du produit à modifier: ");
    io::stdout().flush().unwrap();
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).unwrap();
    let nom = nom.trim().to_string();

    for produit in inventaire {
        if produit.nom == nom {
            print!("Nouvelle quantité: ");
            io::stdout().flush().unwrap();
            let mut quantite_str = String::new();
            io::stdin()
                .read_line(&mut quantite_str)
                .expect("Échec de la lecture de la ligne");
            let quantite: u32 = match quantite_str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Quantité invalide. Veuillez entrer un nombre.");
                    return;
                }
            };

            produit.quantite = quantite;
            println!("Quantité modifiée.");
            return;
        }
    }

    println!("Produit non trouvé.");
}

fn sauvegarder_inventaire(inventaire: &Vec<Produit>, filename: &str) {
    let mut contents = String::new();
    for produit in inventaire {
        contents.push_str(&format!("{},{}\n", produit.nom, produit.quantite));
    }
    match fs::write(filename, contents) {
        Ok(_) => println!("Inventaire sauvegardé dans {}.", filename),
        Err(e) => println!("Erreur lors de la sauvegarde de l'inventaire: {}", e),
    }
}

fn charger_inventaire(filename: &str) -> Vec<Produit> {
    let mut inventaire = Vec::new();

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let nom = parts[0].to_string();
                match parts[1].parse::<u32>() {
                    Ok(quantite) => {
                        let produit = Produit { nom, quantite };
                        inventaire.push(produit);
                    }
                    Err(_) => {
                        println!("Erreur de format dans le fichier pour la ligne: {}", line);
                    }
                }
            } else {
                println!("Erreur de format dans le fichier pour la ligne: {}", line);
            }
        }
    } else {
        println!("Fichier d'inventaire non trouvé ou erreur de lecture, démarrage avec un inventaire vide.");
    }

    inventaire
}

fn main() {
    let filename = "inventaire.txt";
    let mut inventaire = charger_inventaire(filename);

    loop {
        println!("\nMenu:");
        println!("1. Ajouter un produit");
        println!("2. Lister les produits");
        println!("3. Modifier un produit");
        println!("4. Supprimer un produit");
        println!("5. Sauvegarder et quitter");

        print!("Choix: ");
        io::stdout().flush().expect("Échec du flush stdout");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Échec de la lecture de la ligne");

        match choice.trim().parse::<u32>() {
            Ok(1) => ajouter_produit(&mut inventaire),
            Ok(2) => lister_produits(&inventaire),
            Ok(3) => modifier_produit(&mut inventaire),
            Ok(4) => supprimer_produit(&mut inventaire),
            Ok(5) => {
                sauvegarder_inventaire(&inventaire, filename);
                break;
            }
            _ => println!("Choix invalide. Veuillez entrer un nombre entre 1 et 5."),
        }
    }
}
