from typing import Optional, List

data = {
    "taylor swift": ["red", "love story", "you belong with me"],
    "ed sheeran": ["photograph", "perfect", "shape of you"],
}


class Songs:

    def list(artist: str | None) -> List[str]:
        if artist:
            return data[artist]
        else:
            songs = []
            for _, a in data.items():
                songs += a
            return songs
