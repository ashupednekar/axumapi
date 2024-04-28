from typing import Optional, List, Dict

data = {
    "taylor swift": ["red", "love story", "you belong with me"],
    "ed sheeran": ["photograph", "perfect", "shape of you"],
}


# class Songs:
#
#    def list(self, artist: str | None = None) -> List[str]:
#        if artist:
#            return data[artist]
#        else:
#            return [song for artist_songs in data.values() for song in artist_songs]


def list(*request) -> List[str]:
    headers, params, body = request
    print(f"headers: {headers}")
    print(f"paramas: {params}")
    print(f"body: {body}")
    artist = params.get("artist")
    if artist:
        r = data[artist]
    else:
        r = [song for artist_songs in data.values() for song in artist_songs]
    print(f"songs: {r}")
    return r
