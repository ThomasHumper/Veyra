# `test.py`

```python
#!/usr/bin/env python3

import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent
BUILD_DIR = ROOT / "build"


def run(command, cwd=ROOT):
    print(f"$ {' '.join(map(str, command))}")

    result = subprocess.run(
        command,
        cwd=cwd,
        text=True,
        capture_output=True,
    )

    if result.stdout:
        print(result.stdout, end="")

    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)

    return result


def find_executable():
    candidates = [
        BUILD_DIR / "veyra",
        BUILD_DIR / "Debug" / "veyra.exe",
        BUILD_DIR / "Release" / "veyra.exe",
        BUILD_DIR / "veyra.exe",
    ]

    for candidate in candidates:
        if candidate.exists():
            return candidate

    raise RuntimeError(
        "Could not find the Veyra executable in the build directory."
    )


def build():
    print("\n=== BUILD ===")

    result = run([
        "cmake",
        "-S", ".",
        "-B", str(BUILD_DIR),
    ])

    if result.returncode != 0:
        raise RuntimeError("CMake configuration failed.")

    result = run([
        "cmake",
        "--build",
        str(BUILD_DIR),
    ])

    if result.returncode != 0:
        raise RuntimeError("Compilation failed.")


def run_program(executable, source):
    with tempfile.NamedTemporaryFile(
        mode="w",
        suffix=".vy",
        delete=False,
        encoding="utf-8",
    ) as file:
        file.write(source)
        filename = Path(file.name)

    try:
        result = subprocess.run(
            [str(executable), str(filename)],
            cwd=ROOT,
            text=True,
            capture_output=True,
        )

        return result

    finally:
        filename.unlink(missing_ok=True)


TESTS = [
    {
        "name": "addition",
        "source": """
print(ka + ve);
""",
        "expected": "3",
    },

    {
        "name": "operator precedence",
        "source": """
print(ve + tri * nox);
""",
        "expected": "14",
    },

    {
        "name": "subtraction",
        "source": """
print(zen - sai);
""",
        "expected": "5",
    },

    {
        "name": "multiplication",
        "source": """
print(sai * ve);
""",
        "expected": "10",
    },

    {
        "name": "comparison true",
        "source": """
print(sai < zen);
""",
        "expected": "true",
    },

    {
        "name": "comparison false",
        "source": """
print(zen < sai);
""",
        "expected": "false",
    },

    {
        "name": "variables",
        "source": """
lum x = sai;
lum y = zen;

print(x + y);
""",
        "expected": "15",
    },

    {
        "name": "if true",
        "source": """
if zen > sai {
    print(zen);
} else {
    print(ka);
}
""",
        "expected": "10",
    },

    {
        "name": "if false",
        "source": """
if ka > zen {
    print(ka);
} else {
    print(zen);
}
""",
        "expected": "10",
    },

    {
        "name": "function",
        "source": """
fun add(a, b) {
    return a + b;
}

print(add(ve, tri));
""",
        "expected": "5",
    },

    {
        "name": "function multiplication",
        "source": """
fun multiply(a, b) {
    return a * b;
}

print(multiply(sai, ve));
""",
        "expected": "10",
    },

    {
        "name": "nested expression",
        "source": """
print((ve + tri) * nox);
""",
        "expected": "20",
    },

    {
        "name": "multiple statements",
        "source": """
print(ka);
print(ve);
print(tri);
""",
        "expected": "1\n2\n3",
    },
]


def run_tests(executable):
    print("\n=== TESTS ===\n")

    passed = 0
    failed = 0

    for test in TESTS:
        result = run_program(
            executable,
            test["source"],
        )

        actual = result.stdout.strip()
        expected = test["expected"].strip()

        if (
            result.returncode == 0
            and actual == expected
        ):
            print(f"PASS  {test['name']}")
            passed += 1
        else:
            print(f"FAIL  {test['name']}")
            print()
            print("Expected:")
            print(expected)
            print()
            print("Actual:")
            print(actual)

            if result.stderr:
                print()
                print("stderr:")
                print(result.stderr)

            print()
            failed += 1

    print("--------------------------------")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Total:  {passed + failed}")
    print("--------------------------------")

    return failed == 0


def main():
    try:
        build()

        executable = find_executable()

        print(f"\nUsing executable: {executable}")

        success = run_tests(executable)

        if success:
            print("\nAll tests passed. The revolution continues.")
            return 0

        print("\nSome tests failed.")
        return 1

    except Exception as error:
        print(f"\nTEST ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
```

## Run it

From the Veyra project directory:

```bash
python3 test.py
```

or on Windows:

```powershell
python test.py
```

You should get something like:

```text
=== TESTS ===

PASS  addition
PASS  operator precedence
PASS  subtraction
PASS  multiplication
PASS  comparison true
PASS  comparison false
PASS  variables
PASS  if true
PASS  if false
PASS  function
PASS  function multiplication
PASS  nested expression
PASS  multiple statements

--------------------------------
Passed: 13
Failed: 0
Total:  13
--------------------------------

All tests passed. The revolution continues.
```

The useful part is that the tester treats the Veyra interpreter as a black box: **source code in → process → output checked**. That means you can completely rewrite the lexer, parser, or interpreter internally without having to rewrite these tests.
