# Conway's Game of Life

This is an implementation of [Conway's Game of Life]() in [bevy]().

Not sure what else to put here yet.

### gof-plugin
The project that houses the game itself, agnostic to any engines. Just a library that creates the data structures
that house the game info.

### bevy-app
The bevy implementation that visualizes the game.

### console-app
Project to demonstrate the game of life board. It prints to the console a visualized version of the board step by step
until nothing changes anymore or exceeds 25 iterations (can't go on forever, y'know?).
