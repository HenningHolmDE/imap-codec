import unittest

from imap_codec import CommandCodec


class TestCommandEncode(unittest.TestCase):
    def test_command(self):
        message = {"tag": "a", "body": "Noop"}
        encoded = CommandCodec.encode(message)
        self.assertEqual(encoded, b"a NOOP\r\n")
