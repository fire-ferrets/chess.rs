# Features for the chess engine

 - Data base to store statistics (and maybe more?)
 - API to easily write a chess AI
 - TCP connection to play with others
    - Server - Client architecture with two clients connecting?
 - Custom ASCII skins for the figures

# Architecture

The game logic runs on the server while the client can only read the board
state. The board is then displayed on the TUI and the moves are stored in a
client-side data base. 

```
+-------------------------------+     +-------------------------+
| Server                        |     | Client 2                |
| +-------+        +---------+  |<----| +-----+   +-----------+ |
| |       |------->|         |  |     | |     |   |           | |
| | Board |        | Figures |  |     | | TUI |   | Data base | |
| |       |<-------|         |  |---->| |     |   |           | |
| +-------+        +---------+  |     | +-----+   +-----------+ |
+-------------------------------+     +-------------------------+
           ^   |
           |   v
+-------------------------+
| Client 1                |
| +-----+   +-----------+ |
| |     |   |           | |
| | TUI |   | Data base | |
| |     |   |           | |
| +-----+   +-----------+ |
+-------------------------+
```

The user starts the server on his computer and then connects to it as a client.
The other client spot can be taken by either a TCP connection or a bot which is
also hosted on the user's computer.
This architecture makes it easy to treat each player separately.

# "Classes"
## Server
### Traits
- `UpdateBoard`: Update the board state with a move
- `SetBoard`: Return the current board state
- `SetPossibleMoves`: Return the possible moves for a figure

### Attributes
tbd

## Client
### Traits
- `SendMove`: Send the current move to the server to update the board
- `GetBoard`: Get the new board
- `GetPossibleMoves`: Get the possible moves for a figure

### Attributes
- `host`: Is this client the host of the server

## Board
### Traits
- `Init`:  Initialize a board
- `UpdateBoard`: Update the board state with a move

### Attributes
- `board_state`: The current board

## Figure
### Traits
- `Move`: Return the board with the moved figure
- `PossibleMoves`: Return the possible moves for the figure

### Attributes
- `pos`: Position on the board

## TUI
### Traits
- `ShowBoard`: Print the current board
- `ShowMoves`: Show the possible moves
- `ShowFigureNumber`: Show numbers on the figures to choose the figure to play

### Attributes
- `size`: Size of the TUI. small: Only letters as figures, normal: ASCII art figures

## Data Base
### Traits
- `Init`: Initialize the data base
- `AddMove`: Add a move to the data base

### Attributes
- `data_base`: The file used as data base

# Figure Ideas
## Rook
```
#_#_#
|   |
++-++
 | |
_/_\_
-----
```

```
###
\_/
 I
---
```

## King
```
+++
|_|
 I
---
```

## Queen
```
***
| |
 Y
---
```

## Pawn
```

 O
/ \
---
```

