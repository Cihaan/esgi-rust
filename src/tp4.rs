use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Message, SmtpTransport, Transport};
use std::io::{self, Write};

// Structure pour représenter un email
#[derive(Debug)]
struct Email {
    from: String,
    to: String,
    subject: String,
    body: String,
}

// Implémentation des méthodes pour la structure Email
impl Email {
    // Constructeur
    fn new(from: String, to: String, subject: String, body: String) -> Self {
        Email {
            from,
            to,
            subject,
            body,
        }
    }

    // Méthode pour créer un message lettre à partir de notre structure
    fn to_message(&self) -> Result<Message, Box<dyn std::error::Error>> {
        let message = Message::builder()
            .from(self.from.parse()?)
            .to(self.to.parse()?)
            .subject(&self.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())?;

        Ok(message)
    }
}

// Structure pour représenter nos paramètres SMTP
struct SmtpConfig {
    username: String,
    password: String,
}

// Fonction pour lire les entrées de l'utilisateur
fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

// Fonction pour lire un mot de passe sans l'afficher
fn read_password(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    match rpassword::read_password() {
        Ok(password) => Ok(password),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

// Fonction pour envoyer un email
fn send_email(email: &Email, config: &SmtpConfig) -> Result<(), SmtpError> {
    // Créer les identifiants SMTP
    let creds = Credentials::new(config.username.clone(), config.password.clone());

    // Configurer le transport SMTP pour Gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Créer le message à partir de notre structure Email
    let message = email
        .to_message()
        .expect("Erreur lors de la création du message");

    // Envoyer l'email
    mailer.send(&message)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Programme d'envoi d'e-mails en Rust ===");

    // Valeurs codées en dur pour simplifier les tests
    let username = "EMAIL".to_string();
    let password = "APP_PASSWORD".to_string();

    let config = SmtpConfig {
        username: username.clone(),
        password,
    };

    // Informations de l'email en dur
    let from_email = format!("<{}>", username);
    let to_email = "guide7169@gmail.com".to_string();
    let subject = "Test d'envoi automatique depuis Rust".to_string();
    let body = "Ceci est un message de test envoyé depuis mon programme Rust.\n\nSalutations,\nRust Email Sender".to_string();

    // Créer l'email
    let email = Email::new(from_email, to_email, subject, body);

    // Afficher un résumé avant envoi
    println!("\nRésumé de l'e-mail:");
    println!("De: {}", email.from);
    println!("À: {}", email.to);
    println!("Sujet: {}", email.subject);
    println!("Corps:\n{}", email.body);

    // Envoi automatique sans confirmation
    println!("\nEnvoi automatique de l'e-mail...");
    match send_email(&email, &config) {
        Ok(_) => println!("✓ E-mail envoyé avec succès!"),
        Err(e) => println!("✗ Erreur lors de l'envoi de l'e-mail: {}", e),
    }

    Ok(())
}
