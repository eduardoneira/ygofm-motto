#!/usr/bin/env python3
"""Download Yu-Gi-Oh! Forbidden Memories card images politely.

The script uses Fandom's MediaWiki/Semantic MediaWiki APIs instead of scraping
HTML pages. It skips existing files, batches metadata lookups, retries transient
failures, and waits between network requests by default.
"""

from __future__ import annotations

import argparse
import json
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


API_URL = "https://yugioh.fandom.com/api.php"
DEFAULT_OUTPUT_DIR = Path("assets/cards")
DEFAULT_USER_AGENT = "ygofm-motto-card-image-downloader/0.1 (polite MediaWiki API client)"


@dataclass(frozen=True)
class CardImage:
    card_id: int
    card_name: str
    file_name: str
    source_page: str
    image_url: str

    @property
    def output_name(self) -> str:
        return f"{self.card_id:03}.webp"


def main() -> None:
    args = parse_args()
    output_dir = args.output_dir
    output_dir.mkdir(parents=True, exist_ok=True)

    opener = urllib.request.build_opener()
    headers = {"User-Agent": args.user_agent}

    print("Fetching Forbidden Memories card image metadata...")
    card_records = fetch_card_records(opener, headers, args.delay, args.retries)
    if args.limit is not None:
        card_records = card_records[: args.limit]

    card_images = resolve_image_urls(opener, headers, card_records, args.delay, args.retries)
    manifest: dict[str, Any] = {
        "source": "Yu-Gi-Oh! Wiki / Fandom MediaWiki API",
        "api_url": API_URL,
        "downloaded_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "cards": {},
    }

    for index, card_image in enumerate(card_images, start=1):
        destination = output_dir / card_image.output_name
        manifest["cards"][f"{card_image.card_id:03}"] = {
            "name": card_image.card_name,
            "file": destination.name,
            "source_page": card_image.source_page,
            "image_url": card_image.image_url,
            "wiki_file": card_image.file_name,
        }

        if destination.exists() and not args.force:
            print(f"[{index}/{len(card_images)}] exists {destination}")
            continue

        if args.dry_run:
            print(f"[{index}/{len(card_images)}] would download {destination}")
            continue

        print(f"[{index}/{len(card_images)}] downloading {destination}")
        download_file(opener, headers, card_image.image_url, destination, args.retries)
        time.sleep(args.delay)

    if not args.dry_run:
        manifest_path = output_dir / "sources.json"
        manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
        print(f"Wrote {manifest_path}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=DEFAULT_OUTPUT_DIR,
        help=f"directory for downloaded card images, default: {DEFAULT_OUTPUT_DIR}",
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=1.0,
        help="seconds to wait between API/download requests, default: 1.0",
    )
    parser.add_argument(
        "--limit",
        type=int,
        help="download only the first N cards, useful for testing",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="overwrite images that already exist",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="fetch metadata and print planned downloads without writing images",
    )
    parser.add_argument(
        "--retries",
        type=int,
        default=3,
        help="number of retries for each HTTP request, default: 3",
    )
    parser.add_argument(
        "--user-agent",
        default=DEFAULT_USER_AGENT,
        help="HTTP User-Agent sent to Fandom",
    )
    return parser.parse_args()


def fetch_card_records(
    opener: urllib.request.OpenerDirector,
    headers: dict[str, str],
    delay: float,
    retries: int,
) -> list[dict[str, str]]:
    records: list[dict[str, str]] = []
    offset = 0

    while True:
        query = (
            "[[FMR number::+]]"
            "|?Card image"
            "|?FMR number"
            "|?English name"
            "|sort=FMR number"
            "|limit=200"
            f"|offset={offset}"
        )
        response = api_get(
            opener,
            headers,
            retries,
            {
                "action": "ask",
                "query": query,
                "format": "json",
            },
        )

        results = response["query"]["results"]
        for page in results.values():
            printouts = page["printouts"]
            number = first(printouts.get("FMR number"))
            file_name = first(printouts.get("Card image"))
            card_name = first(printouts.get("English name")) or page["fulltext"].replace(" (FMR)", "")

            if not number or not file_name:
                continue

            records.append(
                {
                    "number": str(number),
                    "name": str(card_name),
                    "file_name": str(file_name),
                    "source_page": page["fullurl"],
                }
            )

        next_offset = response.get("query-continue-offset")
        if next_offset is None:
            break

        offset = int(next_offset)
        time.sleep(delay)

    records.sort(key=lambda record: int(record["number"]))
    return records


def resolve_image_urls(
    opener: urllib.request.OpenerDirector,
    headers: dict[str, str],
    records: list[dict[str, str]],
    delay: float,
    retries: int,
) -> list[CardImage]:
    file_names = sorted({record["file_name"] for record in records})
    image_urls_by_file_name: dict[str, str] = {}
    card_images: list[CardImage] = []

    for batch in chunks(file_names, 50):
        response = api_get(
            opener,
            headers,
            retries,
            {
                "action": "query",
                "titles": "|".join(f"File:{file_name}" for file_name in batch),
                "prop": "imageinfo",
                "iiprop": "url",
                "format": "json",
            },
        )

        for page in response["query"]["pages"].values():
            image_info = first(page.get("imageinfo"))
            if not image_info:
                continue

            file_name = page["title"].removeprefix("File:")
            image_urls_by_file_name[file_name] = image_info["url"]

        time.sleep(delay)

    for record in records:
        image_url = image_urls_by_file_name.get(record["file_name"])
        if not image_url:
            continue

        card_images.append(
            CardImage(
                card_id=int(record["number"]),
                card_name=record["name"],
                file_name=record["file_name"],
                source_page=record["source_page"],
                image_url=image_url,
            )
        )

    card_images.sort(key=lambda card_image: card_image.card_id)
    return card_images


def api_get(
    opener: urllib.request.OpenerDirector,
    headers: dict[str, str],
    retries: int,
    params: dict[str, str],
) -> dict[str, Any]:
    url = f"{API_URL}?{urllib.parse.urlencode(params)}"
    for attempt in range(1, retries + 1):
        try:
            request = urllib.request.Request(url, headers=headers)
            with opener.open(request, timeout=30) as response:
                return json.loads(response.read().decode("utf-8"))
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as error:
            if attempt == retries:
                raise

            wait_seconds = 2**attempt
            print(f"  API request failed ({error}); retrying in {wait_seconds}s")
            time.sleep(wait_seconds)

    raise RuntimeError("unreachable API retry state")


def download_file(
    opener: urllib.request.OpenerDirector,
    headers: dict[str, str],
    url: str,
    destination: Path,
    retries: int,
) -> None:
    for attempt in range(1, retries + 1):
        try:
            request = urllib.request.Request(url, headers=headers)
            with opener.open(request, timeout=60) as response:
                destination.write_bytes(response.read())
            return
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as error:
            if attempt == retries:
                raise

            wait_seconds = 2**attempt
            print(f"  request failed ({error}); retrying in {wait_seconds}s")
            time.sleep(wait_seconds)


def first(values: list[Any] | None) -> Any | None:
    if not values:
        return None
    return values[0]


def chunks(values: list[str], size: int) -> list[list[str]]:
    return [values[index : index + size] for index in range(0, len(values), size)]


if __name__ == "__main__":
    main()
