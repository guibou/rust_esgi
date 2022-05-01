# Rust

- guillaum.bouchard@gmail.com

# What about me?

- ask me anything.

# Plan du cours

## Fonctionnement du cours

- Très peu de bla bla / quelques slides
- Mode "projet"
- Interagissez !

## Seances

29h

- 2 mai - 8:00 -> 15:30
- 3 mai - 8:00 -> 13:00
- 4 mai - 8:00 -> 13:00
- 5 mai - 9:45 -> 13:00
- 6 mai - 9:45 -> 13:00 (remote)
- 27 juin - 11:30 -> 13:00
- 28 juin - 11:30 -> 13:00
- 1 juillet - 9:45 -> 13:00

## Notation

- A discuter ensemble.

## Planning (prévisionnel) des séances

- Intro / syntaxe / prise en main
- Interpreteur brainfuck 
- Serveur web (1h30)
- web ? (3h)
- Jeu vidéo (6h)
- You name it...

# Rust intro

# Pourquoi rust?

- Java/Python/C#

- C++/C

- Langage système
- Memory/Everything safe
- Belles abstractions gratuites
- (Pour l'instant) peu d'historique
- Performances
- Web
- Hype
- Adapté aux environment contraints

# Ressources

- https://github.com/guibou/rust_esgi/
- https://doc.rust-lang.org/

# Cargo (devoir maison)

- https://www.rust-lang.org/tools/install

-> débrouillez vous, mais il faut pouvoir lancer un `cargo build` d'une manière ou d'une autre.

-> Installez un LSP!


# Syntax

Long démo qui présentera les choses suivantes:

- Types de base `i32`
- Fonctions factorielle
- `if`, `for`, `while`, `mut`, `fold`
- Types (struct/enum)
- Match

- `impl` et `trait`
- Deriving
- Macro
- Durée de vie
- Gestion d'erreur / Option / Error
- Lambda

# Elements de surprises

- pas de 'return', les instructions sont renvoyées directemnt
- `mut`, `clone`, `copy` -> Laissez vous aider par le LSP / Compilateur. Dans le doute, `.clone()`.
