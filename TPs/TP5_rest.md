# REST

Nous allons implémenter un petit serveur qui répond à des requêtes REST. En fin
de TP nous implementerons un petit algorithm pour trouver des acronymes
efficacement, qui vous fera manipuler une structure classique de la
programmation, les `HashMap`.

Pour cela, nous utiliserons https://github.com/http-rs/tide

# Intro

Commencez par créer un projet avec `cargo new`. Suivez le tutoriel sur la page github de `tide`, celui-ci vous amènera à :

- Modifier votre `Cargo.toml` pour y ajouter des dépendances.
- Copier / coller l'exemple dans votre fichier `main.rs`.
- Tester l'exemple (`cargo run`, et `curl`)

# Compréhension

Analyser l'exemple. Que fait-il ? Attardez vous sur:

- les `use`
- les `derive`
- le sens de `async` / `await`
- le `?`

Il est inutile de comprendre en détail ces éléments, mais avoir une idée de ce qu'ils font.

Il y a beaucoup d'exemples dans le répertoire
https://github.com/http-rs/tide/blob/main/examples/ qui vous seront utiles pour
la suite.

# Premiers "endpoints" simples

- Ecrivez un point d'entrée `/fact/compute` qui récupère un entier en entrée (en `POST` json) et retourne la factorielle de cet entier dans la réponse.
- Écrivez un point d'entrée `/fact/compute2/:n` qui récupère l'entier en entrée en `GET`. Vous trouverez un exemple pour l'extraction des arguments dans le fichier `fib.hs` du répertoire d'exemple.

# Point d'entrée plus complexes

- Écriez un point d'entrée `/acronyms/` qui accepte en entrée un JSON du type:

```
["chien", "tortue"]
```

En premier lieu, le type de retour sera un json qui représente une liste de mots associés à chaque mot en entrée:

```
{
   "chien": ["foo", "bar", "baz"],
   "tortue": ["foo", "bar", "baz"],
}
```

Dans les exemples, `json.rs` vous renseignera sur comment renvoyer du json.

# Acronymes

Puis, vous écrirez une fonction qui renvoi les acronymes associés.

Pour cela, vous aurez besoin de:

- une liste de mot (on en trouve en ligne)
- lire cette liste de mot dans un `Vec`
- rechercher les acronymes.

Note: pour savoir si un mot est acronyme d'un autre, il suffit de trier les lettres qui le composent. Le pseudo-code est le suivant:

```
fn est_acronyme(word: String, words: Vec<String>)
{
   for w in words
   {
      if trier_le_mot(word) == trier_le_mot(w)
      {
          w et word sont acronymes
      }
   }
}
```

Pour `trier_le_word`, vous pourrez utiliser `sort` sur un `Vec` et convertir de
`String` vers `Vec` avec respectivement `chars()` et `collect()`.

Faites vous le calcul à chaque requête ? Est-ce long ? Pourquoi ?

# Optimisation acrynomes

On peut optimiser le calcul des acronyme en pré calculant les acronymes d'un
mot.

Pour cela, on va utiliser un structure associative,
https://doc.rust-lang.org/std/collections/struct.HashMap.html, qui associe à
une clé `String` une liste de mots, `HashMap<String, Vec<String>>`.

Par exemple, en anglais, `dog` et `god` sont acronymes, ils ont tout les deux
la même valeur quand leurs lettres sont triés : `dgo`. Notre structure aura
donc une clé `dgo` pointant vers le `Vec` `["dog", "god"]`.
