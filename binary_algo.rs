fn recherche_binaire(arr: &[i32], cible: i32) -> Option<usize> {
    let mut gauche = 0;
    let mut droite = arr.len();

    while gauche < droite {
        let milieu = gauche + (droite - gauche) / 2;
        if arr[milieu] == cible {
            return Some(milieu);
        } else if arr[milieu] < cible {
            gauche = milieu + 1;
        } else {
            droite = milieu;
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let cible = 7;
    match recherche_binaire(&arr, cible) {
        Some(index) => println!("Élément trouvé à l'indice: {}", index),
        None => println!("Élément non trouvé dans le tableau"),
    }
}
