import unittest

from imap_codec import CommandCodec, Encoded


class TestCommandEncode(unittest.TestCase):
    def test_command(self):
        message = {"tag": "a", "body": "Noop"}
        encoded = CommandCodec.encode(message)
        self.assertIsInstance(encoded, Encoded)
        fragments = list(encoded)
        self.assertEqual(fragments, [{"Line": {"data": list(b"a NOOP\r\n")}}])
