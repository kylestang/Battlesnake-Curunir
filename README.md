# Curunir

A [Battlesnake](https://play.battlesnake.com) server written in Rust.

Curunir is stateless, and uses several algorithms to determine which move it will
make next. Each provides different types of data, from guaranteed outcomes to
strategic positioning. The snake also has a full test suite of >50 unit and integration
tests, as well as several benchmarks.

## Turn Prediction
It uses a Heavily modified minimax algorithm to predict future turns.
The algorithm simulates every possible future move and asseses each outcome
for every snake. It then assumes each snake is playing optimally and picks
the corresponding outcome.

This approach is effective but computationally expensive. The complexity is
3^(number of turns * number of players). The current algorithm can run with an
exponent of 13, becoming more effective as other players are eliminated.

## Route finding
A depth-first longest path algorithm finds routes for the snake to escape to.
Escape is defined as any area long enough to fit the entire snake, or a tail
for the snake to chase indefinitely. This algorithm is NP but can be implemented
efficiently enough to find all possible paths for the next 30 moves without difficulty.

## Areas of control
To provide a high-level advantage, the snake tries to maximise the area of the
board it controls. Control is defined as the number of tiles it can reach before
any other snake. This promotes moving toward the center of the board and cutting
off competitors.

Area of control is calculated using a floodfill from each snake head simultaneously
until all tiles on the board have been claimed.
