use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    isbn: String,
    annee_publication: u32,
}

impl Livre {
    fn to_string(&self) -> String {
        format!(
            "{};{};{};{}",
            self.titre, self.auteur, self.isbn, self.annee_publication
        )
    }

    fn from_string(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 4 {
            return Err("Format de ligne invalide".to_string());
        }

        let annee = parts[3]
            .parse::<u32>()
            .map_err(|_| "Année de publication invalide".to_string())?;

        Ok(Livre {
            titre: parts[0].to_string(),
            auteur: parts[1].to_string(),
            isbn: parts[2].to_string(),
            annee_publication: annee,
        })
    }
}

#[derive(Debug)]
struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Bibliotheque {
    fn new() -> Self {
        Bibliotheque { livres: Vec::new() }
    }

    fn load_from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| {
            format!(
                "Erreur lors de la lecture du fichier de bibliothèque: {}",
                e
            )
        })?;

        let mut bibliotheque = Bibliotheque::new();

        for line in content.lines() {
            if !line.trim().is_empty() {
                let livre = Livre::from_string(line)?;
                bibliotheque.livres.push(livre);
            }
        }

        Ok(bibliotheque)
    }

    fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let content: String = self
            .livres
            .iter()
            .map(|livre| livre.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        fs::write(path, content)
            .map_err(|e| format!("Erreur lors de la sauvegarde de la bibliothèque: {}", e))
    }

    fn ajouter_livre(&mut self, livre: Livre) -> Result<(), String> {
        if self.livres.iter().any(|l| l.isbn == livre.isbn) {
            return Err(format!(
                "Un livre avec l'ISBN {} existe déjà dans la bibliothèque",
                livre.isbn
            ));
        }
        self.livres.push(livre);
        Ok(())
    }

    fn rechercher_par_titre(&self, titre: &str) -> Vec<&Livre> {
        self.livres
            .iter()
            .filter(|livre| livre.titre.to_lowercase().contains(&titre.to_lowercase()))
            .collect()
    }

    fn retirer_livre(&mut self, isbn: &str) -> Result<Livre, String> {
        let position = self.livres.iter().position(|livre| livre.isbn == isbn);

        match position {
            Some(pos) => Ok(self.livres.remove(pos)),
            None => Err(format!("Aucun livre avec l'ISBN {} n'a été trouvé", isbn)),
        }
    }

    fn afficher_tous_les_livres(&self) -> &Vec<Livre> {
        &self.livres
    }
}

fn main() {
    let bibliotheque_file = "bibliotheque.txt";
    let path = Path::new(bibliotheque_file);

    let mut bibliotheque = match Bibliotheque::load_from_file(path) {
        Ok(bib) => bib,
        Err(e) => {
            println!("Création d'une nouvelle bibliothèque: {}", e);
            Bibliotheque::new()
        }
    };

    loop {
        println!("\nGestion de Bibliothèque :");
        println!("1. Afficher tous les livres");
        println!("2. Ajouter un livre");
        println!("3. Rechercher un livre par titre");
        println!("4. Retirer un livre");
        println!("5. Quitter");
        print!("Choisissez une option : ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let livres = bibliotheque.afficher_tous_les_livres();
                if livres.is_empty() {
                    println!("La bibliothèque est vide.");
                } else {
                    println!("Livres dans la bibliothèque :");
                    for (i, livre) in livres.iter().enumerate() {
                        println!(
                            "{}. {} par {}, ISBN: {}, Année: {}",
                            i + 1,
                            livre.titre,
                            livre.auteur,
                            livre.isbn,
                            livre.annee_publication
                        );
                    }
                }
            }
            "2" => {
                println!("Ajout d'un nouveau livre :");

                print!("Titre : ");
                io::stdout().flush().unwrap();
                let mut titre = String::new();
                io::stdin().read_line(&mut titre).unwrap();

                print!("Auteur : ");
                io::stdout().flush().unwrap();
                let mut auteur = String::new();
                io::stdin().read_line(&mut auteur).unwrap();

                print!("ISBN : ");
                io::stdout().flush().unwrap();
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).unwrap();

                print!("Année de publication : ");
                io::stdout().flush().unwrap();
                let mut annee = String::new();
                io::stdin().read_line(&mut annee).unwrap();

                match annee.trim().parse::<u32>() {
                    Ok(annee_pub) => {
                        let livre = Livre {
                            titre: titre.trim().to_string(),
                            auteur: auteur.trim().to_string(),
                            isbn: isbn.trim().to_string(),
                            annee_publication: annee_pub,
                        };

                        match bibliotheque.ajouter_livre(livre) {
                            Ok(_) => println!("Livre ajouté avec succès."),
                            Err(e) => println!("Erreur : {}", e),
                        }
                    }
                    Err(_) => println!("Année de publication invalide."),
                }
            }
            "3" => {
                print!("Entrez le titre à rechercher : ");
                io::stdout().flush().unwrap();
                let mut titre = String::new();
                io::stdin().read_line(&mut titre).unwrap();

                let resultats = bibliotheque.rechercher_par_titre(titre.trim());
                if resultats.is_empty() {
                    println!("Aucun livre trouvé avec ce titre.");
                } else {
                    println!("Livres trouvés :");
                    for (i, livre) in resultats.iter().enumerate() {
                        println!(
                            "{}. {} par {}, ISBN: {}, Année: {}",
                            i + 1,
                            livre.titre,
                            livre.auteur,
                            livre.isbn,
                            livre.annee_publication
                        );
                    }
                }
            }
            "4" => {
                print!("Entrez l'ISBN du livre à retirer : ");
                io::stdout().flush().unwrap();
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).unwrap();

                match bibliotheque.retirer_livre(isbn.trim()) {
                    Ok(livre) => println!("Le livre '{}' a été retiré.", livre.titre),
                    Err(e) => println!("Erreur : {}", e),
                }
            }
            "5" => {
                match bibliotheque.save_to_file(path) {
                    Ok(_) => println!("Bibliothèque sauvegardée avec succès."),
                    Err(e) => println!("Erreur lors de la sauvegarde de la bibliothèque : {}", e),
                }
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }
}
