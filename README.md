# Advent of Code 2025

<!-- AOC TILES BEGIN -->
<h1 align="center">
  2025 - 1 ⭐ - Rust
</h1>
<a href=".daily_template/src/bin/part1.rs">
  <img src=".aoc_tiles/tiles/2025/01.png" width="161px">
</a>
<!-- AOC TILES END -->

---
This setup is based on:
- https://github.com/ChristopherBiscardi/advent-of-code
- https://github.com/LiquidFun/aoc_tiles
---

## Installation Guide

```bash
./.scripts/load-aoc-cookie.sh
./.scripts/add-day-project.sh generate --path ./.daily_template --name day_{{day}}
./.scripts/get-aoc-input.sh

pip install pre-commit
pre-commit install --hook-type post-commit
````


## Benchmarking

```bash
cargo bench -q > benchmarks.txt
```

