### Advent of Code 2022

#### Day 2: Rock Paper Scissors

Language: Rust

[Problem](https://adventofcode.com/2022/day/2)

#### usage 

```bash
> cargo run
```

#### output

```
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must draw - Player: Paper, Draw! Match Points: 5
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must loose - Player: Paper, Player loose! Match Points: 2
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must loose - Player: Scissor, Player loose! Match Points: 3
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Scissor - Strategy: Player must win - Player: Rock, Player wins! Match Points: 7
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must win - Player: Scissor, Player wins! Match Points: 9
Adversary: Rock - Strategy: Player must draw - Player: Rock, Draw! Match Points: 4
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Rock - Strategy: Player must win - Player: Paper, Player wins! Match Points: 8
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Scissor - Strategy: Player must draw - Player: Scissor, Draw! Match Points: 6
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Adversary: Paper - Strategy: Player must loose - Player: Rock, Player loose! Match Points: 1
Total Points: 10560
```