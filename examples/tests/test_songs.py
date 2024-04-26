from unittest import TestCase
from music.songs import Songs, data


class TestSongs(TestCase):
    songs = Songs()

    def test_songs_without_filter(self):
        r = self.songs.list(None)
        self.assertEqual(
            [song for artist_songs in data.values() for song in artist_songs], r
        )

    def test_songs_with_filter(self):
        r = self.songs.list("taylor swift")
        self.assertEqual(r, data["taylor swift"])
