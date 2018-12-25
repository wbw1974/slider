Slider
======

A "simple" sliding puzzle game. Written in Rust as an exploration on how to build a tiling game engine.

Version 0.5: Minimum Viable Product
-----------------------------------

This tagged version does not use OpenGL, Piston, or any other graphics library. It is an implementation of a slider puzzle, and includes the following features:

1. Only allows a 4x4 puzzle grid.
1. Takes an optional seed value (any usize value) using the `--seed` argument from the command line to always produce the same shuffle pattern.
1. Takes an optional interation value (any positive usize value) using the `--iterations` argument from the command line to determine the number of moves made by the shuffle routine.
1. If the optional seed value is not supplied, the random number generator used to power the shuffle algorithm is supplied from a random system source.
1. If the iteration value is not supplied, the shuffle algorithm defaults to ten shuffle moves.
1. A parser is implemented to take keyboard input to either move a tile (implemented as a number displayed in a grid) or the command `quit` to quit the game.
1. The number of legal moves is tracked and read out at the end of the game (quit or win).
