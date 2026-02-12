use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Devinez le nombre entre 1 et 100!");
    //println!("Entrez votre nombre:");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est: {}", nombre_secret);

    let mut supposition = String::new();


    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée");

    let supposition: u32 = match supposition.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Vous avez entré: {}", supposition);

    loop {
        println!("Entrez votre nombre:");

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus"),
            Ordering::Greater => println!("C'est moins"),
            Ordering::Equal => {
                println!("Félicitations! Vous avez deviné le nombre secret!");
                break;
            }
        }
    }
        
}
