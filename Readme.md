# Codebreaker

## About

Codebreaker is a game played with quad paper and pencil. Also knowns as bulls & cows, and
popularized as a board game called mastermind. This Command line tool allows you to play the game as
the codebreaker guessing the code and receiving a hint after each guess. For more information you
may want to visit the wikipedia page about mastermind:
<https://en.wikipedia.org/wiki/Mastermind_(board_game)>

## Solver

Next to plainly guessing the code, the user may also enter `s` to generate a guess, based on a
minmax algorithm. Writing this solver is actually the point of this repository. The algorithm is
based on Donald Knuths Algorithm for mastermind. It sprinkles in alpha beta pruning and parallizes
the search using all cores of the machine.