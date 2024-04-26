from unittest import TestCase
from music.artists import list, retrieve, data  # Artists


class ArtistTest(TestCase):
    # artists = Artists()

    def test_artist_list(self):
        r = list()
        self.assertEqual(len(data), len(r))
        self.assertEqual(data[0], r[0]["name"])

    def test_artist_retrieve(self):
        r = retrieve(1)
        self.assertEqual(r["name"], data[0])

    def test_artist_retrieve_invalid(self):
        with self.assertRaises(IndexError) as e:
            retrieve(0)
        self.assertEqual(str(e.exception), "id should start from 1")
