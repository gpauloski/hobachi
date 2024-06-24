from __future__ import annotations

from hobachi import Proxy


def test_bool() -> None:
    proxy = Proxy(lambda: True)
    assert proxy

    class Foo:
        def __bool__(self) -> bool:
            return False

    proxy = Proxy(lambda: Foo())
    assert not proxy
