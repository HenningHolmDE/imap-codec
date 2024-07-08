import unittest

from imap_codec import Encoded, ResponseCodec


class TestResponseEncode(unittest.TestCase):
    def test_simple_response(self):
        message = {"Data": {"Search": [1]}}
        encoded = ResponseCodec.encode(message)
        self.assertIsInstance(encoded, Encoded)
        fragments = list(encoded)
        self.assertEqual(fragments, [{"Line": {"data": list(b"* SEARCH 1\r\n")}}])

    def test_simple_response_dump(self):
        message = {"Data": {"Search": [1]}}
        encoded = ResponseCodec.encode(message)
        self.assertIsInstance(encoded, Encoded)
        self.assertEqual(encoded.dump(), b"* SEARCH 1\r\n")

    _MULTI_FRAGMENT_MESSAGE = {
        "Data": {
            "Fetch": {
                "seq": 12345,
                "items": [
                    {
                        "BodyExt": {
                            "section": None,
                            "origin": None,
                            "data": {
                                "Literal": {
                                    "data": list(b"ABCDE"),
                                    "mode": "NonSync",
                                }
                            },
                        }
                    }
                ],
            }
        },
    }

    def test_multi_fragment_response(self):
        encoded = ResponseCodec.encode(self._MULTI_FRAGMENT_MESSAGE)
        self.assertIsInstance(encoded, Encoded)
        fragments = list(encoded)
        self.assertEqual(
            fragments,
            [
                {"Line": {"data": list(b"* 12345 FETCH (BODY[] {5+}\r\n")}},
                {"Literal": {"data": list(b"ABCDE"), "mode": "NonSync"}},
                {"Line": {"data": list(b")\r\n")}},
            ],
        )

    def test_multi_fragment_response_dump(self):
        encoded = ResponseCodec.encode(self._MULTI_FRAGMENT_MESSAGE)
        self.assertIsInstance(encoded, Encoded)
        self.assertEqual(
            encoded.dump(),
            b"* 12345 FETCH (BODY[] {5+}\r\nABCDE)\r\n",
        )

    def test_multi_fragment_response_dump_remaining(self):
        encoded = ResponseCodec.encode(self._MULTI_FRAGMENT_MESSAGE)
        self.assertIsInstance(encoded, Encoded)
        self.assertEqual(
            next(encoded), {"Line": {"data": list(b"* 12345 FETCH (BODY[] {5+}\r\n")}}
        )
        self.assertEqual(
            encoded.dump(),
            b"ABCDE)\r\n",
        )
