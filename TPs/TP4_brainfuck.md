# Interpreteur brainfuck

Brainfuck est un petit langage qui se prête très bien au jeu de l'implémentation d'un interpréteur.

On fera référence à la liste des commandes: https://en.wikipedia.org/wiki/Brainfuck#Commands

# But du TP

Dans ce TP (sur plusieurs séances), nous réaliserons un interpréteur Brainfuck en plusieurs étapes:

- Création d'un type représentant un nombre limités des instructions brainfuck (`+-<>`).
- Création d'un interpréteur pour une séquence d'éléments de ce type.
- Extension aux opérateurs d'entrée / sortie `.,`.
- Extensions aux boucles `[]`.
- Création d'un parseur, qui convertira une entrée texte (type `"+-+-"`) vers la séquence d'instruction adaptée.
- (Optionel) Optimisations.

# Type de base

Crée un type `enum Instruction` représentant les différentes instructions possibles. Par exemple, le code suivant:

```rust
let code = vec![Instruction::Plus, Instruction::Plus, Instruction::Moins, Instruction::Moins, Instruction::Droite, Instruction::Gauche, Instruction::Affiche];
```

Représente le programme brainfuck suivant: `++--><.`.

Ignorez les instructions de boucle pour l'instant (i.e. `[` et `]`).

# Interpréteur simple

Notre interpréteur travaille sur une mémoire type `Vec<i32>`. Votre fonction d'interprétation aura la signature suivante:

```rust
fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>)
```

Commencez par une fonction qui traite seulement les instructions `+-<>`. On rappel que:

- `+` va incrémenter la case mémoire en cours
- `-` va décrémenter la case mémoire en cours
- `<` décale la case mémoire en cours vers la gauche
- `>` décale la case mémoire en cours vers la droite.

Notes:

- la case mémoire en cours au début est la case 0
- Les instructions non gérés afficheront un petit message d'erreur.

Ainsi, le code suivant:

```rust
fn main()
{
   let mut mem = vec![0, 0, 0];
   let instructions = vec![Plus, Plus, Plus, Droite, Plus, Droite, Moins];
   interpreteur(&mut mem, &instructions);

   println!("{:?}", mem);
}
```

Devrait retourner le contenu du vecteur `mem` comme étant `vec![3, 1, -1]`.

Vous utiliserez surement:

- un `for`, ou un `while`, pour parcourir toutes les instructions.
- un `match`, pour tester chaque instruction.
- une valeur `mut`able, pour stocker la case mémoire en cours.

Que se passes-t-il si on passe une mémoire "vide" au départ, qui ne contient
pas de cases ? Dans ce TP, on supposera que la mémoire est "infinie" a droite
et que chaque case contient un 0 par défaut.

# Entrée / sortie

Les opérations d'entrée sortie sont les suivantes:

- `.` qui affiche sur la console le contenu de la case mémoire courante
- `,` qui lis depuis la console un caractère et stocke celui-ci dans la case mémoire courante.

Vous pourrez utiliser:

- `std::char::from_u32(u as u32).unwrap_or('?')` pour convertir `u` en `char` pour l'affichage.
- `io::stdin.read` pour lire un caractère.

Essayez maintenant:

```rust
let mut mem = vec![72, 101, 108, 108, 111];
let instructions = vec![Affiche, Droite, Affiche, Droite, Affiche, Droite, Affiche, Droite, Affiche];

interpreteur(&mut mem, &instructions);
```

Que fait ce programme (sans l'exécuter). Fait-il bien cela (en l'exécutant).

Et maintenant:

```rust
let mut mem = vec![72, 101, 108, 108, 111];
let instructions = vec![Lis, Plus, Affiche, Lis, Plus, Affiche, Lis, Plus, Lis, Plus, Affiche];

interpreteur(&mut mem, &instructions);
```

? Il vous faudra entrer des caractères au clavier.

# Les boucles

C'est bien tout cela, mais on doit pouvoir faire mieux. Il manque l'instruction de boucle. Je vous recommande d'ajouter un cas dans votre `enum` du type `Boucle(Vec<Instruction>)`.

L'idée est de représenter une "boucle" par un sous vecteur.

La "boucle" représente un While qui s'exécute tant que la condition n'est pas égale à 0.

Je vous laisse discuter ensemble de l'implémentation. Il existe plusieurs solutions. Je recommande de traiter le contenu de la `Boucle` comme un sous-programme, en appelant récursivement `interpreteur`. Mais vous aurez sans doute à passer la position actuelle de la mémoire en argument.

Une fois les boucles implémentée, on peut exécuter des programmes plus conséquents, comme une addition. Par exemple:

```rust
let mut mem = vec![10, 15];
let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];

interpreteur(&mut mem, &instructions);

println!("{:?}", mem);
```

Dans notre cas, considérons `let mut mem = vec![x, y];`. La Boucle va décrémenter la valeur `x` et incrémenter la valeur `y` tant que `x` n'est pas égal à 0.
Devrait afficher `vec![0, 25]`.

# Parsing

Dans TP4_programs, je vous ai mis plusieurs programmes brainfuck interessants:

- `hello.bf` affiche un joli message de bienvenu
- `rot13.bf` va "chiffre" une chaine (saisie au clavier) selon l'encodage rot13
- `hanoi.bf` exécute les tours de hanoi avec une animation graphique. Ce programme est très couteux en ressource et prendra plusieurs minutes à s'exécuter.

Comme il n'est pas question de convertir à la main ces fichiers texte vers les cas de votre `enum`, nous allons écrire une fonction de parsing:

```rust
fn parse(s: &String) -> Result<Vec<Instruction>, String>
```

Commencez par gérer les cas simple, en utilisant une boucle `for` et un `match` sur chaque caractère de votre chaine. Par exemple:

```
let instructions = parse("+-<>.,".to_string())
println("{:?}", instructions);
```

Devrait afficher `vec![Plus, Moins, Gauche, Droite, Affiche, Lis]`.

La gestion des boucles est plus difficile, bonne chance ;)

Question: Pourquoi le type de retour est-il un `Result`?

Une fois votre parseur fonctionnel, vous pourrez utiliser:

```
std::fs::read_to_string("programs/hello.bf".to_string())
```

Afin de lire un programme sur le disque.

# Optimisations

C'est bien, c'est beau, c'est lent. Normalement, le programme `hanoi.bf` doit s'exécuter à un rythme catastrophique. Pourquoi d'après vous?

Vous pouvez compiler en mode "release" avec `cargo` afin d'obtenir un code optimisé, mais cela sera toujours assez lent.

Une des raison est le nombre impressionnant d'instruction brainfuck exécutées. Vous pouvez ajouter un compteur pour vous en rendre compte (i.e. Incrémenter le compteur à chaque instruction effectuée).

D'ailleurs, combien d'instruction / seconde votre interpréteur exécute t il? Cela vous semble beaucoup, peu ? ...

Pour optimiser, nous allons `étendre` le langage brainfuck. En effet, on observe assez souvent des suites représentent de nombreuses fois la même instruction. Au lieu d'exécuter celle-ci N fois, on pourrait sans doute tout regrouper.

Par exemple, `vec![Plus, Plus, Plus, Plus]` pourrait devenir `vec![Plus(4)]`.

Certains motifs reviennent souvent, par exemple `Boucle(Moins)`, qui se contente de mettre à zéro une valeur. On pourrait le remplacer par `Zero`. Pourquoi cela est il plus rapide ?

Afin de faciliter les regroupements, on peut déplacer des instructions, mais il faut faire attention, les instructions sont relatives. Par exemple, le `Plus` incrémente la cellule mémoire courante, ainsi, `vec![Plus, Droite, Plus]` n'est pas équivalent a `vec![Plus, Plus, Droite]`.

Pourrait-on mettre en place une solution pour regrouper plus efficacement les instructions ?

Discutez en ensemble, faites des essais.


