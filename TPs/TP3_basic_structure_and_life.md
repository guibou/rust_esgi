Ce TP vous fera manipuler quelques types de la libraire standard, à savoir, `Option<T>`, `String`, `Vec<T>`.

Ces types sont fondamentaux dans tout programme et vous permettrons d'avoir une première expérience avec la librairie standard, le système de module et la durée de vie.

Commencez par créer un projet rust pour ce TP :

```rust
$ cargo new tp3
```

# String

`String` permet de reprendre une chaine de caractère de taille variable, mutable.

Vous trouverez la documentation ici: https://doc.rust-lang.org/std/string/struct.String.html

Celle-ci est très complète et contient de nombreux exemples.

Vous pouvez créer une `String` à partir d'un `str` (i.e. une chaine littérale) avec `"foo".to_string()` ou `String::from("foo")`.

1. Créer plusieurs string avec `let`, le nom des variables et leur contenu est laissée à votre imagination.
2. Affichez les plusieurs fois avec `println!`
3. Créer une fonction `fn isbig(input: String, taille: usize) -> bool`. Cette fonction prendra en entrée une `string` et retourne `true` si la longueur de la chaine fait plus de `taille` charactères. Vous pouvez utiliser `s.len()`.

Par exemple `isbig("guillaume".to_string(), 10) == false`

4. Appelez la fonction une fois sur votre string et observer le résultat avec `println!`, comme ceci:

```rust
    let name = "guillaume".to_string();
    println!("{}", isbig(name, 10));
```

5. Appelez la fonction une seconde fois, comme ceci:

```rust
    let name = "guillaume".to_string();
    println!("{}", isbig(name, 10));
    println!("{}", isbig(name, 5));
```

Vous observez une erreur, ici, `name` n'est plus accessible car "moved" lors du premier appel.

6. `String` n'est pas "copiable", c'est un gros objet, ainsi la copie doit être explicitement demandée avec `Clone`. Utilisez `name.clone()` afin de faire fonctionner le code précédant.

7. `Clone` est couteux, la chaine doit être dupliqué. Ce n'est cependant pas nécessaire. Nous pouvons utiliser une référence. Changez le type de `isbig` vers `fn isbig(s: &String, n: usize) -> bool` et suivez les instructions du compilateur pour faire marcher votre code sans le `clone`.

Bravo, vous avez fait vos premiers pas avec les chaines de caractère et la durée de vie (`Copy`, Move, `Clone`, References). Vous devez déjà avoir une expérience de la manipulation de chaine dans d'autres langages. Regardez la documentation rust, demander à votre moteur de recherche, vous devriez être capable de produire des résultats similaires.

# Option<T>

https://doc.rust-lang.org/std/option/

`Option<T>` est un type contenant deux valeurs. `None` ou `Some(x)` ou `x` est une valeur de type `T`.

Par exemple, un `Option<u32>` peut contenir `None`, ou `Some(10)` ou `Some(123)`.

On peut créer des valeurs avec `None` et `Some(x)`.

On peut `matcher` dessus:

```rust
let v = Some(10);

match v {
   None => println!("Vide"),
   Some(x) => println!("Pas vide: {}", x)
}
```

Ce type est pratique pour représenter une fonction qui peut "échouer".

Note: la syntaxe `<T>` permet de représenter un type polymorphic. Similaire aux templates C++, aux générique Java ou aux polymorphisme dans de nombreux langages. Ne vous en préoccupez pas pour l'instant, sachez seulement que changer le `T` dans `Option<T>` change le type contenu dans l'option (i.e. la valeur `x` dans `Some(x)`).

1. rust admet l'opérateur `/` entre `f32` pour faire une division. Ecrivez une fonction `fn inverse(v: f32) -> f32` qui retourne l'inverse (i.e. `1 / v`). Tester cette fonction avec plusieurs valeurs (et `println!`).
2. Que se passe-t-il quand on calcul `inverse(0)`? afficher le résultat du calcul avec `println!()`.
3. Nous voulons écrire une fonction `fn safe_inverse(v: f32) -> Option<f32>`. Celle-ci renvoie `Some(x)` quand la valeur est non infinie, et `None` quand la valeur est infinie.

Bravo, vous avez découvert `Option<T>`. Ce type est utilisé pour représenter l'échec d'une fonction dans rust, là ou d'autres langages pourraient utiliser les exceptions. Une variante existe, `Result<T, E>`, très similaire, qui permet de représenter la réussite avec une valeur, ou l'échec avec une valeur d'échec, là ou `Option<T>` reste vague sur les raisons de l'échec (i.e. `None`).

# Vec<T>

https://doc.rust-lang.org/std/vec/struct.Vec.html

`Vec<T>` représente un vecteur, ou tableau de valeurs de type `T`. Par example, `Vec<f32>` est un tableau de float, alors que `Vec<String>` est un tableau de `String.`

On peut créer un vecteur avec `vec!`. Exemple:

```rust
let v = vec![1,2,3,4];
```

Note: ne faites pas attention a cette syntaxe particulière ou au `!`, c'est une "macro" rust. Ignorez ce détail pour l'instant.


1. Créer un vecteur contenant quelques valeurs entières. Vous pouvez l'afficher avec `println!`. Attention, il faudra utiliser `{:?}` comme formateur et non plus `{}`. Le formateur `{}` est adapté au type pour lesquels il existe une façon évidente de s'afficher, comme c'est le cas pour un nombre, e.g. `1`.

2. Vous pouvez itérer sur les éléments d'un vecteur avec `for i in monVecteur { ... }`. Afficher une à une les valeurs de votre vecteur précédant avec `println!`.

3. Un vecteur peut être modifié. Vous pouvez créer un vecteur `mut`able en ajoutant `mut` au `let`:

```rust
let mut v2 = vec![3, 7];
```

Vous pourrez ensuite ajouter des valeurs manuellement dans le vecteur avec `v.push(x)`.

4. Ajoutez manuellement quelques valeurs dans votre vecteur. Utilisez `println!` pour tester.

5. Écrivez une fonction `fn somme(values : Vec<u32>) -> u32`. Celle-ci calcul la somme des éléments d'un vecteur. Testez avec `println!`. Vous pourrez utiliser un accumulateur `mut`able (i.e. `let mut res = 0`) et une boucle `for i in v`.

6. Comme dans l'exercice sur les `String`, observez que vous ne pouvez pas appeler la fonction `somme` deux fois de suite, comme ceci:

```rust
let mut v = vec![1,2];
println!("{}", somme(v));
v.push(3);
println!("{}", somme(v));
```

Modifiez votre code, en utilisant `.clone()` puis des références, pour pouvoir compiler et exécuter une variante du code précédant.

# Tout ensemble

1. Écrire une fonction `fn maximum(values: &Vec<u32>) -> Option<u32>` qui renvoie la valeur maximum contenue dans un vecteur. Pourquoi utiliser `Option` ici ?

Vous utiliserez sans doute `v.len()`, ainsi que `v[0]` pour récupérer la première valeur du vecteur. Vous pouvez aussi utiliser `v.first()` https://doc.rust-lang.org/std/vec/struct.Vec.html#method.first.

Discutez ensemble de vos solutions. Sachez que `v[0]` peut échouer (si votre vecteur est vide). La solution à base de `first` vous semble-t-elle plus robuste ? 
