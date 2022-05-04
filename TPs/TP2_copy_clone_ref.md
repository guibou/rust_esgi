# Copy / Clone / Reference

Ce sujet de TP est un sujet interactif. Des diagrammes seront rajoutés pendant le cours.

## Gestion des ressources

- Création / Suppression
- "Durée de vie"

- Exemples :

  - Mémoire (exemple principal)
  - Fichiers
  - Mutex

## Problematique

- Création: explicite
- Destruction?

  - Automatique, GC (Python, C#, Java, Haskell, ...)
  - Manuel (C, C++)
  - Système de type (Rust): *SAFE*

```
let c = 10
let mut b = &c;
{
  let a = 10;
  if(...)
  {
     b = &a;
  }
}
println!("{}", b);
```


## Gestion Automatique à l'exécution

- Exemple en python (comptage de référence)

```
def greet(nom, x):
  i = 0
  while i < x:
    i = i + 1
    print(nom)


a = 10
b = a
s = "Guillaume"
greet(s, b)


del a
del s

del b
```

- Simple (pour la/le dev)
- Difficile pour l'implémentation
- Performance ?

## Gestion manuelle

- Exemple en C

```
char *a = malloc(10);

...

free(a);
printf("%s\n", a); 
```

- Complexe pour la/le dev.
- Simple pour l'implémentation
- Sources d'erreurs

## Gestion de Rust

- Allocation / deallocation décidée à la compilation
- Le compilateur "suit" la durée de vie.

- La/le dev ne peut pas se tromper, l'implémentation ne peut pas être fausse.
- Plus dur d'avoir du code qui "tourne".
- Performance.

- `Copy`, `Clone`, Move, `&`

# Rust `Copy`

```
fn foo(a: u32) -> u32
{
  a * 2
}

...
let b = 10;
let j = b;
let c = foo(b);
let d = foo(j);
println!("{} {}", c, d);
```

- Ici, 10 est "copié" dans `b`.
- Le contenu de `b` est copié dans `j`.
- Puis le contenu de `b` est copié dans `a`.
- Puis `a * 2` est calculé et cette valeur est copiée dans `c`.
- Puis le contenu de `j` est copié dans `a` (pas le même `a`).
- Puis `a * 2` est calculé et cette valeur est copiée dans `d`.

- `a` est vivant du début de la fonction jusqu'à sa fin.
- `b` est vivant jusqu'au premier appel à `foo`.
- `j` est vivant jusqu'au second appel à `foo`.
- `c` est vivant jusqu'au `println!`
- `d` est vivant jusqu'au `println!`

`Copy` est un `trait` derivable: `#[derive(Copy)]`.


```
#[derive(Copy)]
struct Animal
{
   age: u32,
   taille: f32
}
```

Tous les types primitifs "simples" sont copiables.
Tout type composé de type copyable permet de faire un Derive(Copy).

`Copy` c'est simple. Mais:

- Copier n'est pas possible sur tous les types et peut être couteux. Exemple `vec<u32>` de 10K items.
- Copier est le comportement par défaut *implicite* lors d'un appel de fonction ou d'une affectation.

# Rust `Clone`

`Clone` c'est comme `Copy`, mais c'est explicite.

- Vous pouvez dériver `Clone` sur n'importe quel type: `#[derive(Clone)]` (dont tout les sous type ont `Clone`).
- Appel *explicite*.

```
let a = "hello".to_string();
let b = a.clone();
println!("{}", b);
println!("{}", a);
```

`a` et `b` sont des copies. Il y a deux chaines en mémoire.

C'est "couteux".

La durée de vie est la même que pour `Copy`:

- `a` est vivant jusqu'au second `println!`
- `b` est vivant jusqu'au premier `println!`

# Rust move

Lors d'un appel du fonction ou d'une affectation, si `Copy` n'est pas possible. Alors rust va faire un move.

Lors d'un move, l'objet "source" "meurt".

```
fn bar(s: String) -> String
{
   s
}

let a = "hello".to_string();
let b = a;
let c = bar(b);

// Lignes fausses, a et b n'existent plus.
// let d = bar(a);
// let e = bar(b);

println!("{}", c);
```

- `"hello"` est converti en `String` et sauvegardé dans `a`.
- `a` est "move" dans `b`. `a` "meurt".
- `b` est "move" dans `s`. `b` "meurt".
- `s` est "move" dans `c`. `s` "meurt".
- `c` est "move" lors de l'appel à `println!`. `c` "meurt".

Les lignes commentées sont impossibles, car `a` et `b` sont "morts".

Rust va s'assurer que on ne peut pas utiliser `a` ou `b` une fois qu'ils sont morts.

## Move, conclusion

- Marche sur tous les types
- Très haute performance (0 copie)
- Garantie sans erreur.

# Références

- `Move`/`Copy`/`Clone` ne permettent pas de "partager" des données ou coutent une copie (mémoire, cpu).

- Rust propose les "références". Similaire aux références C++, c'est une forme de pointeur.


```
let a = "hello".to_string()
let b = &a;
println!("{}", *b);
```

- `a` contient la `String` "hello"
- `b` est une réference (i.e. "un pointeur") sur `a`.

- `&` retourne une référence.
- `*` retourne la valeur référencée.

# Référence et durée de vie

- Rust s'assure que l'objet cible de la référence vivra plus longtemps que les références, sinon ==> erreur à la compilation.

```
let a = "hello".to_string();
    b = &a;

a.drop();

// Erreur de compilation ici
println!("{}", *b);
```

- `a.drop()` détruit `a`.
- l'usage de `*b` après est impossible ==> erreur à la compilation.

- On peut avoir N références non mutables.

```
let a = "hello".to_string();
let b = &a;
let c = &a;
let d = &a;
```

- Si une reference est  `mut`able, il ne peut en avoir qu'une seule.

```
let mut a = "hello".to_string();
let b = &mut a;
// let c = &mut a; // erreur
```

"borrow"

# Exemple ensemble

- Voir `Copy`, move, `Clone` et les références.

Conseils:

- Type simples: utiliser la copie implicites ou le move.
- Vous avez la flemme: utiliser `Clone`. Cela sera peut-être lent.
- Essayer les références. Le compilateur est assez bon en conseil de syntaxe (ajout des `&` et des `*` quand nécessaire).
