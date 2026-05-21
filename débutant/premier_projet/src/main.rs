fn main() {
    let x = 5 ;
    let mut y = 10 ; // mutable
    y = 20 ;// ok
    // x =  6 ERREUR car immuable
    let z : i32 = 42 ; // avec a notation de type explicite
}

// i32 entier signé 32 bit        -> let n: i32 = -5 ;
// i64 entier signé 64 bits       -> let n:i64 = 1_000_00 ;
// u32 entier non signé 32 bits   -> let n:u32 = 42 ;
// f64 flottant 64 bits           -> let x: f64 = 3.14

// bool booléan                   -> let b:bool = true ;
// char caractère Unicode         -> let c:char = 'è' ;
// String chaîne de caractères (allouée) ->let s = String::from("hello");
// &str slice de chaîne (référence) ->let s: &str = "hello";


fn main2() {
    let a: i32 = 100;
    let b: f64 = 3.14;
    let flag: bool = true;
    let lettre: char = 'R';

    // Affichage avec println! (c'est une macro, d'où le !)
    println!("a = {}, b = {:.2}, flag = {}, lettre = {}", a, b, flag, lettre);
    // Sortie : a = 100, b = 3.14, flag = true, lettre = R

    // Conversion de types (pas de cast implicite en Rust !)
    let x: i32 = 10;
    let y: f64 = x as f64;   // cast explicite avec `as`
    println!("y = {}", y);   // 10.0
}


// Syntaxe : fn nom(param: type) -> type_retour { ... }
fn additionner(a: i32, b: i32) -> i32 {
    a + b   // PAS de point-virgule = valeur retournée (comme en OCaml !)
}

fn saluer(nom: &str) {   // &str = référence vers une chaîne (lecture seule)
    println!("Bonjour, {} !", nom);
}

fn main3() {
    let resultat = additionner(3, 7);
    println!("3 + 7 = {}", resultat);  // 10

    saluer("Mathis");
}

fn main4() {
    // loop : boucle infinie (break pour sortir)
    let mut compteur = 0;
    let resultat = loop {
        compteur += 1;
        if compteur == 5 {
            break compteur * 2;   // break peut retourner une valeur !
        }
    };
    println!("Résultat = {}", resultat);  // 10

    // while
    let mut n = 3;
    while n > 0 {
        println!("{}...", n);
        n -= 1;
    }

    // for (le plus courant)
    for i in 0..5 {           // 0, 1, 2, 3, 4 (exclusif)
        print!("{} ", i);
    }
    println!();

    for i in 0..=5 {          // 0, 1, 2, 3, 4, 5 (inclusif)
        print!("{} ", i);
    }
    println!();
}
