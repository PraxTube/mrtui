# Mr. TUI (Mempool Rust Terminal UI)

This CLI tool is for displaying some static block information
from the bitcoin blockchain. It uses the
[Mempool API](https://mempool.space/docs/api/rest)
to fetch the data. Example ouput:

```
╭─────806_715──────╮  ╭─────806_714──────╮  ╭─────806_713──────╮  ╭─────806_712──────╮
│   ≈ 19 sat/vB    │  │   ≈ 17 sat/vB    │  │   ≈ 19 sat/vB    │  │   ≈ 26 sat/vB    │
│ 16 - 1508 sat/vB │  │ 16 - 302 sat/vB  │  │ 17 - 302 sat/vB  │  │ 19 - 1213 sat/vB │
│                  │  │                  │  │                  │  │                  │
│     2.00 MB      │  │     1.74 MB      │  │     1.49 MB      │  │     1.61 MB      │
│    3,663 txs     │  │    2,790 txs     │  │    2,589 txs     │  │    3,516 txs     │
│      F2Pool      │  │   Foundry USA    │  │     AntPool      │  │      ViaBTC      │
│                  │  │                  │  │                  │  │                  │
│  20 minutes ago  │  │  30 minutes ago  │  │  39 minutes ago  │  │  54 minutes ago  │
╰──────────────────╯  ╰──────────────────╯  ╰──────────────────╯  ╰──────────────────╯
╭─────806_711──────╮  ╭─────806_710──────╮  ╭─────806_709──────╮  ╭─────806_708──────╮
│   ≈ 15 sat/vB    │  │   ≈ 16 sat/vB    │  │   ≈ 21 sat/vB    │  │   ≈ 18 sat/vB    │
│ 15 - 256 sat/vB  │  │ 15 - 317 sat/vB  │  │ 18 - 697 sat/vB  │  │ 16 - 302 sat/vB  │
│                  │  │                  │  │                  │  │                  │
│     1.86 MB      │  │     1.79 MB      │  │     1.66 MB      │  │     1.52 MB      │
│    4,706 txs     │  │    3,267 txs     │  │    3,141 txs     │  │    2,278 txs     │
│    MARA Pool     │  │     AntPool      │  │      F2Pool      │  │      ViaBTC      │
│                  │  │                  │  │                  │  │                  │
│   1 hours ago    │  │   1 hours ago    │  │   1 hours ago    │  │   2 hours ago    │
╰──────────────────╯  ╰──────────────────╯  ╰──────────────────╯  ╰──────────────────╯

╭───────Halving in: 7 months, 84%────────╮  ╭────Difficulty Adj. in: 11 days, 16%────╮
│█████████████████████████████████       │  │██████                                  │
╰────────────────────────────────────────╯  ╰────────────────────────────────────────╯
```

To install this tool run

```
cargo install mrtui
```

Then you run the command to get current information about the bitcoin blockchain

```
mrtui
```
