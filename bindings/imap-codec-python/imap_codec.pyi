from typing import Tuple

class CommandCodec:
    """
    Codec for commands.
    """

    @staticmethod
    def encode(command: dict) -> Encoded:
        """
        Encode command into fragments.

        :param bytes: Given command
        :return: `Encoded` type holding fragments of encoded command
        """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode command from given bytes.

        :param bytes: Given bytes
        :return: Tuple of remaining bytes and decoded command
        """

class CommandDecodeError(Exception):
    """
    Error during command decoding.
    """

class CommandDecodeFailed(CommandDecodeError):
    """
    "Failed" error during command decoding:
    Decoding failed.
    """

class CommandDecodeIncomplete(CommandDecodeError):
    """
    "Incomplete" error during command decoding:
    More data is needed.
    """

class CommandDecodeLiteralFound(CommandDecodeError):
    """
    "LiteralFound" error during command decoding:
    More data is needed (and further action may be necessary).
    """

class GreetingCodec:
    """
    Codec for greetings.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode greeting from given bytes.

        :param bytes: Given bytes
        :return: Tuple of remaining bytes and decoded greeting
        """

class GreetingDecodeError(Exception):
    """
    Error during greeting decoding.
    """

class GreetingDecodeFailed(GreetingDecodeError):
    """
    "Failed" error during greeting decoding:
    Decoding failed.
    """

class GreetingDecodeIncomplete(GreetingDecodeError):
    """
    "Incomplete" error during greeting decoding:
    More data is needed.
    """

class ResponseCodec:
    """
    Codec for responses.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode response from given bytes.

        :param bytes: Given bytes
        :return: Tuple of remaining bytes and decoded response
        """

class ResponseDecodeError(Exception):
    """
    Error during response decoding.
    """

class ResponseDecodeFailed(ResponseDecodeError):
    """
    "Failed" error during response decoding:
    Decoding failed.
    """

class ResponseDecodeIncomplete(ResponseDecodeError):
    """
    "Incomplete" error during response decoding:
    More data is needed.
    """

class ResponseDecodeLiteralFound(ResponseDecodeError):
    """
    "LiteralFound" error during response decoding:
    The decoder stopped at the beginning of literal data.
    """

class Encoded:
    """
    An encoded message.

    This struct facilitates the implementation of IMAP client- and server implementations by
    yielding the encoding of a message through [`Fragment`]s. This is required, because the usage of
    literals (and some other types) may change the IMAP message flow. Thus, in many cases, it is an
    error to just "dump" a message and send it over the network.
    """

    def __iter__(self) -> "Encoded": ...
    def __next__(self) -> dict: ...
    def dump(self) -> bytes:
        """
        Dump the (remaining) encoded data without being guided by [`Fragment`]s.
        """
