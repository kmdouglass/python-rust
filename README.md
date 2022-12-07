# Python extensions in Rust

![build](https://github.com/kmdouglass/python-rust/actions/workflows/build.yml/badge.svg)

Template repo for writing Python extensions in Rust.

Based on documentation at https://www.maturin.rs/index.html.

## Getting started

```console
$ python -m venv .venv
$ source .venv/bin/activate
(.venv) $ pip install .
(.venv) $ python
>>> from python_lib import StateMachine, par_run
>>> machines = [StateMachine for _ in range(100)]
>>> _ = par_run(machines)

```

## Project layout

The [src/python](src/python) folder contains example Python objects written in Rust. These are exported into the Python extension module in [src/python_module.rs](src/python_module.rs).

## Development

### Setup the development environment

#### Linux and macOS

1. Install [Rust](https://www.rust-lang.org/learn/get-started): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install [pyenv](https://github.com/pyenv/pyenv): `curl https://pyenv.run | bash`
3. Install Python interpreter(s) listed in the file `.python-version`:

```console
while read line; do
  pyenv install "$line"
done < .python-version
```
4. Create a virtual environment: `python3 -m venv .venv`
5. Activate the venv: `source .venv/bin/activate`
6. Install the dependencies: `pip install -e .[develop]`

### Testing and linting

#### Rust

Run tests:

```console
cargo test
```

Run linters:

 ``` console
 cargo fmt -v --check
 cargo clippy
 ```

 #### Python

 Run all tests and linters:

 ```console
 tox
 ```

 Run specific linters:

```console
# Black, isort, mypy, pylint
tox -e black
# etc. ...
```

### Formatting

#### Rust

```console
cargo fmt
```

#### Python

```console
tox -e format
```
