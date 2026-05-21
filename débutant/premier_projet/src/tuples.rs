fn main() {
    // Tuple : taille fixe, types hétérogènes
    let point: (f64, f64) = (3.0, 4.5);
    println!("x = {}, y = {}", point.0, point.1);

    let (x, y) = point;   // destructuring
    println!("x = {}, y = {}", x, y);

    // Array : taille fixe, type homogène
    let notes: [i32; 4] = [15, 18, 12, 17];
    println!("Première note : {}", notes[0]);
    println!("Nombre de notes : {}", notes.len());

    for note in &notes {
        print!("{} ", note);
    }
    println!();

    // Vec : tableau dynamique (comme ArrayList en Java)
    let mut scores: Vec<i32> = Vec::new();
    scores.push(100);
    scores.push(85);
    scores.push(92);
    println!("Scores : {:?}", scores);  // {:?} = debug print
}
