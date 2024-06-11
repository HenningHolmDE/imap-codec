from imap_types import GreetingCodec


def print_details(obj):
    print("Type:", type(obj))
    print("Repr:", repr(obj))
    print("Str:", str(obj))
    print("-" * 100)


def main():
    codec = GreetingCodec()
    print_details(codec)
    (remaining, greeting) = codec.decode(b"* OK [ALERT] Hello, World!\r\n<remaining>")

    print_details(greeting)
    print(remaining)


if __name__ == "__main__":
    main()
