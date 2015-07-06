# Tic-Tac-Toe
Tic-Tac-Toe AI using a variation of Q-learning

## The Program/How to run
This is a program for playing Tic-Tac-Toe. When it is compiled and run, it will ask you to choose an opponent (1 for 2nd player and 2 for AI). Enter the number corresponding to the opponent you want to face, and press enter. After this, you will be show a Tic-Tac-Toe board and asked where to move. Enter the number corresponding to where you want to move and hit enter. The board is numbered as follows:

0 | 1 | 2
--- | --- | ---
3 | 4 | 5
6 | 7 | 8
 
When the game is over, it will ask you to play again. Enter y for yes and n for no.

## The AI
The AI is trained at the start of the program using a variation of [Q-learning](https://en.wikipedia.org/wiki/Q-learning). The change made for the AI is that the estimate of optimal future value takes into account whose turn it is when training. The AI is O's so when updating the Q value for a board-move (state-action) pair, if it is X's turn for the next board, the estimate of optimal future value is the minimum Q value for the next board over all actions (the X player will want to minimize the O player's chances of winning) instead of the maximum. This change was made so that the AI would take into account the fact that it is competing against another player.
