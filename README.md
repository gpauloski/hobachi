# hobachi

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

## TODO

- [x] Implement getattr, delattr, setattr
- [x] Create Python tests
- [x] Add CI
- [x] Add pre-commit
- [ ] Add branch protection / other github metadata
- [ ] Finish implementation of special methods
- [ ] Add docs
- [ ] Add release mechanism
