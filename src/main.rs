use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
    
    println!("Devinez le nombre !");

    let nombre_secret: u32 = rand::rng().random_range(1..101);
    println!("Le nombre secret est : {}", nombre_secret);

    loop{
        println!("Veuillez entrer un nombre.");
        let mut supposition: String = String::new();
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");
        let supposition: u32 = supposition.trim().parse().expect("Veuillez entrer un nombre !");


        println!("Votre nombre : {}", supposition);
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}