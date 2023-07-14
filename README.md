# BetaFish
Chess engine using 3 languages somehow implementing UCI.

* C - A simple executable that runs the engine when called inside this directory
* Python - (Partially) implements UCI, but is minimally involved with computing the best move
* Rust - Used to compute the best move

Currently, the engine uses a rather bad heuristic of just taking a weighted count of
the pieces on the board, and searches to a fixed depth of 4. 
The search itself is totally unoptimized and runs so painfully slowly on higher depths.

# Build

To build:
```commandline
gcc run.c -o run
cargo build --release
```

# Run

To run with executable:
```commandline
cd /Path/To/BetaFish/
./run
```

To run with Python:
```commandline
python3 engine.py
```

# Todo / Coming Soon
- Memoization table
- Alpha Beta Pruning
- Better Heuristics
