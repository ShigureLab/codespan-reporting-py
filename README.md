# codespan-reporting-py

A python binding for [codespan-reporting](https://github.com/brendanzab/codespan)

<p align="center">
   <a href="https://python.org/" target="_blank"><img alt="PyPI - Python Version" src="https://img.shields.io/pypi/pyversions/codespan-reporting?logo=python&style=flat-square"></a>
   <a href="https://pypi.org/project/codespan-reporting/" target="_blank"><img src="https://img.shields.io/pypi/v/codespan-reporting?style=flat-square" alt="pypi"></a>
   <a href="https://pypi.org/project/codespan-reporting/" target="_blank"><img alt="PyPI - Downloads" src="https://img.shields.io/pypi/dm/codespan-reporting?style=flat-square"></a>
   <a href="LICENSE"><img alt="LICENSE" src="https://img.shields.io/github/license/ShigureLab/codespan-reporting?style=flat-square"></a>
   <br/>
   <a href="https://github.com/astral-sh/ruff"><img alt="ruff" src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json&style=flat-square"></a>
   <a href="https://gitmoji.dev"><img alt="Gitmoji" src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67?style=flat-square"></a>
</p>

## Installation

Requires Python 3.11 or newer. CPython 3.15 (including the free-threaded `3.15t`
build) is tested with native wheels. Standard CPython uses `cp311-abi3` wheels;
free-threaded 3.15 uses separate `cp315-cp315t` wheels for Linux x86-64/AArch64,
Windows x86-64, and macOS x86-64/Apple Silicon. Python 3.15 is currently tested
against its release candidate; CI allows prerelease interpreters until the final
release is available.

The extension does not require the GIL. Concurrent rendering can share files and
diagnostics, but callers must synchronize mutation of a shared `SimpleFiles`
instance (for example, calls to `add` while another thread renders it).

```bash
pip install codespan-reporting
```

## Usage

```python
import textwrap

from codespan_reporting import (
    Config,
    Diagnostic,
    Label,
    SimpleFiles,
    StandardStream,
    emit,
)

files = SimpleFiles()

file_id = files.add(
    "FizzBuzz.fun",
    textwrap.dedent(
        """\
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        """,
    ),
)

diagnostic = Diagnostic.error(
    "E0308",
    "`case` clauses have incompatible types",
    [
        Label.primary(file_id, 328, 331, "expected `String`, found `Nat`"),
        Label.secondary(file_id, 211, 331, "`case` clauses have incompatible types"),
        Label.secondary(file_id, 258, 268, "this is found to be of type `String`"),
        Label.secondary(file_id, 284, 290, "this is found to be of type `String`"),
        Label.secondary(file_id, 306, 312, "this is found to be of type `String`"),
        Label.secondary(file_id, 186, 192, "expected type `String` found here"),
    ],
    [
        textwrap.dedent(
            """\
            expected type `String`
                found type `Nat`
        """,
        )
    ],
)

writer = StandardStream.Stderr
config = Config()
emit(writer, config, files, diagnostic)
```
