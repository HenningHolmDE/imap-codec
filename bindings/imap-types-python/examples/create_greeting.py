import imap_types


def print_details(greeting):
    print(type(greeting))
    print(repr(greeting))
    print(str(greeting))


def main():
    # Greeting without code
    greeting = imap_types.Greeting("Ok", "Hello, world!")
    print_details(greeting)
    greeting = imap_types.Greeting(kind="Ok", text="Hello, world!")
    print_details(greeting)
    greeting = imap_types.Greeting(kind="Ok", text="Hello, world!", code=None)
    print_details(greeting)

    # Greeting with code
    greeting = imap_types.Greeting(
        "Ok",
        "Hello, world!",
        "Alert",
    )
    print_details(greeting)
    greeting = imap_types.Greeting(
        kind="Ok",
        text="Hello, world!",
        code="Alert",
    )
    print_details(greeting)


if __name__ == "__main__":
    main()
