# Possible Features for the chess engine

 - Data base to store statistics (and maybe more?)
 - API to easily write a chess AI
 - TCP connection to play with others
    - Server - Client architecture with two clients connecting?
 - Custom ASCII skins for the figures

# Possible Architecture

The game logic runs on the server while the client can access the board state
using a `Client.read_board()` and a `Client.write_board()` method. The board is then
displayed on the TUI and the moves are stored in a client-side data base. 

```
+-------------------------------+
| Server                        |
| +-------+        +---------+  |
| |       |------->|         |  |
| | Board |        | Figures |  |
| |       |<-------|         |  |
| +-------+        +---------+  |
+-------------------------------+
    ^   |
    |   v
+-------------------------+
| Client                  |
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

# Figures
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

# Pawn
```

 O
/ \
---
```

