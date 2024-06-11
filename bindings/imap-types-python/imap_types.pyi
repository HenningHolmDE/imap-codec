from typing import Final, Optional, Tuple

class Code:
    """
    A response code consists of data inside square brackets in the form of an atom,
    possibly followed by a space and arguments.  The response code
    contains additional information or status codes for client software
    beyond the OK/NO/BAD condition, and are defined when there is a
    specific action that a client can take based upon the additional
    information.

    :param code: Object to be deserialized into Code variant.
    """

    def __init__(self, code: object) -> None: ...

class Greeting:
    """
    Greeting.

    Note: Don't use `code: None` *and* a `text` that starts with "[" as this would be ambiguous in IMAP.
    We could fix this but the fix would make this type unconformable to use.

    :param kind: Greeting kind
    :param text: Greeting text
    :param code: (optional) Greeting code
    """

    def __init__(self, kind: str, text: str, code: Optional[str]) -> None: ...

class GreetingKind:
    """
    IMAP4rev1 defines three possible greetings at connection startup.
    """

    Ok: Final["GreetingKind"]
    """
    The connection is not yet authenticated.
    (Advice: A LOGIN command is needed.)
    """

    PreAuth: Final["GreetingKind"]
    """
    The connection has already been authenticated by external means.

    (Advice: No LOGIN command is needed.)
    """

    Bye: Final["GreetingKind"]
    """
    The server is not willing to accept a connection from this client.

    (Advice: The server closes the connection immediately.)
    """

class GreetingCodec:
    """
    Codec for greetings.
    """

    def decode(self, bytes: bytes) -> Tuple[bytes, Greeting]:
        """
        Decode Greeting from given bytes.

        :param bytes: Given bytes
        :return: Tuple of remaining bytes and decoded greeting
        """
