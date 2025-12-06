# Advent of Code 2025

<!-- AOC TILES BEGIN --> 
<!-- AOC TILES END -->

---
This setup is based on:
- https://github.com/ChristopherBiscardi/advent-of-code
- https://github.com/LiquidFun/aoc_tiles
---

## Installation Guide

```bash
./.scripts/add-day-project.sh generate --path ./.daily_template --name day_{{day}}
./.scripts/get-aoc-input.sh

pre-commit install --hook-type post-commit
````


## Benchmarking

```bash
cargo bench -q > benchmarks.txt
```

