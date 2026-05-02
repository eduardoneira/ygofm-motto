# ygofm-motto

Console tools for simulating Yu-Gi-Oh! Forbidden Memories duels.

## Card lookup

Run the first card lookup app with:

```bash
cargo run
```

Then use one of the console commands:

```text
duelists
duelist 1
card 35
```

## Data

Card data is normalized across CSV tables:

- `data/cards.csv`: one row per card with identity, stats, type, guardian stars, password, and starchip cost.
- `data/fusions.csv`: fusion pairs and their result card.
- `data/equips.csv`: equip card compatibility.
- `data/rituals.csv`: ritual recipes and their result card.
- `data/duelists.csv`: opponent identity and hand size.
- `data/duelist_decks.csv`: opponent deck pools as raw weights out of `2048`.
- `data/duelist_drops.csv`: opponent drop pools by rank as raw weights out of `2048`.

Run tests with:

```bash
cargo test
```
