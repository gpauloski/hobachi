# hobachi

> :warning: This project was used to learn certain features of PyO3 and is not maintained.

Python object proxy with just-in-time resolution implemented in Rust.
*Hobachi* means a copy (noun) or to imitate (verb) in the Chickasaw and Choctaw languages.

This project is based on [python-lazy-object-proxy](https://github.com/ionelmc/python-lazy-object-proxy).

## Development

1. Create a Python virtual environment of your choice.
   ```bash
   $ python -m venv venv
   $ . venv/bin/activate
   ```
3. Build and install the package.
   ```bash
   $ pip install -e .[dev]
   $ pre-commit install
   ```
4. Run `pytest`.
   ```bash
   $ pytest
   ```

## Usage

A `Proxy` instance is initialized with a *factory* function that, when invoked, creates and returns a target object.
The proxy, once initialized, will defer invoking the factory until the first time the proxy is used (e.g., an attribute access is performed).
Once the factory has been called to retrieve the target, the target is cached inside of the proxy, and the proxy forwards all operations on itself to the target.
In essence, the proxy is a transparent wrapper around the target.

```python
import time
from hobachi import Proxy

def factory() -> list[int]:
    time.sleep(1)  # Simulate expensive function
    return [1, 2, 3]

proxy = Proxy(factory)  # Call to factory() is deferred

proxy.append(4)  # factory() is called on first proxy use
assert len(proxy) == 4  # proxy behaves like the target object
```

## TODO

- [x] Implement getattr, delattr, setattr
- [x] Create Python tests
- [x] Add CI
- [x] Add pre-commit
- [x] Add branch protection / other github metadata
- [ ] Add more special methods support
- [ ] Add docs
