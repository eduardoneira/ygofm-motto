# ygofm-motto

Tools for simulating Yu-Gi-Oh! Forbidden Memories duels.

## Card tracker

Run the card tracker GUI with:

```bash
cargo run
```

The tracked cards are configured in `data/tracked_cards.json`. Create the file with
this shape, or edit the existing one:

```json
{
  "cards": [
    { "id": 1, "target": 3 },
    { "id": 35, "label": "Dark Magician copies", "target": 3 }
  ],
  "groups": [
    {
      "id": "thunders",
      "name": "Thunders",
      "image": "assets/groups/thunders.webp"
    }
  ]
}
```

Use card ids from `data/cards.csv`. `label` is optional; when it is missing, the GUI
uses the card number and card name. `target` is optional and defaults to `3`; it sets
the maximum value for that card's counter.

Groups are manual counters with no maximum value. Each group needs a stable `id` and
display `name`; `image` is optional and points to a local image file, typically under
`assets/groups/`. Group counts reset to `0` every time the GUI starts, matching card
counter behavior.

The GUI reads `data/tracked_cards.json` when it starts, so save the file and restart
`cargo run` after changing the tracked cards. If the file is missing, the app falls
back to the tracked cards bundled at build time.

Each card in the GUI is shown as an art-only image tile with minus, counter, and plus
controls below it. The counter is capped by the card's `target`, and hovering the image
shows the card name or custom label.
If `assets/cards/NNN.webp` exists for a tracked card number, the GUI displays it.
Groups use the same counter controls, but the plus button remains enabled because there
is no target cap.

## Tracker images

Card images are optional local assets. They are not committed to this repository because
the images are third-party Yu-Gi-Oh! game/card assets and may not be redistributable.
The repository only keeps `.gitkeep` placeholders so the destination folders exist.
Custom group images are also local-only and can be placed in `assets/groups/`.

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
- `assets/groups/*`

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
