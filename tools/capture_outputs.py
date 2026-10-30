"""Run every lesson once and save what it prints, so the site shows real output.

    python tools/capture_outputs.py

Writes course/lessons/<lesson>/OUTPUT.txt. Interactive or never-ending lessons get scripted input
or are skipped (see SPECIAL). Programs with random keys or timestamps print different values on
each run; the site says so next to their output.
"""
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parent.parent
COURSE = ROOT / "course"
# lesson -> (stdin text, extra args, cargo flags); None = don't run (a server that never exits)
SPECIAL = {
    "04-collections-inventory": ("list\nadd Dune | Frank Herbert | 2\nsell dune\nsell Dune\nsell Dune\nlist\nexit\n", [], []),
    "10-tcp-echo": None,
    "13-blockchain": ("", [], ["--release"]),
}

for lesson in sorted((COURSE / "lessons").iterdir()):
    name = lesson.name
    spec = SPECIAL.get(name, ("", [], []))
    out = lesson / "OUTPUT.txt"
    if spec is None:
        out.unlink(missing_ok=True)
        print(f"{name}: skipped (runs until stopped)")
        continue
    stdin, args, flags = spec
    r = subprocess.run(["cargo", "run", "-q", *flags, "-p", f"lesson-{name}", "--", *args],
                       cwd=COURSE, input=stdin, text=True, capture_output=True, timeout=600)
    if r.returncode != 0:
        raise SystemExit(f"{name} failed:\n{r.stderr}")
    text = r.stdout.replace("\r\n", "\n")
    if name == "04-collections-inventory":  # show the commands that were typed, like a terminal would
        header, _, body = text.partition("\n")
        chunks = body.split("> ")[1:]  # one chunk of output per prompt
        lines = [header]
        for cmd, chunk in zip(stdin.splitlines(), chunks):
            lines.append(f"> {cmd}")
            lines += [l for l in chunk.splitlines() if l]
        text = "\n".join(lines) + "\n"
    out.write_text(text, encoding="utf-8", newline="\n")
    print(f"{name}: {len(text.splitlines())} lines")
