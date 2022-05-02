fn mad(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}

fn sum_from_to(a: i32, b: i32) -> i32 {
    let mut result = 0;
    let mut i = a;
    while i <= b {
        result += i;
        i += 1;
    }

    result
}

struct Livre {
    titre: String,
    année: u32,
    genre: Genre,
}

fn age_livre(l: Livre) -> u32 {
    2022 - l.année
}

enum Genre {
    Fiction,
    Histoire,
    Fantasy,
    Informatique,
}

fn score(l: Livre) -> u32 {
    let genre_score = match l.genre {
        Genre::Fiction => 12,
        Genre::Histoire => 15,
        Genre::Fantasy => 42,
        Genre::Informatique => 55,
    };

    let len: u32 = l.titre.len() as u32;
    (len + l.année) * genre_score
}

enum DivisionResult {
    DivisionByZero,
    DivisionOk(i32),
}

/*
 *
 *
 
 result = f(...);

match result
{
  None => ...
  Some(x) => x


 if(succes)
 {

 }
 else
 {

 }


 *
 */

fn division(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("Hello, world!");

    let a = Livre {
        titre: "Les poulets".to_string(),
        année: 2021,
        genre: Genre::Fiction,
    };
}
