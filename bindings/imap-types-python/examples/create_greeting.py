import imap_types


greeting = imap_types.Greeting({"kind": "Ok", "text": "Hello, world!"})
print(type(greeting))
print(repr(greeting))
print(str(greeting))

greeting_with_code = imap_types.Greeting(
    {"kind": "Ok", "code": "Alert", "text": "Hello, world!"}
)
print(type(greeting_with_code))
print(repr(greeting_with_code))
print(str(greeting_with_code))
