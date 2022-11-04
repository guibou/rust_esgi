# Rust 2022-2023

- guillaum.bouchard@gmail.com

# What about me?

- ask me anything.

# Plan du cours

## Fonctionnement du cours

- Très peu de bla bla / quelques slides
- Mode "projet"
- Interagissez !

## Séances

27h

## Notation

- A discuter ensemble.

## Planning (prévisionnel) des séances

- Intro / syntaxe / prise en main
- Interpreteur brainfuck 
- Serveur web (1h30)
- Command line CLI
- Jeu vidéo
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

# Cargo

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
