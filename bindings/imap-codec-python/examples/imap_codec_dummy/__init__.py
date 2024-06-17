from typing import Tuple


class Command:
    pass


class CommandDecodeError(Exception):
    def __init__(self, kind):
        super().__init__()
        self.kind = kind


class CommandCodec:
    @staticmethod
    def decode(data: bytes) -> Tuple[bytes, Command]:
        print(f"decode({data})")
        raise CommandDecodeError("Incomplete")


class Greeting:
    pass


class GreetingDecodeError(Exception):
    def __init__(self, kind):
        super().__init__()
        self.kind = kind


class GreetingCodec:
    @staticmethod
    def decode(data: bytes) -> Tuple[bytes, Greeting]:
        print(f"decode({data})")
        raise GreetingDecodeError("Incomplete")


class Response:
    pass


class ResponseDecodeError(Exception):
    def __init__(self, kind):
        super().__init__()
        self.kind = kind


class ResponseCodec:
    @staticmethod
    def decode(data: bytes) -> Tuple[bytes, Response]:
        print(f"decode({data})")
        raise ResponseDecodeError("Incomplete")
