from typing import Final, Optional

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
