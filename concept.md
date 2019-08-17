# Features for the chess engine

 - Data base to store statistics (and maybe more?)
 - API to easily write a chess AI
 - TCP connection to play with others
    - Server - Client architecture with two clients connecting?
 - Custom ASCII skins for the pieces

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
### Methods
- `UpdateBoard`: Update the board state with a move
- `SetBoard`: Return the current board state
- `SetPossibleMoves`: Return the possible moves for a piece

### Attributes
tbd

### Connection API
Listens for a "Move" of the player whose turn it currently is. A "Move" could
be defined as a Vector of 2 coordinates, and a coordinate is a Vector `[x, y]`
with `0 <= x, y <= 7`.  
Since sending proper datatypes over a socket is usually unnecessarily
complicated, this will probably be in form of a json stream.  
Then internally tests whether that move is valid, and if so it does the move on
its internal board.  
Finally, sends back a hashmap / dictionary (again, as json) with the success
status as one field, and the current board (as a "matrix" / vector of vectors)
in the other field to the client, so the client now knows the current board and
whether or not it has to perform a different turn again, or now waits for the
opponents turn.  
That json data might look something like this:
```json
{ "status": "valid / invalid",
  "board": [[0, 0, 0, ... ], [...], ...]}
```

## Client
### Methods
- `SendMove`: Send the current move to the server to update the board
- `GetBoard`: Get the new board
- `GetPossibleMoves`: Get the possible moves for a piece

### Attributes
- `host`: Is this client the host of the server

## Board
### Methods
- `init`: Initialize a board
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `NONE`
- `execute_move`: Execute a given move
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `NONE`
- `serialize_board`: Serialize the board for communication
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `ser_board :: JSONArray`

### Attributes
- `board_state :: Array of Array of Piece`: The current board state

## `Piece`
There is one `Piece` class for every chess piece. If there is no piece on a
field, the `EmptyPiece` is used.

### Methods
- `possible_moves`: Return the possible moves for the piece
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `moves :: Vec of Tuples of Tuples`
- `legit_move`: Check if the move is legit
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `legit :: Boolean`
- `to_json`: Return serialized representation
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `ser_piece :: JSONString`

### Attributes
- `color :: String`: Color of the piece
- `board_pointer :: &Board`: Pointer to the current board

## TUI
### Methods
- `ShowBoard`: Print the current board
- `ShowMoves`: Show the possible moves
- `ShowFigureNumber`: Show numbers on the pieces to choose the piece to play

### Attributes
- `size`: Size of the TUI. small: Only letters as pieces, normal: ASCII art pieces

## Data Base
### Methods
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

