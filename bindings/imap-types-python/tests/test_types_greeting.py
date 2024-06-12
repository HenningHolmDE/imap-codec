import unittest

from imap_types import Code, Greeting, GreetingKind


class TestGreetingKind(unittest.TestCase):
    def test_ok(self):
        self.assertEqual(str(GreetingKind.Ok), "GreetingKind.Ok")

    def test_pre_auth(self):
        self.assertEqual(str(GreetingKind.PreAuth), "GreetingKind.PreAuth")

    def test_bye(self):
        self.assertEqual(str(GreetingKind.Bye), "GreetingKind.Bye")


class TestGreeting(unittest.TestCase):
    def test_greeting_ok(self):
        greeting = Greeting(GreetingKind.Ok, "Hello, world!")
        self.assertEqual(str(greeting), "* OK Hello, world!\r\n")

    def test_greeting_bye(self):
        greeting = Greeting(GreetingKind.Bye, "Hello, world!")
        self.assertEqual(str(greeting), "* BYE Hello, world!\r\n")

    def test_greeting_with_code(self):
        greeting = Greeting(
            GreetingKind.Ok,
            "Hello, world!",
            Code("Alert"),
        )
        self.assertEqual(str(greeting), "* OK [ALERT] Hello, world!\r\n")
