from typing import Optional, List

data = {
    "taylor swift": ["red", "love story", "you belong with me"],
    "ed sheeran": ["photograph", "perfect", "shape of you"],
}


class Songs:

    def list(self, artist: Optional[str]) -> List[str]:
        if artist:
            return data[artist]
        else:
            return [song for artist_songs in data.values() for song in artist_songs]
