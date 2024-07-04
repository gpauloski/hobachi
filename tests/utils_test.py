from __future__ import annotations

from hobachi import extract
from hobachi import is_resolved
from hobachi import Proxy
from hobachi import resolve


def test_extract() -> None:
    proxy = Proxy(lambda: 'value')
    assert isinstance(proxy, Proxy)
    extracted = extract(proxy)
    assert extracted == 'value'
    assert not isinstance(extract, Proxy)


def test_is_resolved() -> None:
    proxy = Proxy(lambda: 'value')
    assert not is_resolved(proxy)
    assert proxy == 'value'
    assert is_resolved(proxy)


def test_resolve() -> None:
    proxy = Proxy(lambda: 'value')
    assert not is_resolved(proxy)
    resolve(proxy)
    assert is_resolved(proxy)
