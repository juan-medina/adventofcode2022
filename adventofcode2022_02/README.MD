### Advent of Code 2022

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

#### LICENSE
```
Copyright (c) 2022 Juan Medina

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
