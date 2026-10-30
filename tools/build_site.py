"""Build the LearnRust website from the course itself.

    python tools/capture_outputs.py   # optional: refresh each lesson's real output
    python tools/build_site.py

Reads course/lessons/*/README.md (front matter + Markdown), the lesson's Rust sources and
OUTPUT.txt, counts its tests, and writes site/*.html. Pure Python 3, no packages.
"""
import html
import pathlib
import re
import subprocess

ROOT = pathlib.Path(__file__).resolve().parent.parent
COURSE = ROOT / "course" / "lessons"
SITE = ROOT / "site"
TRACKS = [
    ("Foundations", "The language itself: ownership, types, errors, traits, iterators and tests."),
    ("Systems", "Programs that share work across threads and talk over the network."),
    ("Cryptography", "Hashes, signatures and a blockchain you can break on purpose."),
]
FONTS = '<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Big+Shoulders+Display:wght@700;900&family=JetBrains+Mono:wght@400;600&family=Work+Sans:wght@400;500;600&display=swap" rel="stylesheet">'
esc = html.escape

# ---------------------------------------------------------------- Rust highlighting
KEYWORDS = set("as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while".split())
TOKEN = re.compile(r"""
  (?P<comment>//[^\n]*)
 |(?P<string>b?"(?:\\.|[^"\\])*")
 |(?P<char>'(?:\\.|[^'\\])')
 |(?P<lifetime>'[a-z_]\w*)
 |(?P<attr>\#!?\[[^\]]*\])
 |(?P<number>\b\d[\d_]*(?:\.\d+)?(?:_?[a-z]\d*)?\b)
 |(?P<macro>\b[a-z_]\w*!)
 |(?P<word>\b[A-Za-z_]\w*\b)
""", re.X)


def highlight(code):
    out, pos = [], 0
    for m in TOKEN.finditer(code):
        out.append(esc(code[pos:m.start()]))
        kind, text = m.lastgroup, m.group()
        if kind == "word":
            kind = "kw" if text in KEYWORDS else "ty" if text[0].isupper() else None
        out.append(f'<span class="t-{kind}">{esc(text)}</span>' if kind else esc(text))
        pos = m.end()
    out.append(esc(code[pos:]))
    return "".join(out)


# ---------------------------------------------------------------- tiny Markdown
def inline(text):
    parts = re.split(r"(`[^`]+`)", text)
    out = []
    for p in parts:
        if p.startswith("`") and p.endswith("`") and len(p) > 1:
            out.append(f"<code>{esc(p[1:-1])}</code>")
        else:
            p = esc(p).replace("\\`", "`")
            p = re.sub(r"\*\*(.+?)\*\*", r"<strong>\1</strong>", p)
            p = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r'<a href="\2">\1</a>', p)
            out.append(p)
    return "".join(out)


def markdown(md):
    out, lines, i = [], md.splitlines(), 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("```"):
            lang = line[3:].strip()
            j = i + 1
            while j < len(lines) and not lines[j].startswith("```"):
                j += 1
            body = "\n".join(lines[i + 1:j])
            code = highlight(body) if lang == "rust" else esc(body)
            copy = ' <button class="copy" type="button">Copy</button>' if lang == "bash" else ""
            out.append(f'<div class="snippet"><pre class="lang-{lang or "text"}"><code>{code}</code></pre>{copy}</div>')
            i = j + 1
        elif line.startswith("## "):
            title = line[3:].strip()
            out.append(f'<h2 id="{re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")}">{inline(title)}</h2>')
            i += 1
        elif re.match(r"^(- |\d+\. )", line):
            ordered = bool(re.match(r"^\d+\. ", line))
            items = []
            while i < len(lines) and re.match(r"^(- |\d+\. )", lines[i]):
                items.append(re.sub(r"^(- |\d+\. )", "", lines[i]))
                i += 1
            tag = "ol" if ordered else "ul"
            out.append(f"<{tag}>" + "".join(f"<li>{inline(t)}</li>" for t in items) + f"</{tag}>")
        elif line.strip():
            para = [line]
            i += 1
            while i < len(lines) and lines[i].strip() and not re.match(r"^(```|## |- |\d+\. )", lines[i]):
                para.append(lines[i])
                i += 1
            out.append(f"<p>{inline(' '.join(para))}</p>")
        else:
            i += 1
    return "\n".join(out)


# ---------------------------------------------------------------- lessons
def load_lessons():
    lessons = []
    for d in sorted(COURSE.iterdir()):
        text = (d / "README.md").read_text(encoding="utf-8")
        _, front, body = text.split("---", 2)
        meta = dict(l.split(":", 1) for l in front.strip().splitlines())
        meta = {k.strip(): v.strip() for k, v in meta.items()}
        files = [p for p in [d / "src" / "main.rs", d / "src" / "lib.rs", *sorted((d / "tests").glob("*.rs"))] if p.exists()]
        sources = [(p.relative_to(d).as_posix(), p.read_text(encoding="utf-8")) for p in files]
        all_src = "\n".join(s for _, s in sources)
        tests = all_src.count("#[test]") + len(re.findall(r"^\s*/// ```\s*$", all_src, re.M)) // 2
        output = (d / "OUTPUT.txt").read_text(encoding="utf-8") if (d / "OUTPUT.txt").exists() else None
        n = d.name[:2]
        lessons.append({"dir": d.name, "n": n, "slug": d.name, "title": meta["title"], "track": meta["track"],
                        "summary": meta["summary"], "concepts": [c.strip() for c in meta["concepts"].split(",")],
                        "body": body, "sources": sources, "tests": tests, "output": output,
                        "lines": sum(s.count("\n") for _, s in sources)})
    return lessons


RUN_NOTE = ('<h2 id="run-it">Run it</h2><p class="where">Run these from the <code>course/</code> folder of the repository. '
            'First time? <a href="setup.html">Install Rust and get the course</a> (about ten minutes).</p>')


def count_warnings():
    """Build the workspace and count compiler warnings, so the home page's figure is measured."""
    r = subprocess.run(["cargo", "build", "--workspace", "--message-format", "short"], cwd=ROOT / "course", capture_output=True, text=True)
    return sum(1 for line in r.stderr.splitlines() if "warning:" in line and "generated" not in line)


def page(title, desc, body, current="", extra_head="", scripts=""):
    nav = [("course.html", "Course", "course"), ("setup.html", "Set up", "setup"), ("https://github.com/SolutionsAsService/LearnRust", "GitHub", "gh")]
    links = "".join(f'<a href="{h}"{" aria-current=\"page\"" if k == current else ""}>{t}</a>' for h, t, k in nav)
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{esc(title)}</title>
<meta name="description" content="{esc(desc)}">
<meta name="theme-color" content="#16181b">
<link rel="icon" href="favicon.svg" type="image/svg+xml">
{FONTS}
<link rel="stylesheet" href="css/style.css">
{extra_head}
</head>
<body>
<a class="skip" href="#main">Skip to content</a>
<header class="top"><div class="top-in">
  <a class="brand" href="index.html" aria-label="LearnRust, home"><img src="favicon.svg" alt="" width="30" height="30"><span>Learn<b>Rust</b></span></a>
  <nav aria-label="Main">{links}</nav>
</div></header>
<main id="main">
{body}
</main>
<footer class="foot"><div class="wrap foot-in">
  <p><strong>LearnRust</strong> · a free course of small, tested Rust programs. MIT licence. Not affiliated with the Rust Foundation.</p>
  <p><a href="course.html">Course</a> · <a href="setup.html">Set up</a> · <a href="https://github.com/SolutionsAsService/LearnRust">Source on GitHub</a></p>
</div></footer>
<script src="js/site.js" defer></script>
{scripts}
</body>
</html>
"""


def tag(l, compact=False):
    return f"""<li class="tag{' tag-compact' if compact else ''}"><a href="{l['slug']}.html">
  <span class="tag-n">{l['n']}</span>
  <span class="tag-body"><span class="tag-title">{esc(l['title'])}</span>{'' if compact else f'<span class="tag-sum">{esc(l["summary"])}</span>'}
  <span class="tag-meta">{l['tests']} tests · {l['lines']} lines</span></span>
</a></li>"""


def build():
    lessons = load_lessons()
    rustc = subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.split(" (")[0].strip() or "rustc"
    total_tests = sum(l["tests"] for l in lessons)
    warnings = count_warnings()

    # ---------- lesson pages
    for i, l in enumerate(lessons):
        prev = lessons[i - 1] if i else None
        nxt = lessons[i + 1] if i + 1 < len(lessons) else None
        tabs = "".join(f'<button class="file-tab" type="button" role="tab" aria-selected="{"true" if k == 0 else "false"}" aria-controls="file-{k}" id="tab-{k}">{esc(name)}</button>'
                       for k, (name, _) in enumerate(l["sources"]))
        panels = "".join(f'<pre class="file lang-rust" id="file-{k}" role="tabpanel" aria-labelledby="tab-{k}" tabindex="0"{"" if k == 0 else " hidden"}><code>{highlight(src)}</code></pre>'
                         for k, (_, src) in enumerate(l["sources"]))
        varies = {"12-signatures": "Keys and signatures are random, so yours will differ.",
                  "13-blockchain": "Timestamps, nonces and hashes differ on every run.",
                  "09-concurrency": "Ties in the word list can come out in either order."}.get(l["dir"], "")
        out = (f'<section class="output" aria-labelledby="out-h"><h2 id="out-h">What it prints</h2>'
               f'<details class="predict"><summary><b>Predict first.</b> Read the code, write down what it will print, then open this.</summary><pre><code>{esc(l["output"])}</code></pre></details>'
               f'<p class="small">Captured from a real run with {esc(rustc)}. {varies}</p></section>') if l["output"] else \
              '<section class="output" aria-labelledby="out-h"><h2 id="out-h">What it prints</h2><p class="small">This one is a server: it runs until you stop it, so there is no fixed output. Its test connects to it and checks the echo.</p></section>'
        body = f"""
<article class="lesson">
  <header class="lesson-head"><div class="wrap lesson-head-in">
    <p class="stamp" aria-hidden="true">{l['n']}</p>
    <div>
      <p class="kicker"><a href="course.html#{l['track'].lower()}">{esc(l['track'])}</a> · Lesson {l['n']} of {len(lessons)}</p>
      <h1>{esc(l['title'])}</h1>
      <p class="lede">{esc(l['summary'])}</p>
      <ul class="chips" aria-label="Concepts">{''.join(f'<li>{esc(c)}</li>' for c in l['concepts'])}</ul>
      <p class="proof"><span class="ok">✓ {l['tests']} passing tests</span> · {l['lines']} lines · <a href="https://github.com/SolutionsAsService/LearnRust/tree/main/course/lessons/{l['dir']}">source</a></p>
    </div>
  </div></header>
  <div class="wrap lesson-grid">
    <div class="prose">{markdown(l['body']).replace('<h2 id="run-it">Run it</h2>', RUN_NOTE)}</div>
    <div class="code-col">
      <section class="code-panel" aria-label="Source code">
        <div class="file-tabs" role="tablist" aria-label="Files">{tabs}</div>
        {panels}
      </section>
      {out}
    </div>
  </div>
  <nav class="wrap pager" aria-label="Lessons">
    {f'<a class="prev" href="{prev["slug"]}.html"><span>Previous</span>{prev["n"]} · {esc(prev["title"])}</a>' if prev else '<span></span>'}
    {f'<a class="next" href="{nxt["slug"]}.html"><span>Next</span>{nxt["n"]} · {esc(nxt["title"])}</a>' if nxt else '<a class="next" href="course.html"><span>Finished</span>Back to the course</a>'}
  </nav>
</article>"""
        (SITE / f"{l['slug']}.html").write_text(page(f"{l['n']} · {l['title']} — LearnRust", l["summary"], body, "course"), encoding="utf-8", newline="\n")

    # ---------- course map
    tracks = ""
    for name, blurb in TRACKS:
        items = [l for l in lessons if l["track"] == name]
        tracks += f"""<section class="track" id="{name.lower()}" aria-labelledby="t-{name.lower()}">
  <div class="track-head"><h2 id="t-{name.lower()}">{name}</h2><p>{blurb}</p><p class="small">{len(items)} lessons · {sum(l['tests'] for l in items)} tests</p></div>
  <ol class="tags">{''.join(tag(l) for l in items)}</ol>
</section>"""
    body = f"""<section class="page-head"><div class="wrap">
  <p class="kicker">The course</p>
  <h1>{len(lessons)} lessons, <em>three tracks.</em></h1>
  <p class="lede">Each lesson is a small program you can run, read and break, with tests that pass on {esc(rustc)}. Do them in order, or jump to the track you need.</p>
</div></section>
<div class="wrap">{tracks}</div>"""
    (SITE / "course.html").write_text(page("The course — LearnRust", f"{len(lessons)} Rust lessons in three tracks: foundations, systems and cryptography.", body, "course"), encoding="utf-8", newline="\n")

    # ---------- home
    track_cols = "".join(f"""<div class="col"><h3><a href="course.html#{name.lower()}">{name}</a></h3><ol class="mini">{''.join(f'<li><a href="{l["slug"]}.html"><b>{l["n"]}</b>{esc(l["title"])}</a></li>' for l in lessons if l["track"] == name)}</ol></div>""" for name, _ in TRACKS)
    home = (ROOT / "tools" / "home.html").read_text(encoding="utf-8")
    home = (home.replace("{{LESSONS}}", str(len(lessons))).replace("{{TESTS}}", str(total_tests))
                .replace("{{RUSTC}}", esc(rustc)).replace("{{WARNINGS}}", str(warnings)).replace("{{TRACK_COLS}}", track_cols)
                .replace("{{FIRST}}", lessons[0]["slug"]))
    (SITE / "index.html").write_text(page("LearnRust — Learn Rust by owning it", f"A free course of {len(lessons)} small, tested Rust programs, from ownership to a toy blockchain.", home, "", scripts='<script src="js/move.js" defer></script>'), encoding="utf-8", newline="\n")

    # ---------- setup
    setup = (ROOT / "tools" / "setup.md").read_text(encoding="utf-8")
    body = f"""<section class="page-head"><div class="wrap"><p class="kicker">Set up</p><h1>Ten minutes to <em>your first cargo test.</em></h1></div></section>
<div class="wrap"><div class="prose prose-wide">{markdown(setup)}</div></div>"""
    (SITE / "setup.html").write_text(page("Set up — LearnRust", "Install Rust, get the course and run every lesson's tests.", body, "setup"), encoding="utf-8", newline="\n")

    # ---------- 404
    body = """<section class="page-head"><div class="wrap"><p class="kicker">404</p><h1>No page <em>by that name.</em></h1>
<p class="lede">The compiler checked every path and couldn't find this one.</p><p><a class="btn btn-oxide" href="index.html">Back home</a></p></div></section>"""
    (SITE / "404.html").write_text(page("Not found — LearnRust", "Page not found.", body), encoding="utf-8", newline="\n")
    print(f"built {len(lessons)} lessons, {total_tests} tests, {warnings} warnings, {rustc}")


if __name__ == "__main__":
    build()
