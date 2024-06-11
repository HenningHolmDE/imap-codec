from imap_types import Greeting, GreetingKind


def print_details(greeting):
    print(type(greeting))
    print(repr(greeting))
    print(str(greeting))


def main():
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
        "Alert",
    )
    print_details(greeting)
    greeting = Greeting(
        kind=GreetingKind.Ok,
        text="Hello, world!",
        code="Alert",
    )
    print_details(greeting)


if __name__ == "__main__":
    main()
