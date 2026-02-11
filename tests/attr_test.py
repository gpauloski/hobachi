from __future__ import annotations

import contextlib

from hobachi import Proxy


class Foo:
    def __init__(self) -> None:
        self.bar = 42


def test_getattr() -> None:
    proxy = Proxy(Foo)
    assert proxy.bar == Foo().bar


def test_setattr() -> None:
    proxy = Proxy(Foo)
    assert proxy.bar != 0

    proxy.bar = 0
    assert proxy.bar == 0


def test_delattr() -> None:
    proxy = Proxy(Foo)
    assert proxy.bar != 0

    delattr(proxy, 'bar')
    with contextlib.suppress(AttributeError):
        _ = proxy.bar
