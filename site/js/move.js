// The move: four steps of ownership. While it plays, the step text is not announced (aria-live off);
// when the visitor drives it (Next step, a guess, or reduced motion) each step is announced politely.
(() => {
  "use strict";
  const fig = document.getElementById("move");
  if (!fig) return;
  const text = document.getElementById("step-text");
  const live = document.getElementById("step-live");
  const pause = document.getElementById("move-pause");
  const next = document.getElementById("move-next");
  const guess = document.getElementById("guess");
  const STEPS = [
    "Line 2: <code>s</code> owns a string on the heap.",
    "Line 3: <code>let t = s;</code> moves it. <code>t</code> owns it now and <code>s</code> is empty.",
    "Line 4 uses <code>s</code> again. The compiler refuses: error E0382, borrow of moved value.",
    "The fix: <code>s.clone()</code> makes a second string, so both names own one. It compiles."
  ];
  const HOLD = [2800, 3200, 6000, 4200]; // the error step stays up longest
  let step = 1, timer = null, playing = false, guessed = false;

  const show = (n, prefix = "") => {
    step = n;
    fig.dataset.step = String(n);
    text.innerHTML = prefix + STEPS[n - 1];
    guess.hidden = !(n === 2 && !guessed);
  };
  const schedule = () => {
    clearTimeout(timer);
    if (!playing) return;
    // on step 2, wait for a guess (or move on after a while) before revealing the error
    timer = setTimeout(() => { show(step % 4 + 1); schedule(); }, step === 2 && !guessed ? 9000 : HOLD[step - 1]);
  };
  const setPlaying = (on) => {
    playing = on;
    live.setAttribute("aria-live", on ? "off" : "polite");
    pause.setAttribute("aria-pressed", String(!on));
    pause.textContent = on ? "Pause" : "Play";
    schedule();
  };

  guess.addEventListener("click", (e) => {
    const b = e.target.closest("[data-line]");
    if (!b) return;
    guessed = true;
    setPlaying(false);
    const reply = {
      "4": "<b>Right.</b> ",
      "3": "<b>Close: line 3 is where the move happens,</b> but the error is reported where <code>s</code> is used again. ",
      "2": "<b>Not line 2:</b> creating the string is fine. "
    }[b.dataset.line];
    show(3, reply);
  });
  next.addEventListener("click", () => { setPlaying(false); show(step % 4 + 1); });
  pause.addEventListener("click", () => setPlaying(!playing));

  if (matchMedia("(prefers-reduced-motion: reduce)").matches) {
    fig.classList.add("still");
    pause.hidden = true;
    show(2);
    live.setAttribute("aria-live", "polite");
    return;
  }
  show(1);
  setPlaying(true);
})();
