[![Build & Test](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml)
[![Audit](https://github.com/duesee/imap-codec/actions/workflows/audit.yml/badge.svg)](https://github.com/duesee/imap-codec/actions/workflows/audit.yml)
[![Coverage](https://coveralls.io/repos/github/duesee/imap-codec/badge.svg?branch=main)](https://coveralls.io/github/duesee/imap-codec?branch=main)
[![Documentation](https://docs.rs/imap-codec/badge.svg)](https://docs.rs/imap-codec)

# imap-codec

This library provides parsing, serialization, and support for [IMAP4rev1] implementations.
It is based on [imap-types] and aims to become a rock-solid building block for IMAP client and server implementations in Rust.
The complete [formal syntax] of IMAP4rev1 and several IMAP extensions are implemented.
Please see the [documentation] for more information.

## Features

* Rust's type system is used to enforce correctness and make the library misuse-resistant.
It should not be possible to construct messages that violate the IMAP specification.
* Every parser works in streaming mode, i.e., all parsers will return `Incomplete` when there is insufficient data to make a final decision. 
No command or response will ever be truncated.
* Parsing is zero-copy by default but all types can easily be converted into owned variants when needed.
* Fuzzing (via [cargo fuzz]) and property-based tests are used to uncover parsing and serialization bugs.
For example, the library is fuzz-tested never to produce a message it can not parse itself.

## Usage

```rust
use imap_codec::{
    codec::{Decode, Encode},
    command::Command,
};

fn main() {
    let input = b"ABCD UID FETCH 1,2:* (BODY.PEEK[1.2.3.4.MIME]<42.1337>)\r\n";

    let (remainder, parsed) = Command::decode(input).unwrap();
    println!("# Parsed\n\n{:#?}\n\n", parsed);

    let buffer = parsed.encode_detached().unwrap();

    // Note: IMAP4rev1 may produce messages that are not valid UTF-8.
    println!("# Serialized\n\n{:?}", std::str::from_utf8(&buffer));
}
```

## Examples

### Simple parsing

Try one of the `parse_*` examples, e.g., ...

```sh
$ cargo run --example=parse_command
```

... to parse some IMAP messages.

### Tokio demo

You can also start the [demo server] with ...

```sh
$ cd assets/demos/tokio_server
$ cargo run
```

... and connect to it with ...

```sh
$ netcat -C 127.0.0.1 14300
```

There is also a [demo client] available.

**Note:** All demos are a work-in-progress. Feel free to propose API changes to imap-codec (or imap-types) to simplify them.

### Parsed and serialized IMAP4rev1 connection

The following output was generated by reading the trace from [RFC 3501 section 8](https://tools.ietf.org/html/rfc3501#section-8), printing the input (first line), `Debug`-printing the parsed object (second line), and printing the serialized output (third line).

```rust
// * OK IMAP4rev1 Service Ready
Status(Ok { tag: None, code: None, text: Text("IMAP4rev1 Service Ready") })
// * OK IMAP4rev1 Service Ready

// a001 login mrc secret
Command { tag: Tag("a001"), body: Login { username: Atom(AtomExt("mrc")), password: /* REDACTED */ } }
// a001 LOGIN mrc secret

// a001 OK LOGIN completed
Status(Ok { tag: Some(Tag("a001")), code: None, text: Text("LOGIN completed") })
// a001 OK LOGIN completed

// a002 select inbox
Command { tag: Tag("a002"), body: Select { mailbox: Inbox } }
// a002 SELECT INBOX

// * 18 EXISTS
Data(Exists(18))
// * 18 EXISTS

// * FLAGS (\Answered \Flagged \Deleted \Seen \Draft)
Data(Flags([Answered, Flagged, Deleted, Seen, Draft]))
// * FLAGS (\Answered \Flagged \Deleted \Seen \Draft)

// * 2 RECENT
Data(Recent(2))
// * 2 RECENT

// * OK [UNSEEN 17] Message 17 is the first unseen message
Status(Ok { tag: None, code: Some(Unseen(17)), text: Text("Message 17 is the first unseen message") })
// * OK [UNSEEN 17] Message 17 is the first unseen message

// * OK [UIDVALIDITY 3857529045] UIDs valid
Status(Ok { tag: None, code: Some(UidValidity(3857529045)), text: Text("UIDs valid") })
// * OK [UIDVALIDITY 3857529045] UIDs valid

// a002 OK [READ-WRITE] SELECT completed
Status(Ok { tag: Some(Tag("a002")), code: Some(ReadWrite), text: Text("SELECT completed") })
// a002 OK [READ-WRITE] SELECT completed

// a003 fetch 12 full
Command { tag: Tag("a003"), body: Fetch { sequence_set: SequenceSet(NonEmptyVec([Single(Value(12))])), attributes: Macro(Full), uid: false } }
// a003 FETCH 12 FULL

// * 12 FETCH (FLAGS (\Seen) INTERNALDATE "17-Jul-1996 02:44:25 -0700" RFC822.SIZE 4286 ENVELOPE ("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)" "IMAP4rev1 WG mtg summary and minutes" (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) ((NIL NIL "imap" "cac.washington.edu")) ((NIL NIL "minutes" "CNRI.Reston.VA.US")("John Klensin" NIL "KLENSIN" "MIT.EDU")) NIL NIL "<B27397-0100000@cac.washington.edu>") BODY ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 3028 92))
Data(Fetch { seq_or_uid: 12, attributes: NonEmptyVec([Flags([Flag(Seen)]), InternalDate(1996-07-17T02:44:25-07:00), Rfc822Size(4286), Envelope(Envelope { date: NString(Some(Quoted(Quoted("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)")))), subject: NString(Some(Quoted(Quoted("IMAP4rev1 WG mtg summary and minutes")))), from: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], sender: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], reply_to: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], to: [Address { name: NString(None), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("imap")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], cc: [Address { name: NString(None), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("minutes")))), host: NString(Some(Quoted(Quoted("CNRI.Reston.VA.US")))) }, Address { name: NString(Some(Quoted(Quoted("John Klensin")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("KLENSIN")))), host: NString(Some(Quoted(Quoted("MIT.EDU")))) }], bcc: [], in_reply_to: NString(None), message_id: NString(Some(Quoted(Quoted("<B27397-0100000@cac.washington.edu>")))) }), Body(Single { body: Body { basic: BasicFields { parameter_list: [(Quoted(Quoted("CHARSET")), Quoted(Quoted("US-ASCII")))], id: NString(None), description: NString(None), content_transfer_encoding: Quoted(Quoted("7BIT")), size: 3028 }, specific: Text { subtype: Quoted(Quoted("PLAIN")), number_of_lines: 92 } }, extension: None })]) })
// * 12 FETCH (FLAGS (\Seen) INTERNALDATE "17-Jul-1996 02:44:25 -0700" RFC822.SIZE 4286 ENVELOPE ("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)" "IMAP4rev1 WG mtg summary and minutes" (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) ((NIL NIL "imap" "cac.washington.edu")) ((NIL NIL "minutes" "CNRI.Reston.VA.US")("John Klensin" NIL "KLENSIN" "MIT.EDU")) NIL NIL "<B27397-0100000@cac.washington.edu>") BODY ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 3028 92))

// a003 OK FETCH completed
Status(Ok { tag: Some(Tag("a003")), code: None, text: Text("FETCH completed") })
// a003 OK FETCH completed

// a004 fetch 12 body[header]
Command { tag: Tag("a004"), body: Fetch { sequence_set: SequenceSet(NonEmptyVec([Single(Value(12))])), attributes: FetchAttributes([BodyExt { section: Some(Header(None)), partial: None, peek: false }]), uid: false } }
// a004 FETCH 12 BODY[HEADER]

// * 12 FETCH (BODY[HEADER] {342}
// Date: Wed, 17 Jul 1996 02:23:25 -0700 (PDT)
// From: Terry Gray <gray@cac.washington.edu>
// Subject: IMAP4rev1 WG mtg summary and minutes
// To: imap@cac.washington.edu
// cc: minutes@CNRI.Reston.VA.US, John Klensin <KLENSIN@MIT.EDU>
// Message-Id: <B27397-0100000@cac.washington.edu>
// MIME-Version: 1.0
// Content-Type: TEXT/PLAIN; CHARSET=US-ASCII
// 
// )
Data(Fetch { seq_or_uid: 12, attributes: NonEmptyVec([BodyExt { section: Some(Header(None)), origin: None, data: NString(Some(Literal(Literal { data: [68, 97, 116, 101, 58, 32, 87, 101, 100, 44, 32, 49, 55, 32, 74, 117, 108, 32, 49, 57, 57, 54, 32, 48, 50, 58, 50, 51, 58, 50, 53, 32, 45, 48, 55, 48, 48, 32, 40, 80, 68, 84, 41, 13, 10, 70, 114, 111, 109, 58, 32, 84, 101, 114, 114, 121, 32, 71, 114, 97, 121, 32, 60, 103, 114, 97, 121, 64, 99, 97, 99, 46, 119, 97, 115, 104, 105, 110, 103, 116, 111, 110, 46, 101, 100, 117, 62, 13, 10, 83, 117, 98, 106, 101, 99, 116, 58, 32, 73, 77, 65, 80, 52, 114, 101, 118, 49, 32, 87, 71, 32, 109, 116, 103, 32, 115, 117, 109, 109, 97, 114, 121, 32, 97, 110, 100, 32, 109, 105, 110, 117, 116, 101, 115, 13, 10, 84, 111, 58, 32, 105, 109, 97, 112, 64, 99, 97, 99, 46, 119, 97, 115, 104, 105, 110, 103, 116, 111, 110, 46, 101, 100, 117, 13, 10, 99, 99, 58, 32, 109, 105, 110, 117, 116, 101, 115, 64, 67, 78, 82, 73, 46, 82, 101, 115, 116, 111, 110, 46, 86, 65, 46, 85, 83, 44, 32, 74, 111, 104, 110, 32, 75, 108, 101, 110, 115, 105, 110, 32, 60, 75, 76, 69, 78, 83, 73, 78, 64, 77, 73, 84, 46, 69, 68, 85, 62, 13, 10, 77, 101, 115, 115, 97, 103, 101, 45, 73, 100, 58, 32, 60, 66, 50, 55, 51, 57, 55, 45, 48, 49, 48, 48, 48, 48, 48, 64, 99, 97, 99, 46, 119, 97, 115, 104, 105, 110, 103, 116, 111, 110, 46, 101, 100, 117, 62, 13, 10, 77, 73, 77, 69, 45, 86, 101, 114, 115, 105, 111, 110, 58, 32, 49, 46, 48, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 84, 69, 88, 84, 47, 80, 76, 65, 73, 78, 59, 32, 67, 72, 65, 82, 83, 69, 84, 61, 85, 83, 45, 65, 83, 67, 73, 73, 13, 10, 13, 10], sync: true }))) }]) })
// * 12 FETCH (BODY[HEADER] {342}
// Date: Wed, 17 Jul 1996 02:23:25 -0700 (PDT)
// From: Terry Gray <gray@cac.washington.edu>
// Subject: IMAP4rev1 WG mtg summary and minutes
// To: imap@cac.washington.edu
// cc: minutes@CNRI.Reston.VA.US, John Klensin <KLENSIN@MIT.EDU>
// Message-Id: <B27397-0100000@cac.washington.edu>
// MIME-Version: 1.0
// Content-Type: TEXT/PLAIN; CHARSET=US-ASCII
// 
// )

// a004 OK FETCH completed
Status(Ok { tag: Some(Tag("a004")), code: None, text: Text("FETCH completed") })
// a004 OK FETCH completed

// a005 store 12 +flags \deleted
Command { tag: Tag("a005"), body: Store { sequence_set: SequenceSet(NonEmptyVec([Single(Value(12))])), kind: Add, response: Answer, flags: [Deleted], uid: false } }
// a005 STORE 12 +FLAGS (\Deleted)

// * 12 FETCH (FLAGS (\Seen \Deleted))
Data(Fetch { seq_or_uid: 12, attributes: NonEmptyVec([Flags([Flag(Seen), Flag(Deleted)])]) })
// * 12 FETCH (FLAGS (\Seen \Deleted))

// a005 OK +FLAGS completed
Status(Ok { tag: Some(Tag("a005")), code: None, text: Text("+FLAGS completed") })
// a005 OK +FLAGS completed

// a006 logout
Command { tag: Tag("a006"), body: Logout }
// a006 LOGOUT

// * BYE IMAP4rev1 server terminating connection
Status(Bye { code: None, text: Text("IMAP4rev1 server terminating connection") })
// * BYE IMAP4rev1 server terminating connection

// a006 OK LOGOUT completed
Status(Ok { tag: Some(Tag("a006")), code: None, text: Text("LOGOUT completed") })
// a006 OK LOGOUT completed
```

## A Note on IMAP literals

IMAP literals make separating the parsing logic from the application logic difficult.
When a parser recognizes a literal (e.g. "{42}"), a so-called continuation response (`+ ...`) must be sent.
Otherwise, the client or server will not send more data, and a parser would always return `Incomplete(42)`.

A possible solution is to implement a framing codec first.
This strategy is motivated by the IMAP RFC:

```
The protocol receiver of an IMAP4rev1 client or server is either reading a line,
or is reading a sequence of octets with a known count followed by a line.
```

The framing codec can be implemented like this ...

```rust
loop {
    line = read_line()
    if line.has_literal() {
        literal = read_literal(amount)
    }
}
```

... and variants of this procedure are provided in the [parse_command] example and the [demo server].

# License

This crate is dual-licensed under Apache 2.0 and MIT terms.

[IMAP4rev1]: https://tools.ietf.org/html/rfc3501
[imap-types]: https://github.com/duesee/imap-codec/imap-types
[formal syntax]: https://tools.ietf.org/html/rfc3501#section-9
[documentation]: https://docs.rs/imap-codec/latest/imap_codec/
[cargo fuzz]: https://github.com/rust-fuzz/cargo-fuzz
[demo client]: https://github.com/duesee/imap-codec/tree/main/assets/demos/tokio_client
[demo server]: https://github.com/duesee/imap-codec/tree/main/assets/demos/tokio_server
[parse_command]: https://github.com/duesee/imap-codec/blob/main/examples/parse_command.rs

# Thanks

Thanks to the [NLnet Foundation](https://nlnet.nl/) for supporting imap-codec through their [NGI Assure](https://nlnet.nl/assure/) program!

<div align="right">
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/215262095-ab12d43a-ca8a-4d44-b79b-7e99ab91ca01.png"/>
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/221422192-60d28ed4-10bb-441e-957d-93af58166707.png"/>
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/215262235-0db02da9-7c6c-498e-a3d2-7ea7901637bf.png"/>
</div>
