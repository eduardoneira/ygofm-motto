# ygofm-motto

Console tools for simulating Yu-Gi-Oh! Forbidden Memories duels.

## Card lookup

Run the first card lookup app with:

```bash
cargo run
```

Then enter a Forbidden Memories card number from `1` to `722`.

## Data

Card data is normalized across CSV tables:

- `data/cards.csv`: one row per card with identity, stats, type, guardian stars, password, and starchip cost.
- `data/fusions.csv`: fusion pairs and their result card.
- `data/equips.csv`: equip card compatibility.
- `data/rituals.csv`: ritual recipes and their result card.

Run tests with:

```bash
cargo test
```
