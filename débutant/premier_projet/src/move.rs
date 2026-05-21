fn main() {
    // MOVE : le ownership est transféré
    let s1 = String::from("hello");
    let s2 = s1;         // s1 est "moved" dans s2
    // println!("{}", s1); // ERREUR : s1 n'est plus valide !
    println!("{}", s2);  // OK

    // CLONE : copie profonde explicite
    let s3 = String::from("world");
    let s4 = s3.clone();    // copie indépendante
    println!("{} {}", s3, s4);  // OK

    // Types simples (i32, bool, f64...) sont COPIÉS automatiquement (trait Copy)
    let x = 5;
    let y = x;
    println!("{} {}", x, y);  // OK : x et y sont tous les deux valides
}
