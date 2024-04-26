from unittest import TestCase
from music.songs import list, data  # Songs


class TestSongs(TestCase):
    # songs = Songs()

    def test_songs_without_filter(self):
        r = list()
        self.assertEqual(
            [song for artist_songs in data.values() for song in artist_songs], r
        )

    def test_songs_with_filter(self):
        r = list("taylor swift")
        self.assertEqual(r, data["taylor swift"])
