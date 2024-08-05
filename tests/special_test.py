from __future__ import annotations

import pytest

from hobachi import is_resolved
from hobachi import Proxy
from hobachi import resolve


def test_repr() -> None:
    proxy = Proxy(lambda: 'value')

    assert not is_resolved(proxy)
    assert repr(proxy).startswith('<Proxy with factory')
    resolve(proxy)
    assert repr(proxy).startswith('<Proxy wrapping value with factory')


def test_str() -> None:
    class Foo:
        def __str__(self) -> str:
            return 'bar'

    proxy = Proxy(lambda: Foo())
    assert str(proxy) == 'bar'


def test_hash() -> None:
    value = (1, 2, 3)
    proxy = Proxy(lambda: value)
    assert hash(proxy) == hash(value)


def test_call() -> None:
    class Foo:
        def __call__(self) -> str:
            return 'bar'

    proxy = Proxy(lambda: Foo())
    assert proxy() == 'bar'


def test_call_with_args() -> None:
    class Foo:
        def __call__(self, /, x: int, *, y: int) -> bool:
            return x != y

    proxy = Proxy(lambda: Foo())
    assert proxy(1, y=2)
    assert not proxy(1, y=1)


def test_bool() -> None:
    proxy = Proxy(lambda: True)
    assert proxy

    class Foo:
        def __bool__(self) -> bool:
            return False

    proxy = Proxy(lambda: Foo())
    assert not proxy


def test_cmp() -> None:
    proxy = Proxy(lambda: 0)

    assert proxy < 1
    assert proxy <= 0
    assert proxy <= 1
    assert proxy == 0
    assert proxy != 1
    assert proxy > -1
    assert proxy >= 0
    assert proxy >= -1


def test_iter() -> None:
    proxy = Proxy(lambda: [1, 2, 3])

    for x, y in zip(proxy, [1, 2, 3]):
        assert x == y


def test_next() -> None:
    class Foo:
        def __next__(self) -> int:
            return 1

    proxy = Proxy(lambda: Foo())
    assert next(proxy) == 1


def test_sequence() -> None:
    values = ['1', '2', '3']
    proxy = Proxy(lambda: values)
    assert len(proxy) == len(values)
    assert '1' in proxy
    assert proxy[1] == '2'
    proxy[1] = 'a'
    assert proxy[1] == 'a'


def test_mapping() -> None:
    mapping = {'a': 1, 'b': 2}
    proxy = Proxy(lambda: mapping)
    assert len(proxy) == len(mapping)
    assert proxy['a'] == 1
    proxy['c'] = 3
    assert len(proxy) == 3
    del proxy['c']
    assert len(proxy) == 2


def test_add() -> None:
    proxy = Proxy(lambda: 0)

    result = proxy + 1
    assert result == 1
    assert not isinstance(result, Proxy)


def test_sub() -> None:
    proxy = Proxy(lambda: 1)

    result = proxy - 1
    assert result == 0
    assert not isinstance(result, Proxy)


def test_mul() -> None:
    proxy = Proxy(lambda: 1)
    m = 42

    result = proxy * m
    assert result == m
    assert not isinstance(result, Proxy)


def test_truediv() -> None:
    proxy = Proxy(lambda: 4)

    result = proxy / 2
    assert result == 2
    assert not isinstance(result, Proxy)

    with pytest.raises(ZeroDivisionError):
        proxy / 0


def test_floordiv() -> None:
    proxy = Proxy(lambda: 5)

    result = proxy // 2
    assert result == 2
    assert not isinstance(result, Proxy)

    with pytest.raises(ZeroDivisionError):
        proxy / 0


def test_lshift() -> None:
    proxy = Proxy(lambda: 1)

    result = proxy << 1
    assert result == 2
    assert not isinstance(result, Proxy)


def test_rshift() -> None:
    proxy = Proxy(lambda: 2)

    result = proxy >> 1
    assert result == 1
    assert not isinstance(result, Proxy)
