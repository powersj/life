# life

*** Basic GUI to demo the Game of Life with a variety of rule sets ***

## Overview

John Conway in the 1970s introduced the [Game of Life][]. The game, a cellular
automaton, consistes of a grid, made up of cells generally with two states:
alive or dead. At each turn or generation, the entire grid is evaluated
cell-by-cell, based on a set of rules. The rules determine wheather cells, are
born, continue to live, or die. The rules are applied to create a new
grid for the next generation of cells.

The rules that are used can lead to amazing patterns and sequences. Some of
these patterns can look like ships, usually referred ot as a gliders, that move
across the grid.

The goal of this project was to introduce myself to more Rust and learn how to
layout code, and continue to play with [egui][].

[Game of Life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
[egui]: https://github.com/emilk/egui/

## Controls

Users can control a variety of aspects of the game:

* `rule set`: Pick which rules to apply at each generation. More details on each
  of these below.
* `seed`: At the start, a grid with a random percentage of live seeds is
  created. Users can set a seed to rewatch a certain seed at any time. Users can
  also select a random seed to mix things up further.
* `alive cell %`: Users can also set the percentage of living cells when
  generating a new board to see how that affects certain rule sets.

## Rule sets

A [rule string][] describes when cells should be born or stay alive, otherwise
the cell dies. Rule sets are in the format `B{number list}/S{number list}`. For
example, the classic Conway's Game of Life rule set is `B3/S23` meaning:

* If an empty cell has 3 neighbors, then it is born
* If a living sell has 2 or 3 neightbors, then it dies

Includes are a number of different rules.

| Name               | Rule String     | Description                          |
|--------------------|-----------------|--------------------------------------|
| Conways            | `B3/S23`        | The classic rule set |
| Assimilation       | `B345/S4567`    | |
| Day and night      | `B3678/S34678`  | |
| Diamoeba           | `B35678/S5678`  | |
| Dot life           | `B3/S023`       | |
| Dry life           | `B37/S23`       | |
| High life          | `B36/S23`       | |
| Honey life         | `B38/S238`      | |
| Invert a Maze      | `B028/S0124`    | |
| Life without death | `B3/S012345678` | |
| Vote               | `B5678/S45678`  | |

A neighbor is any cell that touches another cell. This incudes all 8 cells
around another cell. Cells on the edge do not wrap around to the other side.
Sometimes a rule string with a `v` at the end indicates a von Neumann
neighborhood, meaning only the four cells in the north, south, east, and west
direction.

[rule string]: https://conwaylife.com/wiki/Rulestring

## TODO

* Change rule sets to return a single 1 or 0 and evaluate a cell at a time
  to reduce code duplication
* Introduce a custom rule option to let the user set their own rules
* Can the user draw a custom grid
