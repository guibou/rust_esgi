fn factorial(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * factorial(i - 1)
    }
}

fn factorial_while(mut i: i32) -> i32 {
    let mut resultat = 1;

    while i > 0 {
        resultat *= i;
        i = i - 1;
    }

    resultat
}

fn factorial_for(mut i: i32) -> i32 {
    let mut resultat = 1;

    for n in 1..(i + 1) {
        resultat *= n;
    }

    resultat
}

fn toto(a: i32, b: i32) -> i32 {
    a * b
}

fn factorial_rusttttt(i: i32) -> i32 {
    (1..(i + 1)).product()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Red,
    Green,
    Blue,
}

enum ColorWeb {
    NamedColor(Color),
    HexColor(f32, f32, f32)
}

enum Ip {
    Ipv4(u8, u8, u8, u8),
    Upv6(u8)
}
#[derive(Debug)]
struct Rectangle{
    width: f32,
    height: f32
}

#[derive(Debug)]
enum Shape {
    RectangleShape(Rectangle),
    Circle { radius: f32},
    Square { side: f32 }
}

fn shape_area(s : Shape) -> f32
{
    match s
    {
        Shape::RectangleShape (Rectangle { width:w, height:h }) => w * h,
        Shape::Square { side } => side * side,
        Shape::Circle { radius } => 3.14 * radius * radius
    }
}

fn color_to_string(c: Color) -> () {
    let s = match c {
        Color::Red => "Red",
        Color::Green => "Vert",
        Color::Blue => "Jurassica park",
    };

    println!("{}", s)
}

fn distance2(p0: Point, p1: Point) -> i32 {
    let dx = p0.x - p1.x;
    let dy = p0.y - p1.y;
    let dz = p0.z - p1.z;
    dx * dx + dy * dy + dz * dz
}

fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        println!("fact({i}): {}", factorial(i));
        println!("fact({i}): {}", factorial_rusttttt(i));
    }

    let p = Point { x: 1, y: 2, z: 3 };
    let p2 = Point { x: 5, y: 2, z: 3 };
    println!("Mon point est: {p:?}");
    println!("Ma distance est {}", distance2(p, p2));

    let c = Color::Red;

    let s0 = Shape::RectangleShape(Rectangle { width: 2.0, height: 3.0 });
    let s1 = Shape::Square { side: 10.0 };
    let s2 = Shape::Circle { radius: 2.5 };
}
