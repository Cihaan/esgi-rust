use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone)]
struct Pokemon {
    name: String,
    level: u32,
    pokemon_type: PokemonType,
    xp: u32,
    gender: Gender,
}

impl Pokemon {
    fn new(name: &str, level: u32, pokemon_type: PokemonType, gender: Gender) -> Self {
        Pokemon {
            name: name.to_string(),
            level,
            pokemon_type,
            xp: 0,
            gender,
        }
    }

    fn gain_xp(&mut self, amount: u32) {
        self.xp += amount;
        while self.xp >= 100 {
            self.xp -= 100;
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
    }

    fn can_breed(&self, other: &Pokemon) -> bool {
        self.pokemon_type == other.pokemon_type
            && self.gender != other.gender
            && self.level >= 5
            && other.level >= 5
    }

    fn breed(pokemon1: &Pokemon, pokemon2: &Pokemon) -> Option<Pokemon> {
        if pokemon1.can_breed(pokemon2) {
            Some(Pokemon::new(
                "Mystere",
                1,
                pokemon1.pokemon_type.clone(),
                if rand::random() {
                    Gender::Male
                } else {
                    Gender::Female
                },
            ))
        } else {
            None
        }
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (Niveau {} - {} - XP: {} - {})",
            self.name,
            self.level,
            match self.pokemon_type {
                PokemonType::Fire => "Feu",
                PokemonType::Water => "Eau",
                PokemonType::Grass => "Plante",
                PokemonType::Electric => "Electrik",
            },
            self.xp,
            match self.gender {
                Gender::Male => "Male",
                Gender::Female => "Femelle",
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Breeding {
    pokemon_list: Vec<Pokemon>,
}

impl Breeding {
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let content = fs::read_to_string(filename)?;
        let breeding: Breeding = serde_json::from_str(&content)?;
        Ok(breeding)
    }

    fn filter_by_level(&self, min_level: u32) -> Vec<&Pokemon> {
        self.pokemon_list
            .iter()
            .filter(|p| p.level >= min_level)
            .collect()
    }

    fn filter_by_type(&self, pokemon_type: &PokemonType) -> Vec<&Pokemon> {
        self.pokemon_list
            .iter()
            .filter(|p| p.pokemon_type == *pokemon_type)
            .collect()
    }

    fn new() -> Self {
        Breeding {
            pokemon_list: Vec::new(),
        }
    }

    fn add_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemon_list.push(pokemon);
    }

    fn display_all(&self) {
        for pokemon in &self.pokemon_list {
            println!("{}", pokemon);
        }
    }

    fn train_all(&mut self, xp_amount: u32) {
        for pokemon in &mut self.pokemon_list {
            pokemon.gain_xp(xp_amount);
        }
    }

    fn attempt_breeding(&mut self, index1: usize, index2: usize) -> Option<Pokemon> {
        if index1 < self.pokemon_list.len() && index2 < self.pokemon_list.len() {
            let pokemon1 = &self.pokemon_list[index1];
            let pokemon2 = &self.pokemon_list[index2];
            Pokemon::breed(pokemon1, pokemon2)
        } else {
            None
        }
    }
}

fn main() {
    let mut breeding = Breeding::new();

    breeding.add_pokemon(Pokemon::new(
        "Salamèche",
        5,
        PokemonType::Fire,
        Gender::Male,
    ));
    breeding.add_pokemon(Pokemon::new(
        "Carapuce",
        6,
        PokemonType::Water,
        Gender::Female,
    ));
    breeding.add_pokemon(Pokemon::new(
        "Bulbizarre",
        7,
        PokemonType::Grass,
        Gender::Male,
    ));
    breeding.add_pokemon(Pokemon::new(
        "Pikachu",
        5,
        PokemonType::Electric,
        Gender::Female,
    ));

    println!("Pokémons dans l'élevage:");
    breeding.display_all();

    println!("\nEntraînement de tous les Pokémons (+50 XP):");
    breeding.train_all(50);
    breeding.display_all();

    println!("\nTentative de reproduction entre Salamèche et Pikachu:");
    if let Some(baby) = breeding.attempt_breeding(0, 3) {
        println!("Nouveau Pokémon né: {}", baby);
        breeding.add_pokemon(baby);
    } else {
        println!("Ces Pokémons ne peuvent pas se reproduire!");
    }

    println!("\nÉtat final de l'élevage:");
    breeding.display_all();

    if let Err(e) = breeding.save_to_file("pokemon_save.json") {
        println!("Erreur lors de la sauvegarde: {}", e);
    } else {
        println!("\nProgression sauvegardée!");
    }

    match Breeding::load_from_file("pokemon_save.json") {
        Ok(loaded_breeding) => {
            println!("\nChargement de la sauvegarde:");
            loaded_breeding.display_all();

            println!("\nPokémons de niveau 6 ou plus:");
            for pokemon in loaded_breeding.filter_by_level(6) {
                println!("{}", pokemon);
            }

            println!("\nPokémons de type Feu:");
            for pokemon in loaded_breeding.filter_by_type(&PokemonType::Fire) {
                println!("{}", pokemon);
            }
        }
        Err(e) => println!("Erreur lors du chargement: {}", e),
    }
}
