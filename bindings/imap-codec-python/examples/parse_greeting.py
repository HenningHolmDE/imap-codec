from common import Role, read_more
from imap_codec_dummy import GreetingCodec, GreetingDecodeError

WELCOME = r"""# Parsing of IMAP greetings

"S:" denotes the server.

Note: "\n" will be automatically replaced by "\r\n".

--------------------------------------------------------------------------------------------------

Enter IMAP greeting (or "exit").
"""


def main():
    print(WELCOME)

    buffer = bytearray()

    while True:
        # Try to parse the first greeting in `buffer`.
        try:
            remaining, greeting = GreetingCodec.decode(buffer)
            # Parser succeeded.
            # Do something with the greeting ...
            print(greeting)
            # ... and proceed with the remaining data.
            buffer = remaining
        except GreetingDecodeError as e:
            if e.kind == "Incomplete":
                # Parser needs more data.
                # Read more data.
                read_more(buffer, Role.Server)
            else:
                # Parser failed.
                print("Error parsing greeting.")
                print("Clearing buffer.")

                # Clear the buffer and proceed with loop.
                buffer.clear()


if __name__ == "__main__":
    main()
