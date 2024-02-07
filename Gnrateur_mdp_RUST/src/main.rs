// import
use rand::Rng;
use std::io;

// Fonction pour générer un mot de passe aléatoire
fn generate_password(length: usize, use_uppercase: bool, use_special_chars: bool, use_numbers: bool) -> String {
    // Définir les caractères utilisables dans le mot de passe
    let mut charset = "abcdefghijklmnopqrstuvwxyz".to_string();
    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); // Ajouter des majuscules
    }
    if use_special_chars {
        charset.push_str("!@#$%^&*()"); // Ajouter des caractères spéciaux
    }
    if use_numbers {
        charset.push_str("0123456789"); // Ajouter des chiffres
    }

    // Créer un générateur de nombres aléatoires
    let mut rng = rand::thread_rng();

    // Générer le mot de passe
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap() // Sélectionner un caractère aléatoire à partir du jeu de caractères
        })
        .collect();

    password // Retourner le mot de passe généré
}

fn main() {
    println!("Générateur de mot de passe");

    // Définir la longueur minimale du mot de passe
    let min_length = 10;

    // Demander la longueur du mot de passe à l'utilisateur
    let length: usize;
    loop {
        println!("Entrez la longueur du mot de passe (minimum {} caractères) : ", min_length);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        length = match input.trim().parse() {
            Ok(num) if num >= min_length => num,
            _ => {
                println!("Veuillez entrer un entier positif supérieur ou égal à {}.", min_length);
                continue;
            }
        };
        break;
    }

    // Demander à l'utilisateur s'il veut inclure des majuscules
    println!("Ajouter des majuscules ? (o/n) : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let use_uppercase = input.trim().to_lowercase() == "o";

    // Demander à l'utilisateur s'il veut inclure des caractères spéciaux
    println!("Ajouter des caractères spéciaux ? (o/n) : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let use_special_chars = input.trim().to_lowercase() == "o";

    // Demander à l'utilisateur s'il veut inclure des chiffres
    println!("Ajouter des chiffres ? (o/n) : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let use_numbers = input.trim().to_lowercase() == "o";

    // Générer le mot de passe
    let password = generate_password(length, use_uppercase, use_special_chars, use_numbers);

    // Afficher le mot de passe généré
    println!("Mot de passe généré : {}", password);
}
