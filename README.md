# ygofm-motto

Tools for simulating Yu-Gi-Oh! Forbidden Memories duels.

## Card tracker

Run the card tracker GUI with:

```bash
cargo run
```

The tracked cards are configured in `data/tracked_cards.json`:

```json
{
  "cards": [
    { "id": 1, "target": 3 },
    { "id": 35, "label": "Dark Magician copies", "target": 3 }
  ]
}
```

Each row in the GUI has a button to add one copy and a button to remove one copy.
If `assets/cards/NNN.webp` exists for a tracked card number, the GUI displays it in that row.

## Card images

Card images are optional local assets. They are not committed to this repository because
the images are third-party Yu-Gi-Oh! game/card assets and may not be redistributable.
The repository only keeps `assets/cards/.gitkeep` so the destination folder exists.

Download or refresh card images with:

```bash
python3 scripts/download_card_images.py
```

The downloader uses the Yu-Gi-Oh! Wiki / Fandom MediaWiki API, skips existing files,
waits one second between requests by default, and writes source metadata to
`assets/cards/sources.json`. Use `--dry-run --limit 3` to test without downloading.

The generated files are intentionally ignored by Git:

- `assets/cards/*.webp`
- `assets/cards/sources.json`

If you clone the project on another machine, run the downloader again to populate
the local image cache.

## Card lookup

Run the console lookup app with:

```bash
cargo run -- --cli
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
