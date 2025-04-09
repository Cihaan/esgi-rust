fn main() {
    loop {
        println!("Toujours là !");
        let mut i = 0i32;

        loop {
            println!("sous-boucle !");
            i += 1;
            if i > 2 {
                return;
                // break;
            }
        }
    }
}

fn exo2() {
    let var = if true { 1u32 } else { 2u32 };
}

fn entier_et_float() -> (usize, f32) {
    (12, 0.1)
}

fn exo() {
    let tuple = entier_et_float();
    println!("entier: {}, float: {}", tuple.0, tuple.1)
}

fn age2() {
    let age: i32 = 18;

    match age {
        tmp if tmp > 60 => {
            println!("vieillard !")
        }
        tmp if tmp > 17 => {
            println!("majeur !")
        }
        _ => {
            println!("vieillard !")
        }
    }
}

fn age1() {
    let age: i32 = 18;

    match age {
        tmp if tmp > 60 => {
            println!("vieillard !")
        }
        tmp if tmp > 17 => {
            println!("majeur !")
        }
        _ => {
            println!("vieillard !")
        }
    }
}

fn age() {
    let age: i32 = 18;

    match age {
        17 => {
            println!("mineur !")
        }
        18 => {
            println!("majeur !")
        }
        _ => {
            println!("vieillard !")
        }
    }
}
