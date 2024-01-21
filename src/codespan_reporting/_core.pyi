# config.rs
class Config:
    def __init__(self) -> None: ...

# file.rs
class SimpleFiles:
    def __init__(self) -> None: ...
    def add(self, name: str, source: str) -> int: ...

# diagnostic.rs
class Severity:
    Bug: Severity
    Error: Severity
    Warning: Severity
    Note: Severity
    Help: Severity

class LabelStyle:
    Primary: LabelStyle
    Secondary: LabelStyle
    SecondaryUnderline: LabelStyle

type FileId = int

class Label:
    def __init__(
        self,
        label_style: LabelStyle,
        file_id: FileId,
        start: int,
        end: int,
        message: str,
    ) -> None: ...
    @staticmethod
    def primary(
        file_id: FileId,
        start: int,
        end: int,
        message: str,
    ) -> Label: ...
    @staticmethod
    def secondary(
        file_id: FileId,
        start: int,
        end: int,
        message: str,
    ) -> Label: ...

class Diagnostic:
    def __init__(
        self,
        severity: Severity,
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> None: ...
    @staticmethod
    def bug(
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> Diagnostic: ...
    @staticmethod
    def error(
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> Diagnostic: ...
    @staticmethod
    def warning(
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> Diagnostic: ...
    @staticmethod
    def note(
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> Diagnostic: ...
    @staticmethod
    def help(
        code: str,
        message: str,
        labels: list[Label],
        notes: list[str],
    ) -> Diagnostic: ...

# emit.rs
class StandardStream:
    Stdout: StandardStream
    Stderr: StandardStream

def emit(
    writer: StandardStream,
    config: Config,
    files: SimpleFiles,
    diagnostic: Diagnostic,
) -> None: ...
