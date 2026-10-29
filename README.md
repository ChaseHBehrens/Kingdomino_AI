# Kingdomino AI
This project contains several high performance AI bots for the board game Kingdomino.

> ⚠️ **Disclaimer:** This project is an unofficial and independent AI implementation that is not affiliated with, endorsed by, or connected to Blue Orange Games, Bruno Cathala, or any official creators or publishers of Kingdomino.

## Usage

<div align="center">

### Play in your browser

**[Open Kingdomino AI](https://chasehbehrens.github.io/Kingdomino_AI/)**

<a href="https://chasehbehrens.github.io/Kingdomino_AI/"><img width="800" alt="The web version mid-game: the draw pile, the player's kingdom showing where a domino can go, and the bots' kingdoms" src="https://github.com/user-attachments/assets/c5bea50a-10f7-4fa4-b753-484b8a5a3e82" /></a>

### Play in the terminal
Download the **[latest release](https://github.com/ChaseHBehrens/Kingdomino_AI/releases/latest)** for Linux, Windows or macOS

<img width="800" alt="The terminal version mid-game" src="https://github.com/user-attachments/assets/ff7360be-3321-48d2-bee2-7d75b5f7b1b9" />

</div>

## About 
Kingdomino is a game in which you draft dominoes into a 5x5 grid with the aim of creating large continuous areas. 
There are several aspects of this game that make it an interesting problem. 
- A known finite depth of a maximum of 12 turns
- A very high branching factor caused by combinations of placements and the arrangement of the deck
- Unknown information about future drafts
- Multiplayer adversarial rules

This project uses Monte Carlo evaluation search combined with heuristic functions. 
Monte Carlo evaluation has been found to perform well for the game of Kingdomino. 
To enable high performance board states are represented using bit boards. 
This enables a board to be stored in 32 bytes. Possible moves can be generated using simple bit manipulations.  
This allows for the analysis of thousands of positions in milliseconds.

Claude Opus 5.5 was used to generate code for three areas of the project.
- Test scripts for measuring evaluator performance
- Terminal and web user interface
- MCE Heuristic and Claude Heuristic evaluation functions where it made improvements to my existing evaluators

## Results

### Performance
Average time each bot takes per decision, timed on one thread of an Intel Core i5-1145G7.
Every bot was timed on the same positions: every decision of one game per player count, with the Middle Kingdom and Harmony rules.

<div align="center">

| Bot | 4 players | 3 players | 2 players | Slowest decision (4 players) |
|:--|--:|--:|--:|--:|
| Random | 7.4 ns | 7.0 ns | 7.2 ns | 7.9 ns |
| Greedy | 0.5 µs | 0.4 µs | 0.4 µs | 1.6 µs |
| Claude Heuristic | 2.3 µs | 1.8 µs | 3.4 µs | 6.4 µs |
| EOH | 7.8 µs | 6.8 µs | 11.6 µs | 25.4 µs |
| NN Score | 476.4 µs | 384.3 µs | 478.0 µs | 1.5 ms |
| MCE Halving | 17.2 ms | 9.8 ms | 8.4 ms | 87.5 ms |
| MCE Lead | 28.5 ms | 15.8 ms | 13.7 ms | 146.3 ms |
| MCE Score | 28.7 ms | 16.1 ms | 13.1 ms | 151.9 ms |
| MCE Place | 28.8 ms | 16.0 ms | 13.0 ms | 144.0 ms |
| MCE EOH | 30.8 ms | 19.7 ms | 14.7 ms | 94.1 ms |
| MCE Race | 33.9 ms | 46.8 ms | 34.0 ms | 194.6 ms |
| MCE Heuristic | 149.0 ms | 78.0 ms | 79.8 ms | 410.0 ms |
| MCE NN | 1.19 s | 748.8 ms | 550.1 ms | 3.14 s |

</div>

### Ratings

<div align="center">

| Bot | Ordinal | μ | σ |
|:--|--:|--:|--:|
| MCE Heuristic | 29.13 | 32.18 | 1.02 |
| MCE Race | 27.54 | 30.41 | 0.96 |
| MCE Score | 27.20 | 30.07 | 0.96 |
| MCE Halving | 26.45 | 29.33 | 0.96 |
| MCE Place | 26.42 | 29.28 | 0.95 |
| Claude Heuristic | 22.40 | 25.25 | 0.95 |
| MCE Lead | 21.34 | 24.11 | 0.92 |
| MCE EOH | 20.10 | 22.90 | 0.93 |
| EOH | 19.35 | 22.26 | 0.97 |
| Greedy | 18.25 | 21.09 | 0.95 |
| NN Score | 15.94 | 19.11 | 1.06 |
| Random | −8.18 | −1.48 | 2.24 |

</div>

### Head to Head
I tested three bots in a head to head match.

1. **Greedy:** simply chooses the highest scoring option for the given turn
2. **MCE Race:** (The best model I designed) simulates games using a greedy policy with a maximum of 5000 simulations. Moves are no longer considered when there is a less than 5% chance that they are best.
3. **MCE Heuristic:** (The model designed by Claude Opus) simulates games using the Claude heuristic with a maximum of 2000 simulations. Only the 16 moves the heuristic rates highest are considered, and a move is dropped once its average result trails the best by more than 2.5 standard errors. The heuristic adds to the board score a look ahead at next round's pick, the shape of the kingdom, the crowns and squares left in the deck for each landscape, and denying opponents the domino they want most. In the simulations opponents pick moves at random, weighted towards the ones the heuristic rates highest, and in the final round they only block a player when it changes where they finish. Each simulation is scored by the number of opponents beaten plus a small bonus for the margin over the best opponent.

All games were played with the Middle Kingdom and Harmony rules, 100 games per player count.
In the head to head tables, each cell shows how often the row bot outscores the column bot (ties count half), followed by the average margin.

<div align="center">

#### 4 players

| Bot | Score | 1st place | Avg place |
|:--|--:|--:|--:|
| MCE Heuristic | 60.96 ± 0.89 | 47.0% | 1.75 |
| Greedy | 40.61 ± 0.84 | 1.5% | 3.58 |
| MCE Race | 55.25 ± 0.80 | 26.3% | 2.17 |

| | MCE Heuristic | Greedy | MCE Race |
|:--|:-:|:-:|:-:|
| **MCE Heuristic** | — | 95.3% (+20.60 ± 1.07) | 64.8% (+5.54 ± 1.02) |
| **Greedy** | 4.7% (−20.60 ± 1.07) | — | 8.8% (−14.02 ± 0.88) |
| **MCE Race** | 35.2% (−5.54 ± 1.02) | 91.2% (+14.02 ± 0.88) | — |

</div>

<div align="center">

#### 3 players

| Bot | Score | 1st place | Avg place |
|:--|--:|--:|--:|
| MCE Heuristic | 58.04 ± 0.94 | 68.5% | 1.32 |
| Greedy | 35.83 ± 0.87 | 0.0% | 2.93 |
| MCE Race | 49.19 ± 0.95 | 31.5% | 1.75 |

| | MCE Heuristic | Greedy | MCE Race |
|:--|:-:|:-:|:-:|
| **MCE Heuristic** | — | 99.0% (+22.21 ± 1.17) | 68.5% (+8.85 ± 1.31) |
| **Greedy** | 1.0% (−22.21 ± 1.17) | — | 6.0% (−13.36 ± 1.16) |
| **MCE Race** | 31.5% (−8.85 ± 1.31) | 94.0% (+13.36 ± 1.16) | — |

</div>

<div align="center">

#### 2 players

| Bot | Score | 1st place | Avg place |
|:--|--:|--:|--:|
| MCE Heuristic | 57.07 ± 1.23 | 97.8% | 1.02 |
| Greedy | 29.13 ± 0.95 | 0.0% | 2.00 |
| MCE Race | 36.91 ± 0.99 | 52.3% | 1.48 |

| | MCE Heuristic | Greedy | MCE Race |
|:--|:-:|:-:|:-:|
| **MCE Heuristic** | — | 100.0% (+32.03 ± 1.59) | 95.5% (+17.64 ± 1.92) |
| **Greedy** | 0.0% (−32.03 ± 1.59) | — | 0.0% (−9.61 ± 0.94) |
| **MCE Race** | 4.5% (−17.64 ± 1.92) | 100.0% (+9.61 ± 0.94) | — |

</div>

## Improvements
In my implementation placement it decided when choosing a tile for simplicity. 
In the official rules the next set of dominoes is revealed before placement is decided. 
Adding this additional information would probably improve play. Improvements could probably be made to the heuristic functions and search. 
Specifically move generation can sometimes give duplicate positions and positions that are symmetrically equivalent. 
This choice was deliberately made to improve speed of move generation at the expense of some search speed, but it is possible there is a better tradeoff here.
Additionally, I believe reinforcement learning techniques could be used to build better evaluators. 
However, in my implementations self play neural network heuristics performed worse than 
manual heuristics and were too slow to be reliably used inside of MCE search. 

## References
This implementation builds upon research on statistical evaluation and Monte Carlo methods in Kingdomino:

* Gedda, M., Lagerkvist, M. Z., & Butler, M. (2018). *Monte Carlo methods for the game Kingdomino*. arXiv. https://doi.org/10.48550/arxiv.1807.04458

```bibtex
@article{gedda2018kingdomino,
  title     = {Monte Carlo Methods for the Game Kingdomino},
  author    = {Gedda, Magnus and Lagerkvist, Mikael Z. and Butler, Martin},
  journal   = {arXiv preprint arXiv:1807.04458},
  year      = {2018},
  doi       = {10.48550/arxiv.1807.04458}
}
```


