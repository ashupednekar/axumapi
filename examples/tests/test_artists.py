from unittest import TestCase
from music.artists import Artists, data


class ArtistTest(TestCase):
    artists = Artists()

    def test_artist_list(self):
        r = self.artists.list()
        self.assertEqual(len(data), len(r))
        self.assertEqual(data[0], r[0]["name"])
