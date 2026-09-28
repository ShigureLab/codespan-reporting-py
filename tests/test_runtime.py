import json
import os
import subprocess
import sys
import sysconfig
from concurrent.futures import ThreadPoolExecutor

import pytest

from codespan_reporting import Config, Diagnostic, Label, SimpleFiles, StandardStream, emit


def test_native_import_and_gil():
    # Use a fresh process with the default GIL policy: forcing PYTHON_GIL=0
    # would hide an extension which silently re-enables the GIL on import.
    env = os.environ.copy()
    env.pop("PYTHON_GIL", None)
    result = subprocess.run(
        [
            sys.executable,
            "-Werror::RuntimeWarning",
            "-c",
            """
import importlib.machinery
import json
import sys
import sysconfig

free_threaded = bool(sysconfig.get_config_var("Py_GIL_DISABLED"))
is_gil_enabled = getattr(sys, "_is_gil_enabled", lambda: True)
assert is_gil_enabled() == (not free_threaded)
from codespan_reporting import _core
assert any(_core.__file__.endswith(suffix) for suffix in importlib.machinery.EXTENSION_SUFFIXES)
assert is_gil_enabled() == (not free_threaded)
print(json.dumps({
    "version": f"{sys.version_info.major}.{sys.version_info.minor}" + ("t" if free_threaded else ""),
    "extension": _core.__file__,
}))
""",
        ],
        env=env,
        capture_output=True,
        text=True,
        check=True,
    )
    runtime = json.loads(result.stdout)
    current_version = f"{sys.version_info.major}.{sys.version_info.minor}"
    if sysconfig.get_config_var("Py_GIL_DISABLED"):
        current_version += "t"
    assert runtime["version"] == os.environ.get("EXPECTED_PYTHON_VERSION", current_version)


def test_invalid_file_id():
    diagnostic = Diagnostic.error("E001", "missing file", [Label.primary(0, 0, 1, "missing")], [])
    with pytest.raises(ValueError):
        emit(StandardStream.Stderr, Config(), SimpleFiles(), diagnostic)


def test_parallel_native_rendering(capfd: pytest.CaptureFixture[str]):
    # Read-only objects may be shared. SimpleFiles.add mutates its receiver and
    # callers must serialize mutation of a shared instance, as with other PyO3
    # mutable classes. Build this shared source before starting the threads.
    files = SimpleFiles()
    file_id = files.add("parallel.py", "value = 1\n")
    config = Config()
    diagnostic = Diagnostic.error("E001", "parallel diagnostic", [Label.primary(file_id, 0, 5, "value")], [])

    def render(_: int) -> None:
        emit(StandardStream.Stderr, config, files, diagnostic)
        local_files = SimpleFiles()
        assert local_files.add("thread.py", "x\n") == 0
        local_diagnostic = Diagnostic.note("N001", "thread-local diagnostic", [Label.primary(0, 0, 1, "x")], [])
        emit(StandardStream.Stderr, config, local_files, local_diagnostic)

    with ThreadPoolExecutor(max_workers=8) as executor:
        list(executor.map(render, range(32)))

    output = capfd.readouterr().err
    assert output.count("parallel diagnostic") == 32
    assert output.count("thread-local diagnostic") == 32
    assert output.count("parallel.py") == 32
    assert output.count("thread.py") == 32
