use std::io;
use rand::Rng;

fn main() {

    println!("Devinez le nombre entre 1 et 100!");
    println!("Entrez votre nombre:");

    let nombre_Secret = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est: {}", nombre_Secret);

    let mut supposition = String::new();
    let user_input = "Alain";

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée");

    println!("Bonjour {}! Vous avez entré: {}", user_input, supposition.trim());


}
