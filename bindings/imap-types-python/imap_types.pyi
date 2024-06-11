from typing import Optional

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
