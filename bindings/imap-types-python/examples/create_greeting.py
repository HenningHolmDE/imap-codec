from imap_types import Code, Greeting, GreetingKind


def print_details(obj):
    print("Type:", type(obj))
    print("Repr:", repr(obj))
    print("Str:", str(obj))
    print("-" * 100)


def main():
    # Code
    # TODO: There has to be a more Pythonic way!
    print_details(Code("Alert"))
    print_details(Code({"BadCharset": {"allowed": []}}))

    # GreetingKind
    print_details(GreetingKind.Ok)

    # Greeting without code
    greeting = Greeting(GreetingKind.Ok, "Hello, world!")
    print_details(greeting)
    greeting = Greeting(GreetingKind.Ok, text="Hello, world!")
    print_details(greeting)
    greeting = Greeting(GreetingKind.Ok, text="Hello, world!", code=None)
    print_details(greeting)

    # Greeting with code
    greeting = Greeting(
        GreetingKind.Ok,
        "Hello, world!",
        Code("Alert"),
    )
    print_details(greeting)
    greeting = Greeting(
        kind=GreetingKind.Ok,
        text="Hello, world!",
        code=Code("Alert"),
    )
    print_details(greeting)


if __name__ == "__main__":
    main()
