//fn main() {
//    println!("Hello, world!");
//}
//
//Import des bibliothèques
use rand::Rng; 

fn generate_password(length: usize) -> String {
    // Définir les caractères utilisables dans le mot de passe
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    // Créer un générateur de nombres aléatoires
    let mut rng = rand::thread_rng();

    // Générer le mot de passe
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}

fn main() {
    // Définir la longueur du mot de passe souhaitée
    let length = 12;

    // Générer le mot de passe
    let password = generate_password(length);

    // Afficher le mot de passe généré
    println!("Mot de passe généré : {}", password);
}
