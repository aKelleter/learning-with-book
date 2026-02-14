// Je voudrais écrire une fonction qui calcile la moyenne de plusieurs nombres.

fn moyenne(numbers: &[f64]) -> f64 {
   // Vérifier que le tableau n'est pas vide pour éviter une division par zéro
    if numbers.is_empty() {
        return 0.0; // ou vous pouvez choisir de retourner une valeur spéciale ou de lancer une erreur
    }
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn main() {
    
    // Test de la fonction moyenne
    let numbers = [10.0, 20.0, 30.0, 40.0, 50.0];
    let avg = moyenne(&numbers);
    println!("The average of {:?} is {}", numbers, avg);


}
