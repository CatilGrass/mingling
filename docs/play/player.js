function parseTeachMarkdown(raw) {
  raw = raw.replace(/^```\s*\n?/, "");

  const blocks = raw.split(/^---\s*$/m);
  const steps = [];

  for (const block of blocks) {
    const text = block.trim();
    if (!text) continue;

    const codes = [];
    let match;
    const codeRe = /```(rust|toml|bash|text)\n([\s\S]*?)```/g;
    while ((match = codeRe.exec(text)) !== null) {
      const lang = match[1];
      const code = match[2].trimEnd();
      codes.push({ lang, code });
    }

    const last = codes.length > 0 ? codes[codes.length - 1] : null;
    const code = last ? { lang: last.lang, code: last.code } : null;

    const speech = text
      .replace(/```(rust|toml|bash|text)\n[\s\S]*?```/g, "")
      .trim();
    const paragraphs = speech
      .split(/\n+/)
      .map((s) => s.trim())
      .filter(Boolean);

    steps.push({ code, paragraphs });
  }

  let lastCode = "";
  let lastLang = "rust";
  let lastParagraphs = [];
  for (const step of steps) {
    if (step.code !== null) {
      lastCode = step.code.code;
      lastLang = step.code.lang;
      step.code = { lang: lastLang, code: lastCode };
    } else {
      step.code = { lang: lastLang, code: lastCode };
    }

    if (step.paragraphs.length > 0) {
      lastParagraphs = step.paragraphs;
    } else {
      step.paragraphs = lastParagraphs;
    }
  }

  return steps;
}

function renderCode(codeInfo) {
  if (!codeInfo || !codeInfo.code) return "";

  const lang = codeInfo.lang || "rust";
  const code = codeInfo.code;
  const lines = code.split("\n");
  const html = [];

  const plainText = lang === "text";

  for (let i = 0; i < lines.length; i++) {
    const lineNum = i + 1;
    const rawLine = lines[i];
    const prefix = `<span class="line-num">${lineNum}</span>`;

    if (plainText) {
      html.push(`${prefix}${escapeHtml(rawLine)}`);
      continue;
    }

    const hl = rawLine.match(/^(.*?)\s*<{8,}\s*"([^"]*?)"\s*$/);

    if (hl) {
      const indent = rawLine.match(/^\s*/)[0];
      const hlCode = hljs.highlight(indent + hl[1].trimStart(), {
        language: lang,
      }).value;
      html.push(
        `${prefix}${hlCode} <span class="bubble">${escapeHtml(hl[2])}</span>`,
      );
    } else if (rawLine.trim() === "") {
      html.push(prefix);
    } else {
      const hlLine = hljs.highlight(rawLine, { language: lang }).value;
      html.push(`${prefix}${hlLine}`);
    }
  }

  return html.join("\n");
}

function escapeHtml(text) {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function renderSpeech(paragraphs) {
  return paragraphs
    .map((p) => {
      if (p.startsWith("> ")) {
        return `<div class="cmd">${escapeHtml(p.slice(2))}</div>`;
      }
      let html = p
        .replace(/`([^`]+)`/g, "<code>$1</code>")
        .replace(
          /\*\*([^*]+)\*\*/g,
          '<strong style="color:#7dcfff">$1</strong>',
        );
      return `<p>${html}</p>`;
    })
    .join("");
}

const dom = {
  code: document.getElementById("codeDisplay"),
  speech: document.getElementById("speechDisplay"),
  stepNum: document.getElementById("stepNum"),
  totalSteps: document.getElementById("totalSteps"),
  progress: document.getElementById("progressFill"),
  progressTrack: document.querySelector(".controls__progress"),
  progressText: document.getElementById("progressText"),
  prevBtn: document.getElementById("prevBtn"),
  nextBtn: document.getElementById("nextBtn"),
};

let steps = [];
let currentStep = 0;

function goToStep(index) {
  currentStep = Math.max(0, Math.min(index, steps.length - 1));
  const step = steps[currentStep];

  dom.code.innerHTML = renderCode(step.code);
  dom.speech.innerHTML = renderSpeech(step.paragraphs);

  dom.stepNum.textContent = currentStep + 1;
  dom.totalSteps.textContent = steps.length;
  const pct =
    steps.length > 1
      ? Math.round((currentStep / (steps.length - 1)) * 100)
      : 100;
  dom.progress.style.width = pct + "%";
  dom.progressText.textContent = pct + "%";

  dom.prevBtn.disabled = currentStep === 0;
  const isLast = currentStep === steps.length - 1;
  dom.nextBtn.disabled = false;
  dom.nextBtn.textContent = isLast ? "🎉 COMPLETE" : "NEXT ▶";
}

dom.prevBtn.addEventListener("click", () => goToStep(currentStep - 1));
dom.nextBtn.addEventListener("click", () => {
  if (currentStep < steps.length - 1) goToStep(currentStep + 1);
});

function progressClick(e) {
  if (steps.length < 2) return;
  const rect = dom.progressTrack.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const pct = Math.max(0, Math.min(1, x / rect.width));
  const idx = Math.round(pct * (steps.length - 1));
  goToStep(idx);
}

dom.progressTrack.addEventListener("click", progressClick);

dom.progressTrack.addEventListener("mousedown", function (e) {
  e.preventDefault();
  progressClick(e);
  function onMove(ev) {
    progressClick(ev);
  }
  function onUp() {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
  }
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
});

document.addEventListener("keydown", (e) => {
  if (e.key === "ArrowRight" || e.key === " " || e.key === "Enter") {
    e.preventDefault();
    if (currentStep < steps.length - 1) goToStep(currentStep + 1);
  } else if (e.key === "ArrowLeft") {
    e.preventDefault();
    if (currentStep > 0) goToStep(currentStep - 1);
  }
});

async function loadTutorial(filePath) {
  try {
    const resp = await fetch(filePath);
    const raw = await resp.text();
    steps = parseTeachMarkdown(raw);
    dom.totalSteps.textContent = steps.length;
    goToStep(0);
  } catch (e) {
    dom.code.textContent = "Load tutorial failed: " + e.message;
  }
}

const params = new URLSearchParams(window.location.search);
const tutorialFile = "sources/" + (params.get("tur") || "default.md");

const title = params.get("title");
if (title) {
  document.getElementById("tutorialTitle").textContent = title;
}

const layoutEl = document.getElementById("layout");
const speechPanel = document.querySelector(".speech-panel");
const codePanel = document.querySelector(".code-panel");
layoutEl.append(speechPanel, codePanel);
layoutEl.classList.add("layout--tb");

loadTutorial(tutorialFile);
