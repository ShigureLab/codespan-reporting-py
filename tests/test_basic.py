import textwrap

from codespan_reporting import (
    Config,
    Diagnostic,
    Label,
    SimpleFiles,
    StandardStream,
    emit,
)


def test_basic():
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
