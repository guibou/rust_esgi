/*
 * factorial(n) = n * (n - 1) * (n - 2) .... 1
 * factorial(0) = 1
 *
 * Cette implementation est recursive.
 */
fn factorial(n: i32) -> i32 {
    let res = if n == 1 {
        1
    } else {
        // Variable intermediaire, observez le 'let'
        let n2 = n - 1;

        n * factorial(n2)
    };

    res
}

// Implementation non récursive, avec `mut` pour les mutations
fn factorial2(mut n: i32) -> i32 {
    let mut res = 1;

    while n > 1
    {
        res *= n;
        n -= 1;
    }

    res
}

// Création d'un type Eleve, ayant deux champs.
// derive permet de generer automatiquement des comportements:
//
// Debug: println!("{:?}", monObject) => Affichage
// PartialEq: == et !=
// Eq: == et != pour toutes les valeurs
// Clone: .clone(): permet de dupliquer l'objet
#[derive(Debug, PartialEq, Eq, Clone)]
struct Eleve
{
    prenom: String,
    age: i32,
}

// cette fonction prend un Eleve par réference entrée
fn calcul_date_naissance(e : &Eleve) -> i32
{
    2021 - e.age
}

// Creation d'un type Couleur, avec 3 cas simples
enum Couleur
{
    Rouge, Vert, Bleu
}

// Creation d'un type Géometry, avec 2 cas. Observez que les cas peuvent être élaborés
enum Geometry
{
    Square(f32),
    Rectangle(f32, f32)
}

// Cette fonction travaille sur Geometry
fn geometry_coucou(toto: Geometry)
{
    // match permet de gerer les N cas.
    // Attention, un "match" doit être "total".
    match toto {
        Geometry::Square(i) => println!("C'était un carré de cote {}", i),
        Geometry::Rectangle(i, j) => println!("C'était un rectangle de cotés {} {}", i, j)
    }
}

fn main() {
    // Boucle for. La valeur de _i est ignorée.
    for _i in 1..10 {
        println!("Hello, world!")
    }

    // Attention, "fooo" n'est pas une string, il faut utiliser "to_string()".
    let guillaume = Eleve { prenom: "guillaume".to_string(), age: 34};

    println!("Guillaume, tu es né en {}", calcul_date_naissance(&guillaume));

    let ma_couleur_preferee = Couleur::Rouge;

    let bob = Geometry::Square(10.0);
    let toto = Geometry::Rectangle(20.0, 3.0);

    println!("{:?}", guillaume);
}
