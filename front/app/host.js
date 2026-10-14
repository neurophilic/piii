// SPDX-License-Identifier: LicenseRef-PiTechLab-Proprietary
// The host for ScholarPi's Rust front end (back/rust/ui, built to ui.wasm).
//
// WebAssembly cannot touch the page by itself, so this lends it the DOM, the network and storage as numbered
// handles. It holds no views, no text and no logic of the site: all of that is in the Rust crate.
const nodes = [null], freed = [], forms = [null], files = [null];
const enc = new TextEncoder(), dec = new TextDecoder();
let W, capturing = false;

const str = (p, l) => dec.decode(new Uint8Array(W.memory.buffer, p, l));
function put(s) {
  const b = enc.encode(s), p = W.alloc(b.length);
  new Uint8Array(W.memory.buffer, p, b.length).set(b);
  return [p, b.length];
}
function ret(s) {
  if (s == null) return -1;
  const b = enc.encode(s), p = W.ret(b.length);
  new Uint8Array(W.memory.buffer, p, b.length).set(b);
  return b.length;
}
function keep(n) {
  const id = freed.length ? freed.pop() : nodes.length;
  nodes[id] = n;
  return id;
}
function send(id, kind, value) {
  const [kp, kl] = put(kind), [vp, vl] = put(value);
  W.event(id, kp, kl, vp, vl);
}
function fileList(list) {
  const out = [];
  for (const f of list) { out.push(files.length + "\t" + f.size + "\t" + f.name.replace(/[\t\n]/g, " ")); files.push(f); }
  return out.join("\n");
}
const safe = (f, d) => { try { return f(); } catch (_) { return d; } };

const host = {
  create: (p, l) => {
    const t = str(p, l);
    return keep(t.startsWith("svg:") ? document.createElementNS("http://www.w3.org/2000/svg", t.slice(4)) : document.createElement(t));
  },
  text: (p, l) => keep(document.createTextNode(str(p, l))),
  set_text: (id, p, l) => { nodes[id].data = str(p, l); },
  attr: (id, kp, kl, vp, vl) => nodes[id].setAttribute(str(kp, kl), str(vp, vl)),
  unattr: (id, kp, kl) => nodes[id].removeAttribute(str(kp, kl)),
  prop: (id, kp, kl, vp, vl) => {
    const n = nodes[id], k = str(kp, kl), v = str(vp, vl);
    if (k === "checked") n.checked = v === "1"; else if (n[k] !== v) n[k] = v;
  },
  append: (p, c) => nodes[p].appendChild(nodes[c]),
  insert: (p, c, b) => nodes[p].insertBefore(nodes[c], nodes[b]),
  remove: (id) => { nodes[id].remove(); nodes[id] = undefined; freed.push(id); },
  release: (id) => { nodes[id] = undefined; freed.push(id); },
  listen: (id, kp, kl) => {
    const n = nodes[id], k = str(kp, kl);
    if (k === "enter") n.addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.isComposing) { e.preventDefault(); send(id, k, n.value || ""); } });
    else if (k === "pdown") n.addEventListener("pointerdown", (e) => { if (e.button === 0) send(id, k, e.clientX + "," + e.clientY); });
    // a double click (left: "dbl", right: "dbl2") on the node itself, not on anything marked data-solid inside it
    // the pointer over a surface that draws itself (the map): x, y within it, and its width and height
    else if (k === "mdown" || k === "mmove" || k === "mup" || k === "mwheel") {
      const ev = { mdown: "pointerdown", mmove: "pointermove", mup: "pointerup", mwheel: "wheel" }[k];
      n.addEventListener(ev, (e) => {
        const r = n.getBoundingClientRect(), at = (e.clientX - r.left) + "," + (e.clientY - r.top) + "," + r.width + "," + r.height;
        if (k === "mwheel") { e.preventDefault(); send(id, k, e.deltaY + "," + at); return; }
        if (k === "mdown") { if (e.button !== 0 || e.target.closest("button, a, input, select")) return; try { n.setPointerCapture(e.pointerId); } catch (_) {} }
        send(id, k, at);
      }, k === "mwheel" ? { passive: false } : undefined);
      if (k === "mup") n.addEventListener("pointercancel", (e) => { const r = n.getBoundingClientRect(); send(id, k, (e.clientX - r.left) + "," + (e.clientY - r.top) + "," + r.width + "," + r.height); });
    }
    else if (k === "dbl") n.addEventListener("dblclick", (e) => { if (!e.target.closest("[data-solid]")) { e.preventDefault(); send(id, k, e.clientX + "," + e.clientY); } });
    else if (k === "dbl2") {
      let last = 0;
      n.addEventListener("contextmenu", (e) => {
        if (e.target.closest("[data-solid]")) return;
        e.preventDefault();
        const t = Date.now();
        if (t - last < 450) { last = 0; send(id, k, e.clientX + "," + e.clientY); } else last = t;
      });
    }
    // a key typed into something that is not a field (the HAL-OS screen): it goes only there
    else if (k === "key") n.addEventListener("keydown", (e) => {
      if (e.ctrlKey || e.metaKey || e.altKey) return;
      const key = e.key;
      if (key.length === 1 || key === "Enter" || key === "Backspace" || key === "Escape" || key === "Tab") { e.preventDefault(); e.stopPropagation(); send(id, k, key); }
    });
    else if (k === "files") n.addEventListener("change", () => { const v = fileList(n.files); n.value = ""; send(id, k, v); });
    else if (k === "drop") {
      n.addEventListener("dragover", (e) => e.preventDefault());
      n.addEventListener("drop", (e) => { e.preventDefault(); send(id, k, fileList(e.dataTransfer.files)); });
    } else n.addEventListener(k, (e) => {
      if (n.dataset && n.dataset.stop) e.stopPropagation();
      send(id, k, n.type === "checkbox" ? (n.checked ? "1" : "0") : (n.value ?? ""));
    });
  },
  body: () => keep(document.body),
  head: () => keep(document.head),
  fetch: (req, mp, ml, up, ul, bp, bl, form) => {
    const url = str(up, ul), init = { method: str(mp, ml), headers: {} };
    const tok = safe(() => localStorage.getItem("sp_token"), null);
    if (tok && url.startsWith("/api/")) init.headers.Authorization = "Bearer " + tok;
    if (form) { init.body = forms[form]; forms[form] = undefined; }
    else if (bl) { init.body = str(bp, bl); init.headers["Content-Type"] = "application/json"; }
    (async () => {
      let status = 0;
      try {
        const r = await fetch(url, init);
        status = r.status;
        const rd = r.body.getReader(), d = new TextDecoder();
        for (;;) {
          const { done, value } = await rd.read();
          if (done) break;
          const s = d.decode(value, { stream: true });
          if (s) { const [p, l] = put(s); W.fetched(req, status, p, l, 0); }
        }
        const [p, l] = put(d.decode());
        W.fetched(req, status, p, l, 1);
      } catch (e) {
        const [p, l] = put(String(e));
        W.fetched(req, 0, p, l, 1);
      }
    })();
  },
  form: () => { forms.push(new FormData()); return forms.length - 1; },
  form_str: (f, kp, kl, vp, vl) => forms[f].append(str(kp, kl), str(vp, vl)),
  form_file: (f, kp, kl, file) => { if (files[file]) forms[f].append(str(kp, kl), files[file]); },
  store_get: (kp, kl) => ret(safe(() => localStorage.getItem(str(kp, kl)), null)),
  store_set: (kp, kl, vp, vl) => safe(() => localStorage.setItem(str(kp, kl), str(vp, vl))),
  store_del: (kp, kl) => safe(() => localStorage.removeItem(str(kp, kl))),
  cookie: () => ret(document.cookie),
  set_cookie: (p, l) => { document.cookie = str(p, l); },
  href: () => ret(location.href),
  replace_url: (p, l) => history.replaceState(null, "", str(p, l)),
  go: (p, l) => { location.href = str(p, l); },
  open: (p, l) => window.open(str(p, l), "_blank", "noopener"),
  set_timer: (ms, token) => setTimeout(() => W.timer(token), ms),
  now: () => Date.now(),
  log_: (p, l) => console.log(str(p, l)),
  title: (p, l) => { document.title = str(p, l); },
  scroll_top: () => window.scrollTo(0, 0),
  focus: (id) => nodes[id].focus(),
  print: () => window.print(),
  vw: () => window.innerWidth,
  vh: () => window.innerHeight,
  capture: (on) => { capturing = !!on; document.documentElement.classList.toggle("dragging", capturing); },
  download: (np, nl, tp, tl, dp, dl) => {
    const blob = new Blob([new Uint8Array(W.memory.buffer, dp, dl).slice()], { type: str(tp, tl) });
    const a = Object.assign(document.createElement("a"), { href: URL.createObjectURL(blob), download: str(np, nl) });
    document.body.appendChild(a); a.click(); a.remove();
    setTimeout(() => URL.revokeObjectURL(a.href), 10000);
  },
  call: (op, ol, ap, al) => { const f = ops[str(op, ol)]; return ret(f ? safe(() => f(str(ap, al)), null) : null); },
};

// Helpers with no import of their own, called by name: host.call("audio", "play\turl\tloopStart\tloopLength").
let actx = null, song = null, songUrl = "", src = null, gain = null;
// Hands bytes back to the program: W.pixels(token, pointer, length, width, height).
function give(token, bytes, w, h) {
  const p = bytes.length ? W.alloc(bytes.length) : 0;
  if (bytes.length) new Uint8Array(W.memory.buffer, p, bytes.length).set(bytes);
  W.pixels(token, p, bytes.length, w, h);
}
const ops = {
  // the browser's Ethereum wallet: token, method, params (JSON); the answer comes back as {"ok": …} or {"err": code, "msg": …}
  eth: (a) => {
    const [token, method, params] = a.split("\t"), w = window.ethereum;
    if (!w || !w.request) return null;
    const back = (o) => give(+token, enc.encode(JSON.stringify(o)), 1, 1);
    w.request({ method, params: JSON.parse(params || "[]") }).then((r) => back({ ok: r ?? null }), (e) => back({ err: (e && e.code) || 0, msg: String((e && e.message) || e) }));
    return "1";
  },
  // RGBA bytes in the program's memory, onto the canvas a selector names: selector, pointer, length, width, height
  blit: (a) => {
    const [sel, p, l, w, h] = a.split("\t"), cv = document.querySelector(sel);
    if (!cv) return null;
    cv.getContext("2d").putImageData(new ImageData(new Uint8ClampedArray(W.memory.buffer, +p, +l).slice(), +w, +h), 0, 0);
    return "1";
  },
  copy: (a) => { navigator.clipboard && navigator.clipboard.writeText(a).catch(() => {}); return "1"; },
  // a chosen picture, decoded by the browser and shrunk to fit max × max: its RGBA pixels
  rgba: (a) => {
    const [token, file, max] = a.split("\t").map(Number);
    (async () => {
      try {
        const bmp = await createImageBitmap(files[file]);
        const k = Math.min(1, max / Math.max(bmp.width, bmp.height));
        const w = Math.max(1, Math.round(bmp.width * k)), h = Math.max(1, Math.round(bmp.height * k));
        const cv = typeof OffscreenCanvas !== "undefined" ? new OffscreenCanvas(w, h) : Object.assign(document.createElement("canvas"), { width: w, height: h });
        const ctx = cv.getContext("2d", { willReadFrequently: true });
        ctx.drawImage(bmp, 0, 0, w, h);
        if (bmp.close) bmp.close();
        give(token, ctx.getImageData(0, 0, w, h).data, w, h);
      } catch (_) { give(token, new Uint8Array(0), 0, 0); }
    })();
    return "1";
  },
  // a chosen file's bytes
  bytes: (a) => {
    const [token, file] = a.split("\t").map(Number);
    files[file].arrayBuffer().then((b) => give(token, new Uint8Array(b), 1, 1)).catch(() => give(token, new Uint8Array(0), 0, 0));
    return "1";
  },
  // a short tone (the game's sounds): from, to (Hz), ms, wave, gain
  tone: (a) => {
    const [f1, f2, ms, type, gain] = a.split("\t");
    const AC = window.AudioContext || window.webkitAudioContext;
    if (!AC) return null;
    if (!actx) actx = new AC();
    if (actx.state === "suspended") actx.resume().catch(() => {});
    const t0 = actx.currentTime, d = (+ms) / 1000, osc = actx.createOscillator(), amp = actx.createGain();
    osc.type = type || "sine";
    osc.frequency.setValueAtTime(+f1, t0);
    if (+f2 && +f2 !== +f1) osc.frequency.exponentialRampToValueAtTime(Math.max(1, +f2), t0 + d);
    amp.gain.setValueAtTime(0.0001, t0);
    amp.gain.exponentialRampToValueAtTime(+gain || 0.05, t0 + 0.012);
    amp.gain.exponentialRampToValueAtTime(0.0001, t0 + d);
    osc.connect(amp); amp.connect(actx.destination); osc.start(t0); osc.stop(t0 + d + 0.02);
    return "1";
  },
  // a chosen file, as an address the page can show
  fileurl: (a) => (files[+a] ? URL.createObjectURL(files[+a]) : ""),
  audio: (a) => {
    const [cmd, url, ls, ll] = a.split("\t");
    const AC = window.AudioContext || window.webkitAudioContext;
    if (!AC) return null;
    if (!actx) actx = new AC();
    if (cmd === "play") {
      if (actx.state === "suspended") actx.resume().catch(() => {});
      if (src) { gain.gain.setTargetAtTime(0.8, actx.currentTime, 0.08); return "1"; }
      const go = () => {
        if (src || !song) return;
        const d = song.getChannelData(0); let i = 0;
        while (i < d.length && Math.abs(d[i]) < 1e-4) i++;
        const at = i / song.sampleRate;
        src = actx.createBufferSource(); src.buffer = song; src.loop = true;
        src.loopStart = at; src.loopEnd = Math.min(song.duration, at + (+ll || song.duration));
        gain = actx.createGain(); gain.gain.setValueAtTime(0, actx.currentTime);
        gain.gain.linearRampToValueAtTime(0.8, actx.currentTime + 0.4);
        src.connect(gain).connect(actx.destination); src.start(actx.currentTime + 0.02, at + (+ls || 0));
      };
      if (song && songUrl === url) go();
      else { songUrl = url; fetch(url).then((r) => r.arrayBuffer()).then((b) => actx.decodeAudioData(b)).then((b) => { song = b; go(); }).catch(() => {}); }
      return "1";
    }
    if (cmd === "pause" && src && actx.state === "running") {
      gain.gain.setTargetAtTime(0, actx.currentTime, 0.08);
      setTimeout(() => { if (gain && gain.gain.value < 0.05) actx.suspend().catch(() => {}); }, 350);
    }
    return "1";
  },
};

const wasm = new URL("ui.wasm" + new URL(import.meta.url).search, import.meta.url);
const bytes = await (await fetch(wasm)).arrayBuffer();
W = (await WebAssembly.instantiate(bytes, { host })).instance.exports;
addEventListener("hashchange", () => W.nav());
addEventListener("resize", () => W.resized());
addEventListener("keydown", (e) => {
  const t = e.target;
  if (t && (t.tagName === "INPUT" || t.tagName === "TEXTAREA" || t.isContentEditable)) return;
  if (e.key === "Delete" || e.key === "Backspace" || e.key === "Escape") { const [p, l] = put(e.key); W.keyed(p, l); }
  // keys a game steers with, down (+) and up (−); the program says whether it used them
  if (STEER.has(e.key)) { const [p, l] = put("+" + e.key); if (W.keyed(p, l) === 1) e.preventDefault(); }
});
const STEER = new Set(["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "w", "a", "s", "d", "W", "A", "S", "D", "Escape"]);
addEventListener("keyup", (e) => { if (STEER.has(e.key)) { const [p, l] = put("-" + e.key); W.keyed(p, l); } });
addEventListener("pointermove", (e) => { if (capturing) W.pointer(0, e.clientX, e.clientY); });
addEventListener("pointerup", (e) => { if (capturing) W.pointer(1, e.clientX, e.clientY); });
addEventListener("pointercancel", (e) => { if (capturing) W.pointer(2, e.clientX, e.clientY); });
W.start();
