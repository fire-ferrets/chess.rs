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

# Classes
## General structures

 - `move`: A tuple of tuples

## `Server`
### Methods
- `update_board`: Update the board state with a move
- `set_board`: Return the current board state
- `possible_moves`: Return the possible moves for a piece
- `check_data`: Check if the data is valid
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `valid_code :: i8`

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

### Codes

There may be different reasons for the invalidity of the move which should be
communicated to the client. We can account for this by using integer codes which
can generate either an error message or can be used by an automated client as
information about the move.

#### Error codes

| Code | Method                  | Meaning                             |
| :-   | :-                      | :-                                  |
| -1   | `Piece.valid_move`      | No valid move for the piece         |
| -2   | `Board.check_positions` | No figure at source position        |
| -3   | `Board.check_positions` | Opponent's piece at source position |
| -4   | `Board.check_positions` | Own figure at end position          |
| -5   | `Board.check_positions` | Source position outside the board   |
| -6   | `Board.check_positions` | End position outside the board      |
| -7   | `Piece.check_way`       | Other piece blocks the move         |
| -8   | `Board.check_check`     | Move results in check               |
| -9   | `Server.check_data`     | Invalid data                        |

#### Non-error codes

| Code | Method                  | Meaning        |
| :-   | :-                      | :-             |
| 0    | `Piece.valid_move`      | Valid move     |
| 1    | `Board.check_check`     | Check          |
| 2    | `Board.check_check`     | Checkmate      |
| 3    | `Board.check_transform` | Transform pawn |

## `Board`
### Methods
- `init`: Initialize a board
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `NONE`
- `check_positions`: Check the positions in the move for their spatial validity
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `valid_code :: i8`
- `check_check`: Check if the move results in a check
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `valid_code :: i8`
- `check_transform`: Check if a pawn has reached the transformation line
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `valid_code :: i8`
- `execute_move`: Execute a given move
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `NONE`
- `repr`: Return a string representation of the board. Can be used for the serialization.
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `str_board :: String`

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
- `valid_move`: Check if the move is legit
    - **Parameters:**
        - `move :: Tuple of Tuples`
    - **Return:**
        - `valid_code :: i8`
- `repr`: Return a string representation of the piece. Can be used for the serialization.
    - **Parameters:**
        - `NONE`
    - **Return:**
        - `str_piece :: String`

### Attributes
- `color :: String`: Color of the piece
- `board_pointer :: &Board`: Pointer to the current board

## `Client`
### Methods
- `send_move`: Send the current move to the server to update the board
- `get_board`: Get the new board
- `get_possible_moves`: Get the possible moves for a piece

### Attributes
- `host`: Is this client the host of the server


## `TUI`
### Methods
- `ShowBoard`: Print the current board
- `ShowMoves`: Show the possible moves
- `ShowFigureNumber`: Show numbers on the pieces to choose the piece to play

### Attributes
- `size`: Size of the TUI. small: Only letters as pieces, normal: ASCII art pieces

## `Data Base`
### Methods
- `Init`: Initialize the data base
- `AddMove`: Add a move to the data base

### Attributes
- `data_base`: The file used as data base

## `Repr` Trait

Implements:
 - `repr :: None -> String`: Return a string representation.

# Figure Ideas

It may be nice to have some kind of ASCII representation for the figure. But
this might require a lot of space for an `8x8` board. Thus, it would be nice to
have the option to switch between a small and normal mode. The small mode would
then just display the piece code as a single character and the normal mode
would show the ASCII pieces.

## Rook
```
#_#_#
|   |      ###
++-++  or  \_/
 | |        I
_/_\_      ---
-----
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

