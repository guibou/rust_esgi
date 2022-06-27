# Intro


Nous allons suivre un TP pour faire une application ligne de commande en rust qui cherche un motif dans un fichier.

Ce sujet est l'occasion de mettre en pratique plusieurs outils intéressants de Rust, de la gestion d'erreur, au deriving automatique de comportement.

# Todo

Suivez les instructions de https://rust-cli.github.io/book/tutorial/index.html

J'ajoute quelques informations complémentaires avec le sujet.

# First step

Il vous sera demandé d'ajouter:

```
[dependencies]
clap = { version = "3.0", features = ["derive"] }
```

Dans votre `Cargo.toml`. Faites cela rapidement, au cas ou votre base de
package ne soit pas à jour, cela peut prendre du temps.

# Structure `Cli`

N'hésitez pas à "tester" vos premiere lignes, `pattern`, `path` et `args` en utilisant `println!("{args:?}")`. Il vous faudra peut-être `#[derive(Debug)]`. Ceci est redondant avec le "Exercice for the reader" à la fin du la première partie.

N'oubliez pas de penser "développement itératif" ;)

*pro-tip*: ne confondez pas `unwrap` et `unwarp`, l'un parle de tacos, l'autre vous fais revenir en arrière après un bond spatio-temporel.

# La fin

Faites les parties du sujets, "Nicer error reporting", elle concerne aussi ce TP et vous fera manipuler des erreurs, ce qui est toujours bien.

Une fois cette partie réalisée, vous pouvez, au choix:

- Lire et faire les parties "Output for humans and machines" et "Testing", elles sont indépendantes, mais vous ferons voir l'usage de quelques *crates* intéressantes, ainsi que le test, ce qui est bien.
- Faire évoluer votre CLI en lui ajoutant de nombreuses fonctionnalités. Par exemple, vous pourriez:

    - Si l'argument de chemin est un répertoire, scanner (récursivement?) ce répertoire à la recherche de fichiers ET afficher les lignes qui fonctionnent par fichier. Vous utiliserez https://doc.rust-lang.org/std/fs/fn.read_dir.html
    - Faire ce traitement en parallel, une très bonne introduction est disponible ici https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html, en utilisant la crate `rayon`.
    - Chercher des motifs plus avancés, en utilisant par exemple des regex https://docs.rs/regex/latest/regex/
    - Comparer vos performances avec l'outil système `grep`.
    - Tester votre code sur de très gros fichiers, quel quantité de RAM consomme il? Sachant que votre code charge le contenu du fichier en RAM intégralement, pouvez vous proposer une solution?
