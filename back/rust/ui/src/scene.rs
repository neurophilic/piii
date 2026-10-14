// The canvas, as the site has always looked (owner, 2026-10-14: "keep the wiring and original look", "keep all the
// supers"): a blue frame round the page; the title super; the controls (save, lock, chain, reset) along the top and
// the arrows along the bottom; and in the middle the site's super holding the welcome, Tools, Connect, Privacy
// choices and the holder with Explore, Library, Lab and the project. Supers are joined by wires between the ports
// on their edges (✦), as written in the site's program (pitechlab.sasm): ▷ an output, ◁ an input, △ sync,
// ▽ condition. A wire from an output is magenta; the others are ink.
//
// Wiring, by the logic of the superellipse language (github.com/ex-tern/superellipse, src/super.rs and Pact): a
// super has four terminals at its extreme points. Out, on the right, is where a wire leaves; in (left), control (top)
// and gate (bottom) are where wires arrive, and data flows from left to right. Click an out, then a tip: a wire. Click
// a control or gate, then another super's control or gate: what arrives at the one is passed on to the other. Esc, or
// a click anywhere else, lets go; a double click on a wire, or a click and Delete, takes it away. A wire is refused for
// the language's reasons (a node can not feed itself, out is where a wire leaves, a super has one gate, already wired,
// that would make a loop), and the reason is said. Save, lock, chain and reset are the canvas's controls: their out
// goes into a control tip, and what arrives at a super's control (wired, or passed on) acts on it: lock holds it in
// place, chain makes it bring what it holds, save keeps the layout, and wiring reset puts it back where it started.
// Wires are drawn as the language draws them: the ins of one tip share a bus, control wires are dashed, gate wires
// dotted, every wire ends in an arrowhead. Dragging a bar out of a bubble onto anything opens what it stands for.
//
// Supers can be dragged (unless locked); with the chain on, the holder brings what it holds. Save keeps the
// layout in this browser, reset brings back the start. A click opens a super: it fills the canvas, every other
// super becomes an icon in the rail (owner, 2026-10-14: "everything selected/open should be completely shown fit
// the canvas and supers, other things get smaller turn into icons"). Included into app.rs (it shares its state).

const S_TITLE: usize = 0;
const S_SAVE: usize = 1;
const S_LOCK: usize = 2;
const S_CHAIN: usize = 3;
const S_RESET: usize = 4;
const S_LEFT: usize = 5;
const S_UP: usize = 6;
const S_DOWN: usize = 7;
const S_RIGHT: usize = 8;
const S_SITE: usize = 9;
const S_WELCOME: usize = 10;
const S_TOOLS: usize = 11;
const S_CONNECT: usize = 12;
const S_PRIVACY: usize = 13;
const S_HOLD: usize = 14;
const S_EXPLORE: usize = 15;
const S_LIBRARY: usize = 16;
const S_LAB: usize = 17;
const S_PROJECT: usize = 18;
// the program (program.rs): its super, the palette, the examples, and run, step and clear
const S_PROG: usize = 19;
const S_PALETTE: usize = 20;
const S_EXAMPLES: usize = 21;
const S_RUN: usize = 22;
const S_STEP: usize = 23;
const S_CLEAR: usize = 24;
const NSUP: usize = 25;

type Rect = (f64, f64, f64, f64);

/// Where each super starts (the controls and arrows spaced so that one's out and the next one's in do not touch), in a design canvas (wide: 1440 × 900, as the site is drawn on a laptop; narrow: a phone).
fn base(narrow: bool) -> ([Rect; NSUP], f64, f64) {
    if !narrow {
        (
            [
                (27.0, 15.0, 206.0, 44.0),
                (560.0, 9.0, 50.0, 48.0),
                (644.0, 9.0, 50.0, 48.0),
                (728.0, 9.0, 50.0, 48.0),
                (812.0, 9.0, 50.0, 48.0),
                (540.0, 843.0, 50.0, 48.0),
                (624.0, 843.0, 50.0, 48.0),
                (756.0, 843.0, 50.0, 48.0),
                (840.0, 843.0, 50.0, 48.0),
                (24.0, 96.0, 676.0, 420.0),
                (44.0, 118.0, 300.0, 98.0),
                (44.0, 236.0, 300.0, 168.0),
                (44.0, 428.0, 64.0, 64.0),
                (132.0, 436.0, 170.0, 48.0),
                (370.0, 118.0, 310.0, 378.0),
                (398.0, 146.0, 104.0, 100.0),
                (552.0, 146.0, 104.0, 100.0),
                (398.0, 366.0, 104.0, 100.0),
                (552.0, 366.0, 104.0, 100.0),
                (724.0, 96.0, 692.0, 734.0),
                (370.0, 540.0, 330.0, 220.0),
                (24.0, 540.0, 320.0, 220.0),
                (996.0, 9.0, 50.0, 48.0),
                (1080.0, 9.0, 50.0, 48.0),
                (1164.0, 9.0, 50.0, 48.0),
            ],
            1440.0,
            900.0,
        )
    } else {
        // a phone: the same canvas, stacked (the site, then the examples and palette, then the program), scaled to fit
        (
            [
                (14.0, 14.0, 190.0, 44.0),
                (330.0, 12.0, 50.0, 48.0),
                (412.0, 12.0, 50.0, 48.0),
                (494.0, 12.0, 50.0, 48.0),
                (576.0, 12.0, 50.0, 48.0),
                (0.0, 1460.0, 50.0, 48.0),
                (0.0, 1460.0, 50.0, 48.0),
                (0.0, 1460.0, 50.0, 48.0),
                (0.0, 1460.0, 50.0, 48.0),
                (8.0, 80.0, 684.0, 420.0),
                (24.0, 96.0, 310.0, 110.0),
                (24.0, 222.0, 310.0, 160.0),
                (24.0, 400.0, 64.0, 64.0),
                (112.0, 408.0, 170.0, 48.0),
                (360.0, 96.0, 316.0, 390.0),
                (390.0, 124.0, 104.0, 100.0),
                (546.0, 124.0, 104.0, 100.0),
                (390.0, 350.0, 104.0, 100.0),
                (546.0, 350.0, 104.0, 100.0),
                (8.0, 748.0, 684.0, 700.0),
                (354.0, 516.0, 338.0, 216.0),
                (8.0, 516.0, 330.0, 216.0),
                (240.0, 1460.0, 50.0, 48.0),
                (324.0, 1460.0, 50.0, 48.0),
                (408.0, 1460.0, 50.0, 48.0),
            ],
            700.0,
            1515.0,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Side {
    L,
    R,
    T,
    B,
}

/// A super's four terminals, at its extreme points (±a, 0) and (0, ±b).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tip {
    In,
    Out,
    Control,
    Gate,
}

impl Tip {
    fn name(self) -> &'static str {
        match self {
            Tip::In => "in",
            Tip::Out => "out",
            Tip::Control => "control",
            Tip::Gate => "gate",
        }
    }
    fn code(self) -> u8 {
        match self {
            Tip::In => 0,
            Tip::Out => 1,
            Tip::Control => 2,
            Tip::Gate => 3,
        }
    }
    fn of(c: u8) -> Option<Tip> {
        [Tip::In, Tip::Out, Tip::Control, Tip::Gate].into_iter().find(|t| t.code() == c)
    }
    fn side(self) -> Side {
        match self {
            Tip::In => Side::L,
            Tip::Out => Side::R,
            Tip::Control => Side::T,
            Tip::Gate => Side::B,
        }
    }
}

/// The page's own super, for drawing; and the page's edge as two ends of a program: what comes in at its left
/// (a source, with only an out) and what goes out at its right (a sink, with only an in).
const FRAME: usize = 99;
const PAGE_IN: usize = 97;
const PAGE_OUT: usize = 98;

/// A wire: from a node's out to another's tip.
type WireT = (usize, usize, Tip);
/// A pass: what arrives at one super's control or gate, passed on to another's.
type PassT = (usize, Tip, usize, Tip);

/// The start (pitechlab.sasm, `diagram`), in the language's terms.
const START_WIRES: [WireT; 6] = [
    (PAGE_IN, S_SITE, Tip::In),
    (S_TOOLS, S_EXPLORE, Tip::In),
    (S_EXPLORE, S_LIBRARY, Tip::In),
    (S_LAB, S_PROJECT, Tip::In),
    (S_SAVE, S_TOOLS, Tip::Control),
    (S_LOCK, S_SITE, Tip::Control),
];
const START_PASSES: [PassT; 4] = [
    (S_SITE, Tip::Control, S_CONNECT, Tip::Control),
    (S_SITE, Tip::Control, S_HOLD, Tip::Control),
    (S_EXPLORE, Tip::Gate, S_LAB, Tip::Control),
    (S_LIBRARY, Tip::Gate, S_PROJECT, Tip::Control),
];

/// Supers taken off the canvas (owner, 2026-10-14: the arrows, "delete these icons"); the title still moves the view.
fn is_gone(id: usize) -> bool {
    matches!(id, S_LEFT | S_UP | S_DOWN | S_RIGHT)
}

fn is_ctl_super(id: usize) -> bool {
    matches!(id, S_SAVE | S_LOCK | S_CHAIN | S_RESET)
}

/// The tips a node has: the page's edges one each; every super all four (owner, 2026-10-14: "make everything
/// wirable"), the title, the controls and the arrows too.
fn tips_of(id: usize) -> &'static [Tip] {
    match id {
        PAGE_IN => &[Tip::Out],
        PAGE_OUT => &[Tip::In],
        _ => &[Tip::In, Tip::Out, Tip::Control, Tip::Gate],
    }
}

fn has_tip(id: usize, tip: Tip) -> bool {
    tips_of(id).contains(&tip)
}

/// Is there a path of wires (or passes) from `a` to `b`?
fn reaches(s: &State, a: usize, b: usize) -> bool {
    let (mut stack, mut seen) = (alloc::vec![a], Vec::new());
    while let Some(n) = stack.pop() {
        if n == b {
            return true;
        }
        if !seen.contains(&n) {
            seen.push(n);
            stack.extend(s.wires.iter().filter(|w| w.0 == n).map(|w| w.1));
            stack.extend(s.passes.iter().filter(|p| p.0 == n).map(|p| p.2));
        }
    }
    false
}

/// Has the super a gate already, wired or passed on to it?
fn gated(s: &State, id: usize) -> bool {
    s.wires.iter().any(|w| w.1 == id && w.2 == Tip::Gate) || s.passes.iter().any(|p| p.2 == id && p.3 == Tip::Gate)
}

/// Is there a path of wires (or passes) from `a` to `b` that does not pass through a register? (A loop through a
/// register is how a program repeats; one without would never settle.)
fn reaches_direct(s: &State, a: usize, b: usize) -> bool {
    let (mut stack, mut seen) = (alloc::vec![a], Vec::new());
    while let Some(n) = stack.pop() {
        if n == b {
            return true;
        }
        if !seen.contains(&n) && (n == a || !is_reg(s, n)) {
            seen.push(n);
            stack.extend(s.wires.iter().filter(|w| w.0 == n).map(|w| w.1));
            stack.extend(s.passes.iter().filter(|p| p.0 == n).map(|p| p.2));
        }
    }
    false
}

fn loops(s: &State, from: usize, to: usize) -> bool {
    !is_reg(s, from) && !is_reg(s, to) && reaches_direct(s, to, from)
}

/// Can `from`'s out be wired to `to`'s `tip`? Or why not.
fn can_connect(s: &State, from: usize, to: usize, tip: Tip) -> Result<(), &'static str> {
    if from == to && !is_reg(s, from) {
        return Err("a node can not feed itself (a register can: reg feeds its own next value)");
    }
    if !has_tip(from, Tip::Out) {
        return Err("the page's right edge has no out");
    }
    if tip == Tip::Out {
        return Err("out is where a wire leaves, not where it arrives");
    }
    if !has_tip(to, tip) {
        return Err(if to == PAGE_OUT { "the page's edge takes only an in" } else { "this super has no inputs" });
    }
    if is_ctl_super(from) && tip != Tip::Control {
        return Err("save, lock, chain and reset go into a control");
    }
    if tip == Tip::Gate && gated(s, to) {
        return Err("a super has one gate");
    }
    if s.wires.contains(&(from, to, tip)) {
        return Err("already wired");
    }
    if loops(s, from, to) {
        return Err("that would make a loop with no register in it: put a reg in the loop");
    }
    Ok(())
}

/// Can what arrives at `from`'s `ft` be passed on to `to`'s `tt`? Or why not.
fn can_relay(s: &State, from: usize, ft: Tip, to: usize, tt: Tip) -> Result<(), &'static str> {
    for (id, tip) in [(from, ft), (to, tt)] {
        if !matches!(tip, Tip::Control | Tip::Gate) {
            return Err("only a control or a gate passes things on");
        }
        if !has_tip(id, tip) {
            return Err("only a super's control and gate pass things on");
        }
    }
    if from == to {
        return Err("a super can not pass to itself");
    }
    if s.passes.contains(&(from, ft, to, tt)) {
        return Err("already wired");
    }
    if tt == Tip::Gate && gated(s, to) {
        return Err("a super has one gate");
    }
    if loops(s, from, to) {
        return Err("that would make a loop with no register in it: put a reg in the loop");
    }
    Ok(())
}

/// The nodes whose outs arrive at `id`'s `tip`: wired there, or passed on to it.
fn arriving(s: &State, id: usize, tip: Tip, depth: u32) -> Vec<usize> {
    let mut v: Vec<usize> = s.wires.iter().filter(|w| w.1 == id && w.2 == tip).map(|w| w.0).collect();
    if depth < 32 {
        for p in s.passes.iter().filter(|p| p.2 == id && p.3 == tip) {
            v.extend(arriving(s, p.0, p.1, depth + 1));
        }
    }
    v
}

fn wires_save() {
    let s = st();
    let mut out = String::from("v2;");
    for (a, b, t) in &s.wires {
        out.push_str(&format!("w{}-{}-{};", a, b, t.code()));
    }
    for (a, ta, b, tb) in &s.passes {
        out.push_str(&format!("p{}-{}-{}-{};", a, ta.code(), b, tb.code()));
    }
    host::store_set("piii:wires2", &out);
}

fn wires_load() {
    let s = st();
    s.wires = START_WIRES.to_vec();
    s.passes = START_PASSES.to_vec();
    let Some(v) = host::store_get("piii:wires2") else { return };
    let mut it = v.split(';');
    if it.next() != Some("v2") {
        return;
    }
    let ok = |id: usize| (id < NSUP && !is_gone(id)) || id == PAGE_IN || id == PAGE_OUT || s.prog.nodes.iter().any(|n| n.id == id);
    let (mut ws, mut ps): (Vec<WireT>, Vec<PassT>) = (Vec::new(), Vec::new());
    for e in it.filter(|e| !e.is_empty()) {
        let n: Vec<usize> = e[1..].split('-').filter_map(|x| x.parse().ok()).collect();
        match (e.as_bytes()[0], n.as_slice()) {
            (b'w', [a, b, t]) if ok(*a) && ok(*b) => {
                if let Some(t) = Tip::of(*t as u8) {
                    ws.push((*a, *b, t));
                }
            }
            (b'p', [a, ta, b, tb]) if ok(*a) && ok(*b) => {
                if let (Some(ta), Some(tb)) = (Tip::of(*ta as u8), Tip::of(*tb as u8)) {
                    ps.push((*a, ta, *b, tb));
                }
            }
            _ => {}
        }
    }
    s.wires = ws;
    s.passes = ps;
}

/// Does a control have a wire out (it turns cobalt)?
fn ctl_wired(s: &State, ctl: usize) -> bool {
    s.wires.iter().any(|w| w.0 == ctl)
}

fn locked(s: &State, id: usize) -> bool {
    s.locked || arriving(s, id, Tip::Control, 0).contains(&S_LOCK)
}

fn chain_on(s: &State, id: usize) -> bool {
    s.chained || arriving(s, id, Tip::Control, 0).contains(&S_CHAIN)
}

fn name_of(id: usize) -> String {
    match id {
        FRAME => "the page".to_string(),
        PAGE_IN => "the page's left edge".to_string(),
        PAGE_OUT => "the page's right edge".to_string(),
        S_SITE => "the site".to_string(),
        S_HOLD => "the holder".to_string(),
        _ if is_unode(id) => match unode(st(), id) {
            Some(n) => format!("“{}”", n.text),
            None => "a node".to_string(),
        },
        _ => sup_label(id).to_string(),
    }
}

/// A bar of a bubble being dragged out: wherever it lands, what it stands for opens.
#[derive(Clone, Copy)]
pub enum From {
    Bar(usize, usize),
}

pub struct Wiring {
    from: From,
    at: (f64, f64),
    to: Option<(usize, Side)>,
}

/// What moves with a super when the chain is on.
fn held_by(id: usize) -> &'static [usize] {
    match id {
        S_HOLD => &[S_EXPLORE, S_LIBRARY, S_LAB, S_PROJECT],
        S_SITE => &[S_WELCOME, S_TOOLS, S_CONNECT, S_PRIVACY, S_HOLD, S_EXPLORE, S_LIBRARY, S_LAB, S_PROJECT],
        _ => &[],
    }
}

// ---------------------------------------------------------------------------------------------- the bubbles' members

#[derive(Clone, Copy)]
enum Target {
    /// One of the site's windows, by its address.
    Hash(&'static str),
    /// A file or another site, in a new tab.
    Url(&'static str),
    /// Words, shown in the member's window.
    Words(&'static str),
    /// What an endpoint answers, laid out.
    Data(&'static str, &'static str),
    /// A tool that ran in JavaScript and is being rewritten in Rust.
    Soon(&'static str),
    /// A tool rewritten in Rust (tools.rs).
    Tool(Tool),
}

struct Group {
    sup: usize,
    title: &'static str,
    members: &'static [(&'static str, Target)],
}

const GROUPS: [Group; 5] = [
    Group {
        sup: S_TOOLS,
        title: "Tools",
        members: &[
            ("Assess Manuscripts [Pi]", Target::Hash("#/assess")),
            ("Assist Diagnosis [Intern]", Target::Tool(Tool::Intern)),
            ("FaceMace", Target::Tool(Tool::Face)),
        ],
    },
    Group {
        sup: S_EXPLORE,
        title: "Explore",
        members: &[
            ("Live", Target::Hash("#/live")),
            ("The journal", Target::Hash("#/explore/journal")),
            ("Leaderboards", Target::Tool(Tool::Board)),
            ("Super AI", Target::Tool(Tool::Ask)),
            ("Proof-of-Research Ledger Explorer", Target::Tool(Tool::Ledger)),
            ("Recent assessments", Target::Tool(Tool::Recent)),
            ("Analytics", Target::Tool(Tool::Analytics)),
            ("Performance", Target::Tool(Tool::Perf)),
            ("Minting Difficulty", Target::Hash("#/library")),
        ],
    },
    Group {
        sup: S_LIBRARY,
        title: "Library",
        members: &[
            ("RiBD", Target::Tool(Tool::Buddy)),
            ("The Global Map of Science", Target::Tool(Tool::Map)),
            ("Whitepaper", Target::Url("/ScholarPi_Whitepaper.pdf")),
            ("Architecture", Target::Tool(Tool::Arch)),
            ("SciM Assistant", Target::Tool(Tool::Scim)),
            ("Capabilities", Target::Words(
                "siM is the lab's own assistant model. riB learns which papers are worth reading. piD forecasts from the \
                 ledger. PiEn learns how the site is used. Super AI joins them, with Llama, Mistral, Qwen, Gemini and \
                 DeepSeek as independent jurors and a judge over them.",
            )),
            ("Neuro Frenzy", Target::Tool(Tool::Frenzy)),
            ("MD", Target::Tool(Tool::Md)),
        ],
    },
    Group {
        sup: S_LAB,
        title: "Lab",
        members: &[
            ("QuVI", Target::Tool(Tool::Quvi)),
            ("HAL-OS", Target::Soon("HAL-OS booted an x86 machine in your browser.")),
            ("Unigyro", Target::Tool(Tool::Unigyro)),
            ("Private projects", Target::Tool(Tool::Private)),
            ("Permission requests", Target::Tool(Tool::Requests)),
        ],
    },
    Group {
        sup: S_CONNECT,
        title: "Connect",
        members: &[
            ("Your account", Target::Hash("#/account")),
            ("Your assessments", Target::Hash("#/account")),
            ("Research profile", Target::Tool(Tool::Profile)),
            ("piQ held for you", Target::Hash("#/account")),
            ("Assessment results", Target::Hash("#/assess")),
            ("Owner console", Target::Tool(Tool::Owner)),
            ("Why sign in", Target::Words(
                "Signing in with ORCID links your assessments to you: they stay in your history, and the piQ they earn \
                 is minted to you instead of held in escrow. Anonymous visitors have a short free trial.",
            )),
            ("Invite a researcher", Target::Words(
                "Send a colleague to www.piii.app. Researchers signed in with ORCID can request and give peer reviews \
                 of each other's papers.",
            )),
            ("Support Pi", Target::Tool(Tool::Donate)),
            ("Contact us", Target::Tool(Tool::Bug)),
            ("GitHub", Target::Url("https://github.com/ex-tern/pi")),
        ],
    },
];

fn group_of(sup: usize) -> Option<usize> {
    GROUPS.iter().position(|g| g.sup == sup)
}

// ---------------------------------------------------------------------------------------------- geometry

struct Geo {
    k: f64,
    ox: f64,
    oy: f64,
    w: f64,
    h: f64,
    rects: [Rect; NSUP],
    /// the program's nodes, placed
    extra: Vec<(usize, Rect)>,
}

/// Where a super or a program node is, in design units.
fn rect(g: &Geo, id: usize) -> Rect {
    if id < NSUP {
        return g.rects[id];
    }
    g.extra.iter().find(|(i, _)| *i == id).map(|(_, r)| *r).unwrap_or((0.0, 0.0, 0.0, 0.0))
}

fn geo(s: &State) -> Geo {
    let (w, h) = (host::vw(), host::vh());
    geo_box(s, w, h, w < 700.0)
}

/// The canvas laid out in a box of w × h: the whole page, or the floating π canvas over an open super.
fn geo_box(s: &State, w: f64, h: f64, narrow: bool) -> Geo {
    let (b, dw, dh) = base(narrow);
    let k = (w / dw).min(h / dh).max(0.2);
    let ox = (w - dw * k) / 2.0;
    let oy = (h - dh * k) / 2.0;
    let mut rects = b;
    for (i, r) in rects.iter_mut().enumerate() {
        let (dx, dy) = s.offs[i];
        let pan = if i >= S_SITE && !matches!(i, S_RUN | S_STEP | S_CLEAR) { s.pan } else { (0.0, 0.0) };
        r.0 += dx + pan.0;
        r.1 += dy + pan.1;
    }
    let (px, py, _, _) = rects[S_PROG];
    let extra = s.prog.nodes.iter().map(|n| {
        let (w, h) = unode_size(&n.text);
        (n.id, (px + n.x, py + n.y, w, h))
    }).collect();
    Geo { k, ox, oy, w, h, rects, extra }
}

/// A port, in design units (the frame's own ports sit on the page's edge).
fn port(g: &Geo, id: usize, side: Side) -> (f64, f64) {
    if id == FRAME {
        let (l, t) = (-g.ox / g.k, -g.oy / g.k);
        let (r, b) = ((g.w - g.ox) / g.k, (g.h - g.oy) / g.k);
        return match side {
            Side::L => (l + 1.0, (t + b) / 2.0),
            Side::R => (r - 1.0, (t + b) / 2.0),
            Side::T => ((l + r) / 2.0, t + 1.0),
            Side::B => ((l + r) / 2.0, b - 1.0),
        };
    }
    let (x, y, w, h) = rect(g, id);
    match side {
        Side::L => (x, y + h / 2.0),
        Side::R => (x + w, y + h / 2.0),
        Side::T => (x + w / 2.0, y),
        Side::B => (x + w / 2.0, y + h),
    }
}

fn normal(side: Side) -> (f64, f64) {
    match side {
        Side::L => (-1.0, 0.0),
        Side::R => (1.0, 0.0),
        Side::T => (0.0, -1.0),
        Side::B => (0.0, 1.0),
    }
}

/// An ink wire: a curve leaving each port straight out of its edge.
fn curve(a: (f64, f64), sa: Side, b: (f64, f64), sb: Side) -> String {
    let d = ((b.0 - a.0).abs() + (b.1 - a.1).abs()).max(60.0) * 0.4;
    // a wire out of a top port runs down into what it feeds, as the site draws it
    let na = if sa == Side::T && b.1 > a.1 { (0.0, 1.0) } else { normal(sa) };
    let nb = normal(sb);
    // a wire into an input on the same side (main◁ site◁) runs straight
    let nb = if sa == sb && (sa == Side::L || sa == Side::R) { (-nb.0, -nb.1) } else { nb };
    format!(
        "M{:.1} {:.1}C{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}",
        a.0,
        a.1,
        a.0 + na.0 * d,
        a.1 + na.1 * d,
        b.0 + nb.0 * d,
        b.1 + nb.1 * d,
        b.0,
        b.1
    )
}

/// A data wire: out of an output, across in straight runs with rounded corners, into an input.
fn step(a: (f64, f64), b: (f64, f64)) -> String {
    let mx = (a.0 + b.0) / 2.0;
    let dy = b.1 - a.1;
    if dy.abs() < 1.0 {
        return format!("M{:.1} {:.1}H{:.1}", a.0, a.1, b.0);
    }
    let r = 12.0f64.min(dy.abs() / 2.0).min((mx - a.0).abs()).max(0.0);
    let sy = if dy > 0.0 { 1.0 } else { -1.0 };
    let sx = if b.0 >= a.0 { 1.0 } else { -1.0 };
    format!(
        "M{:.1} {:.1}H{:.1}Q{:.1} {:.1} {:.1} {:.1}V{:.1}Q{:.1} {:.1} {:.1} {:.1}H{:.1}",
        a.0,
        a.1,
        mx - sx * r,
        mx,
        a.1,
        mx,
        a.1 + sy * r,
        b.1 - sy * r,
        mx,
        b.1,
        mx + sx * r,
        b.1,
        b.0
    )
}

/// A port's mark: a four-pointed star.
fn star(x: f64, y: f64, r: f64) -> String {
    format!(
        "M{x:.1} {:.1}Q{x:.1} {y:.1} {:.1} {y:.1}Q{x:.1} {y:.1} {x:.1} {:.1}Q{x:.1} {y:.1} {:.1} {y:.1}Q{x:.1} {y:.1} {x:.1} {:.1}Z",
        y - r,
        x + r,
        y + r,
        x - r,
        y - r
    )
}

// ---------------------------------------------------------------------------------------------- icons

fn svg_icon(paths: &[&str], size: &str, cls: &str) -> N {
    el("svg:svg")
        .cls(cls)
        .attr("viewBox", "0 0 24 24")
        .attr("width", size)
        .attr("height", size)
        .attr("fill", "none")
        .attr("stroke", "currentColor")
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .attr("aria-hidden", "true")
        .kids(paths.iter().map(|d| -> N { el("svg:path").attr("d", *d).into() }))
        .into()
}

fn control_icon(id: usize, s: &State) -> N {
    match id {
        S_SAVE => svg_icon(&["M5 3h11l3 3v15H5z", "M8 3v6h8V3", "M8 21v-7h8v7"], "26", if ctl_wired(s, S_SAVE) { "ci accent" } else { "ci" }),
        S_LOCK => {
            if s.locked || ctl_wired(s, S_LOCK) {
                el("svg:svg")
                    .cls("ci accent")
                    .attr("viewBox", "0 0 24 24")
                    .attr("width", "26")
                    .attr("height", "26")
                    .attr("aria-hidden", "true")
                    .kid(el("svg:path").attr("d", "M7 10V7a5 5 0 0 1 10 0v3").attr("fill", "none").attr("stroke", "currentColor").attr("stroke-width", "2.4"))
                    .kid(el("svg:rect").attr("x", "4.5").attr("y", "10").attr("width", "15").attr("height", "11").attr("rx", "2").attr("fill", "currentColor"))
                    .into()
            } else {
                svg_icon(&["M7 10V7a5 5 0 0 1 9.6-2", "M4.5 10h15v11h-15z"], "26", "ci")
            }
        }
        S_CHAIN => svg_icon(
            &["M9 12h6", "M8 8H6.5a4 4 0 0 0 0 8H8", "M16 8h1.5a4 4 0 0 1 0 8H16"],
            "28",
            if ctl_wired(s, S_CHAIN) { "ci accent" } else if s.chained { "ci" } else { "ci faint" },
        ),
        S_RESET => svg_icon(&["M4 4v6h6", "M4.6 15a8 8 0 1 0 1.9-8.3L4 10"], "24", "ci"),
        S_LEFT => svg_icon(&["M19 12H5", "M11 6l-6 6 6 6"], "26", "ci"),
        S_UP => svg_icon(&["M12 19V5", "M6 11l6-6 6 6"], "26", "ci"),
        S_DOWN => svg_icon(&["M12 5v14", "M6 13l6 6 6-6"], "26", "ci"),
        S_RIGHT => svg_icon(&["M5 12h14", "M13 6l6 6-6 6"], "26", "ci"),
        S_RUN => el("svg:svg").cls("ci accent").attr("viewBox", "0 0 24 24").attr("width", "24").attr("height", "24").attr("aria-hidden", "true")
            .kid(el("svg:path").attr("d", "M7 4.5v15l12-7.5z").attr("fill", "currentColor")).into(),
        S_STEP => svg_icon(&["M6 5l9 7-9 7z", "M18 5v14"], "24", "ci"),
        S_CLEAR => svg_icon(&["M6 6h12v12H6z"], "22", "ci"),
        _ => txt(""),
    }
}

fn sup_label(id: usize) -> &'static str {
    match id {
        S_TITLE => "piii lab",
        S_PROG => "Program",
        S_PALETTE => "Palette",
        S_EXAMPLES => "Examples",
        S_RUN => "Run the program",
        S_STEP => "One step",
        S_CLEAR => "Clear the values",
        S_SAVE => "Save the layout",
        S_LOCK => "Lock: supers stay where they are",
        S_CHAIN => "Chain: a holder brings what it holds",
        S_RESET => "Reset: back to the start",
        S_LEFT => "Move left",
        S_UP => "Move up",
        S_DOWN => "Move down",
        S_RIGHT => "Move right",
        S_WELCOME => "Welcome",
        S_TOOLS => "Tools",
        S_CONNECT => "Connect",
        S_PRIVACY => "Privacy choices",
        S_EXPLORE => "Explore",
        S_LIBRARY => "Library",
        S_LAB => "Lab",
        S_PROJECT => "Project",
        _ => "",
    }
}

// ---------------------------------------------------------------------------------------------- the canvas

const WELCOME: &str = "ScholarPi: AI models assess your paper against open, science-aligned criteria and show what holds it \
back. Start in Assess a Manuscript; the rest of the lab is round it.";

/// A bubble's inside: its name on a tag, and one bar for each thing it holds.
fn bubble_inside(gi: usize, small: bool) -> Vec<N> {
    let g = &GROUPS[gi];
    let mut v: Vec<N> = Vec::new();
    let _ = small;
    v.push(el("span").cls("tag").text(g.title).into());
    let mut bars = el("span").cls("bars-tree");
    for (m, (name, _)) in g.members.iter().enumerate() {
        // a bar has no words on it, so it is not a button (owner, 2026-10-14: "I should not be able to click an object
        // if I can't read it"); the click goes to the super, whose window lists each member by name
        let _ = (m, name);
        bars = bars.kid(el("span").cls("bar-i").attr("aria-hidden", "true"));
    }
    v.push(bars.into());
    v
}

fn tools_inside() -> Vec<N> {
    let g = &GROUPS[0];
    let mut bars = el("span").cls("t-bars");
    for (m, (name, _)) in g.members.iter().enumerate() {
        let _ = (m, name);
        bars = bars.kid(el("span").cls("t-bar").attr("aria-hidden", "true"));
    }
    let mut v: Vec<N> = Vec::new();
    v.push(el("span").cls("t-in").text("Tools").into());
    v.push(el("span").cls("t-brace").into());
    v.push(bars.into());
    v
}

fn canvas_view(s: &State) -> N {
    scene_view(s, false)
}

/// The super an open page belongs to, lit on the floating canvas.
fn lit(s: &State) -> usize {
    match s.page {
        Page::Group(g) | Page::Item(g, _) => GROUPS.get(g as usize).map(|x| x.sup).unwrap_or(99),
        Page::Map => S_PROJECT,
        Page::Assess => S_TOOLS,
        Page::Explore | Page::Live | Page::Paper => S_EXPLORE,
        Page::Library => S_LIBRARY,
        Page::Account | Page::Confirm => S_CONNECT,
        Page::Home => 99,
    }
}

/// The π canvas floating on top of an open super (owner, 2026-10-14: "pi canvas is always floating on top"):
/// the same supers and wires, small, the open one lit; a click opens another, π goes back to the whole canvas.
fn float_size() -> (f64, f64) {
    let (vw, vh) = (host::vw(), host::vh());
    if vw < 700.0 {
        let w = vw - 24.0;
        (w, (w * 0.5).min(vh * 0.3))
    } else {
        let w = (vw * 0.26).clamp(260.0, 420.0);
        (w, w * 0.625)
    }
}

fn scene_view(s: &State, mini: bool) -> N {
    let g = if mini {
        let (w, h) = float_size();
        geo_box(s, w, h, false)
    } else {
        geo(s)
    };
    // wires, drawn under the supers; their ports over them
    let mut wires = el("svg:svg").cls("wires").attr("aria-hidden", "true").attr("width", "1").attr("height", "1").kid(
        el("svg:defs")
            .kid(arrowhead("ah-flow", "flow"))
            .kid(arrowhead("ah-ink", "ink")),
    );
    let nw = s.wires.len();
    for (i, (a, b, tip)) in s.wires.iter().enumerate() {
        let (a, b, tip) = (*a, *b, *tip);
        let (px, py) = leave_point(&g, a, tip);
        let (qx, qy) = tip_point(&g, b, tip);
        let d = match tip {
            Tip::In | Tip::Out => {
                // the ins of one tip share a bus, halfway from the furthest of them
                let reach = s.wires.iter().filter(|w| w.1 == b && w.2 == Tip::In).map(|w| leave_point(&g, w.0, Tip::In).0).fold(px, f64::max);
                let bus = (reach + qx) / 2.0;
                format!("M{:.1},{:.1} H{:.1} V{:.1} H{:.1}", px, py, bus, qy, qx)
            }
            _ => {
                // top-to-bottom wires curve (owner, 2026-10-14): leave the way the tip faces, arrive straight down
                // into a control or straight up into a gate
                let k = ((qy - py).abs() / 2.0).max(40.0);
                let (c1x, c1y) = (px + k, py);
                let c2y = if tip == Tip::Control { qy - k } else { qy + k };
                format!("M{:.1},{:.1} C{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}", px, py, c1x, c1y, qx, c2y, qx, qy)
            }
        };
        wires = draw_link(wires, mini, s.wire_sel == Some(i), i, &d, tip);
    }
    for (j, (a, ta, b, tb)) in s.passes.iter().enumerate() {
        let (a, ta, b, tb) = (*a, *ta, *b, *tb);
        let ((px, py), (qx, qy)) = (tip_point(&g, a, ta), tip_point(&g, b, tb));
        // out of its tip (up from a control, down from a gate), round by a lane left of both, and into the other tip
        // from outside, clear of everything between
        let away = |t: Tip| if t == Tip::Control { -40.0 } else { 40.0 };
        // curved too (owner, 2026-10-14): one sweep, leaving and arriving the way each tip faces
        let k = (((qy - py).abs() + (qx - px).abs()) / 80.0).max(1.5);
        let d = format!(
            "M{:.1},{:.1} C{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
            px,
            py,
            px,
            py + away(ta) * k,
            qx,
            qy + away(tb) * k,
            qx,
            qy
        );
        wires = draw_link(wires, mini, s.wire_sel == Some(nw + j), nw + j, &d, tb);
    }
    if let (false, Some(w)) = (mini, s.wiring.as_ref()) {
        let From::Bar(gi, _) = w.from;
        let pa = port(&g, GROUPS[gi].sup, Side::R);
        let d = match w.to {
            Some((id, side)) => curve(pa, Side::R, port(&g, id, side), side),
            None => curve(pa, Side::R, w.at, Side::L),
        };
        wires = wires.kid(el("svg:path").cls("w-temp").attr("d", d));
    }
    if let (false, Some((a, ta))) = (mini, s.held_port) {
        let (x1, y1) = if ta == Tip::Out { leave_point(&g, a, Tip::In) } else { tip_point(&g, a, ta) };
        wires = wires.kid(
            el("svg:line")
                .cls("w-temp")
                .attr("x1", format!("{:.1}", x1))
                .attr("y1", format!("{:.1}", y1))
                .attr("x2", format!("{:.1}", s.pend_at.0))
                .attr("y2", format!("{:.1}", s.pend_at.1)),
        );
    }
    let ports_svg = ports_view(s, &g, mini);

    let mut stage = el("div")
        .cls("stage")
        .style(format!("transform:translate({:.1}px,{:.1}px) scale({:.4})", g.ox, g.oy, g.k))
        .kid(wires);
    for id in 0..NSUP {
        if id == S_SITE || is_gone(id) {
            continue;
        }
        let (x, y, w, h) = g.rects[id];
        let mut sx = el("div")
            .cls("sx")
            .cls_if(mini && id == lit(s), "lit")
            .cls_if(!mini && s.wiring.as_ref().and_then(|w| w.to).map(|(t, _)| t == id).unwrap_or(false), "wire-target")
            .cls_if(!mini && locked(s, id) && id >= S_WELCOME, "locked")
            .attr("data-s", format!("{}", id))
            .attr_if(id != S_PROG && id != S_HOLD, "data-solid", "1")
            .style(format!("left:{:.1}px;top:{:.1}px;width:{:.1}px;height:{:.1}px", x, y, w, h));
        // only what can be read is clickable: not the small canvas floating on an open super (too small to read; the
        // rail and π get about), and not the holder, which has no words
        if !mini && id != S_HOLD {
            sx = sx.on("pdown", Msg::PDown(id, String::new()));
        }
        let label = sup_label(id);
        if !label.is_empty() && id != S_HOLD && !mini {
            sx = sx.attr("tabindex", "0").attr("role", "button").attr("aria-label", label).on("enter", Msg::Activate(id as u8));
        }
        sx = match id {
            S_TITLE => sx.cls("title").text("piii lab"),
            S_SAVE | S_LOCK | S_CHAIN | S_RESET | S_LEFT | S_UP | S_DOWN | S_RIGHT | S_RUN | S_STEP | S_CLEAR => sx.cls("ctl").attr("title", label).kid(control_icon(id, s)),
            S_PROG => sx.cls("prog").kids(prog_region_view(s)),
            S_PALETTE => sx.cls("palette").kids(palette_view()),
            S_EXAMPLES => sx.cls("examples").kids(examples_view()),
            S_WELCOME => sx.cls("words").text(WELCOME),
            S_TOOLS => sx.cls("tools").kids(tools_inside()),
            S_CONNECT => sx.cls("bubble small").kids(bubble_inside(4, true)),
            S_PRIVACY => sx.cls("pchip").text("Privacy choices"),
            S_HOLD => sx.cls("hold"),
            S_EXPLORE => sx.cls("bubble").kids(bubble_inside(1, false)),
            S_LIBRARY => sx.cls("bubble").kids(bubble_inside(2, false)),
            S_LAB => sx.cls("bubble big").kids(bubble_inside(3, false)),
            S_PROJECT => {
                let mut bars = el("span").cls("bars-tree");
                for _ in 0..3 {
                    bars = bars.kid(el("span").cls("bar-i"));
                }
                sx.cls("bubble").kid(el("span").cls("tag").text("Project")).kid(bars)
            }
            _ => sx,
        };
        if s.drag.as_ref().map(|d| d.id == id && d.moved).unwrap_or(false) {
            sx = sx.cls("moving");
        }
        stage = stage.kid(sx);
    }
    // the program's nodes, over its super
    for n in &s.prog.nodes {
        stage = stage.kid(unode_view(s, n, rect(&g, n.id), mini));
    }
    stage = stage.kid(ports_svg);
    if mini {
        let (w, h) = float_size();
        let at = match s.float_at {
            Some((l, t)) => format!("left:{:.0}px;top:{:.0}px;right:auto;bottom:auto;", l, t),
            None => String::new(),
        };
        return el("aside")
            .cls("scene floating")
            .cls_if(s.float_drag.as_ref().map(|d| d.moved).unwrap_or(false), "moving")
            .attr("aria-label", "The canvas: drag it anywhere")
            .attr("title", "Drag to move")
            .style(format!("{}width:{:.0}px;height:{:.0}px", at, w, h))
            .on("pdown", Msg::FloatDown(String::new()))
            .kid(el("div").cls("frame"))
            // no π badge on it (owner, 2026-10-14: "delete this logo"); π in the rail goes back to the canvas
            .kid(stage)
            .into();
    }
    el("div")
        .cls("scene")
        .cls_if(s.wiring.is_some() || s.held_port.is_some(), "wiring")
        .attr("title", "Double click: a sparkle. Double right click: a super.")
        .on("click", Msg::WireSel(255))
        .on("dbl", Msg::Dbl(false, String::new()))
        .on("dbl2", Msg::Dbl(true, String::new()))
        .kid(el("div").cls("frame"))
        .kid(stage)
        .kid_if(!s.note.is_empty(), || el("div").cls("wire-note").attr("role", "status").text(s.note.clone()).into())
        .kid(footer())
        .into()
}

/// Where a node's tip is: the page's edges at the page's sides, a super's at its extreme points.
fn tip_point(g: &Geo, id: usize, tip: Tip) -> (f64, f64) {
    match id {
        PAGE_IN => port(g, FRAME, Side::L),
        PAGE_OUT => port(g, FRAME, Side::R),
        _ => port(g, id, tip.side()),
    }
}

/// Where a wire into `into` leaves its node: a control's bottom into a control tip (it hangs from above); else the
/// node's out.
fn leave_point(g: &Geo, id: usize, into: Tip) -> (f64, f64) {
    // every super has a gate at its bottom now, so a control leaves by its out, on the right, like the rest
    let _ = into;
    tip_point(g, id, Tip::Out)
}

fn arrowhead(id: &'static str, cls: &'static str) -> N {
    el("svg:marker")
        .attr("id", id)
        .attr("viewBox", "0 0 10 10")
        .attr("refX", "10")
        .attr("refY", "5")
        .attr("markerUnits", "userSpaceOnUse")
        .attr("markerWidth", "16")
        .attr("markerHeight", "16")
        .attr("orient", "auto")
        .kid(el("svg:path").cls(format!("head {}", cls).as_str()).attr("d", "M0,0L10,5L0,10Z"))
        .into()
}

/// A wire or pass on the canvas: its kind by where it arrives (flow into in, control, gate), an arrowhead, and on
/// the whole canvas a wider unseen line over it to take the click.
fn draw_link(mut wires: crate::dom::El<Msg>, mini: bool, sel: bool, i: usize, d: &str, arrives: Tip) -> crate::dom::El<Msg> {
    let kind = match arrives {
        Tip::In | Tip::Out => "flow",
        Tip::Control => "control",
        Tip::Gate => "gate",
    };
    wires = wires.kid(
        el("svg:path")
            .cls("w")
            .cls(kind)
            .cls_if(sel, "sel")
            .attr("d", d)
            .attr("marker-end", if kind == "flow" { "url(#ah-flow)" } else { "url(#ah-ink)" }),
    );
    if !mini {
        wires = wires.kid(
            el("svg:path")
                .cls("w-hit")
                .attr("data-stop", "1")
                .attr("d", d)
                .attr("data-solid", "1")
                .on("click", Msg::WireSel(i as u8))
                .on("dblclick", Msg::WireDel(i as u8)),
        );
    }
    wires
}

/// A small sparkle (n = ½) of radius r: a port's mark.
fn sparkle(cx: f64, cy: f64, r: f64) -> String {
    let mut d = String::new();
    for i in 0..48 {
        let t = i as f64 / 48.0 * core::f64::consts::TAU;
        let (c, sn) = (libm_cos(t), libm_cos(t - core::f64::consts::FRAC_PI_2));
        // |x/r|^½ + |y/r|^½ = 1: x = r·sign(c)·c⁴
        let x = cx + r * c.signum() * (c * c) * (c * c);
        let y = cy + r * sn.signum() * (sn * sn) * (sn * sn);
        d.push_str(&format!("{}{:.2},{:.2}", if i == 0 { "M" } else { "L" }, x, y));
    }
    d.push('Z');
    d
}

/// cos without the standard library: a short series after folding into [-π, π].
fn libm_cos(x: f64) -> f64 {
    let tau = core::f64::consts::TAU;
    let mut x = x % tau;
    if x > core::f64::consts::PI {
        x -= tau;
    } else if x < -core::f64::consts::PI {
        x += tau;
    }
    let x2 = x * x;
    let (mut term, mut sum, mut k) = (1.0, 1.0, 0.0);
    while k < 20.0 {
        term *= -x2 / ((k + 1.0) * (k + 2.0));
        sum += term;
        k += 2.0;
    }
    sum
}

/// The ports to wire with, over everything: each node's out (filled), its tips (drawn round); while a wire is being
/// made, the tips it may join are marked.
fn ports_view(s: &State, g: &Geo, mini: bool) -> N {
    let r = if mini { 4.0 / g.k } else if g.w < 700.0 { 12.0 } else { 10.0 / g.k.max(0.6) };
    let mut svg = el("svg:svg").cls("ports-svg").attr("width", "1").attr("height", "1").attr("aria-hidden", if mini { "true" } else { "false" });
    let mut nodes: Vec<usize> = (0..NSUP).filter(|i| *i != S_PALETTE && *i != S_EXAMPLES && !is_gone(*i)).collect();
    nodes.push(PAGE_IN);
    nodes.push(PAGE_OUT);
    nodes.extend(s.prog.nodes.iter().map(|n| n.id));
    for id in nodes {
        for &tip in tips_of(id) {
            let (x, y) = if tip == Tip::Out { leave_point(g, id, if is_ctl_super(id) { Tip::Control } else { Tip::In }) } else { tip_point(g, id, tip) };
            let on = s.held_port == Some((id, tip));
            let target = !mini
                && match s.held_port {
                    Some((a, Tip::Out)) => tip != Tip::Out && can_connect(s, a, id, tip).is_ok(),
                    Some((a, ta)) => matches!(tip, Tip::Control | Tip::Gate) && can_relay(s, a, ta, id, tip).is_ok(),
                    None => false,
                };
            let mut gp = el("svg:g")
                .attr("data-p", format!("{}-{}", id, tip.name()))
                .attr("data-solid", "1")
                .cls("port")
                .cls(if tip == Tip::Out { "out" } else { "tip" })
                .cls_if(on, "on")
                .cls_if(target, "target")
                .kid(el("svg:path").cls("mark").attr("d", sparkle(x, y, r)));
            if !mini {
                gp = gp
                    .attr("data-stop", "1")
                    .on("click", Msg::Port(id, tip.code()))
                    .kid(el("svg:circle").cls("pad").attr("cx", format!("{:.1}", x)).attr("cy", format!("{:.1}", y)).attr("r", format!("{:.1}", r * 1.4)))
                    .kid(el("svg:title").text(format!("{}: {}", name_of(id), match tip {
                        Tip::Out => "out. Click, then click a tip",
                        Tip::In => "in",
                        Tip::Control => "control. Click, then another super's control or gate to pass it on",
                        Tip::Gate => "gate. Click, then another super's control or gate to pass it on",
                    })));
            }
            svg = svg.kid(gp);
        }
    }
    svg.into()
}

/// A click on a port: the first one held, the second one wired to it (or the reason it can not be).
fn port_click(id: usize, tip: Tip) {
    let s = st();
    let Some((a, ta)) = s.held_port else {
        match tip {
            Tip::In => note("In is where a wire arrives: click an out first, then this".to_string()),
            _ => {
                s.wire_sel = None;
                s.held_port = Some((id, tip));
                let g = geo(s);
                s.pend_at = tip_point(&g, id, tip);
                host::capture(true);
            }
        }
        return;
    };
    s.held_port = None;
    host::capture(false);
    if (a, ta) == (id, tip) {
        return;
    }
    if ta == Tip::Out && tip != Tip::Out {
        match can_connect(s, a, id, tip) {
            Err(e) => note(format!("Not wired: {}", e)),
            Ok(()) => {
                s.wires.push((a, id, tip));
                wires_save();
                let msg = match a {
                    S_RESET => {
                        if id < NSUP {
                            s.offs[id] = (0.0, 0.0);
                            for h in held_by(id) {
                                s.offs[*h] = (0.0, 0.0);
                            }
                            layout_save();
                        }
                        format!("Back where it started: {}", name_of(id))
                    }
                    S_LOCK => format!("Locked: {} stays where it is while this wire is there", name_of(id)),
                    S_CHAIN => format!("Chained: {} brings what it holds", name_of(id)),
                    S_SAVE => {
                        layout_save();
                        format!("Kept: {}", name_of(id))
                    }
                    _ => format!("Wired {}'s out to {}'s {}", name_of(a), name_of(id), tip.name()),
                };
                note(msg);
            }
        }
    } else if ta != Tip::Out && matches!(tip, Tip::Control | Tip::Gate) {
        match can_relay(s, a, ta, id, tip) {
            Err(e) => note(format!("Not passed on: {}", e)),
            Ok(()) => {
                s.passes.push((a, ta, id, tip));
                wires_save();
                note(format!("What arrives at {}'s {} passes on to {}'s {}", name_of(a), ta.name(), name_of(id), tip.name()));
            }
        }
    } else if tip == Tip::Out {
        s.held_port = Some((id, tip));
        let g = geo(s);
        s.pend_at = tip_point(&g, id, tip);
        host::capture(true);
    } else {
        note("Not wired: a wire leaves from an out and arrives at an in, a control or a gate".to_string());
    }
}

/// Takes away a wire (or a pass: they are numbered after the wires).
fn remove_link(i: usize) {
    let s = st();
    let nw = s.wires.len();
    if i < nw {
        s.wires.remove(i);
    } else if i - nw < s.passes.len() {
        s.passes.remove(i - nw);
    } else {
        return;
    }
    s.wire_sel = None;
    wires_save();
    note("Wire taken away".to_string());
}

// ---------------------------------------------------------------------------------------------- dragging and opening

pub struct Drag {
    id: usize,
    member: Option<u8>,
    sx: f64,
    sy: f64,
    k: f64,
    moved: bool,
    start: Vec<(usize, (f64, f64))>,
    pan0: (f64, f64),
}

/// Where a super has been moved to (its offset), or where a program node is in its super.
fn pos_of(s: &State, id: usize) -> (f64, f64) {
    if id < NSUP {
        return s.offs[id];
    }
    unode(s, id).map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0))
}

fn set_pos(s: &mut State, id: usize, p: (f64, f64)) {
    if id < NSUP {
        s.offs[id] = p;
    } else if let Some(n) = s.prog.nodes.iter_mut().find(|n| n.id == id) {
        n.x = p.0;
        n.y = p.1;
    }
}

fn begin_drag(id: usize, at: &str) {
    let s = st();
    let mut it = at.split(',');
    let x: f64 = it.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let y: f64 = it.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let g = geo(s);
    if s.held_port.is_some() {
        // a wire is being made: a press on a super lets it go
        s.held_port = None;
        host::capture(false);
        render();
        return;
    }
    let mut ids = alloc::vec![id];
    if chain_on(s, id) {
        ids.extend_from_slice(held_by(id));
    }
    if is_unode(id) {
        s.prog.sel = Some(id);
    }
    let start = ids.iter().map(|i| (*i, pos_of(s, *i))).collect();
    s.drag = Some(Drag { id, member: s.pend_member.take(), sx: x, sy: y, k: g.k, moved: false, start, pan0: s.pan });
    host::capture(true);
}

/// The super under a point (the topmost), the site when only the site is there, the page near its edge.
fn sup_at(g: &Geo, p: (f64, f64)) -> Option<usize> {
    for (id, (x, y, w, h)) in g.extra.iter().rev() {
        if p.0 >= *x && p.0 <= x + w && p.1 >= *y && p.1 <= y + h {
            return Some(*id);
        }
    }
    for id in (0..NSUP).rev() {
        if id == S_SITE || is_gone(id) {
            continue;
        }
        let (x, y, w, h) = g.rects[id];
        if p.0 >= x - 4.0 && p.0 <= x + w + 4.0 && p.1 >= y - 4.0 && p.1 <= y + h + 4.0 {
            return Some(id);
        }
    }
    let (x, y, w, h) = g.rects[S_SITE];
    if p.0 >= x && p.0 <= x + w && p.1 >= y && p.1 <= y + h {
        return Some(S_SITE);
    }
    let (vx, vy) = (p.0 * g.k + g.ox, p.1 * g.k + g.oy);
    if vx < 28.0 || vy < 28.0 || vx > g.w - 28.0 || vy > g.h - 28.0 {
        return Some(FRAME);
    }
    None
}

/// The side of the target a wire joins: the nearest side of the kind the wire carries (sync and condition join
/// sync and condition, input and output join input and output).
fn side_for(g: &Geo, id: usize, p: (f64, f64), ctl: Option<bool>) -> Side {
    let (cx, cy) = if id == FRAME {
        (port(g, FRAME, Side::T).0, port(g, FRAME, Side::L).1)
    } else {
        let (x, y, w, h) = rect(g, id);
        (x + w / 2.0, y + h / 2.0)
    };
    let (dx, dy) = (p.0 - cx, p.1 - cy);
    let horiz = if dx < 0.0 { Side::L } else { Side::R };
    let vert = if dy < 0.0 { Side::T } else { Side::B };
    match ctl {
        Some(true) => vert,
        Some(false) => horiz,
        None => {
            let (w, h) = if id == FRAME { (g.w / g.k, g.h / g.k) } else { (rect(g, id).2, rect(g, id).3) };
            if dx.abs() / w.max(1.0) > dy.abs() / h.max(1.0) { horiz } else { vert }
        }
    }
}

fn wiring_move(x: f64, y: f64) {
    let s = st();
    let g = geo(s);
    let p = ((x - g.ox) / g.k, (y - g.oy) / g.k);
    let Some(w) = s.wiring.as_mut() else { return };
    w.at = p;
    let From::Bar(gi, _) = w.from;
    let own = GROUPS[gi].sup;
    w.to = sup_at(&g, p).filter(|t| *t != own).map(|t| (t, side_for(&g, t, p, None)));
}

fn note(text: String) {
    let s = st();
    s.note = text;
    host::timer(2600, T_NOTE);
}

fn wiring_end() {
    let s = st();
    let Some(w) = s.wiring.take() else { return };
    let Some((b, sb)) = w.to else { return };
    let From::Bar(gi, m) = w.from;
    note(format!("Opened {}: wired to {}", GROUPS[gi].members[m].0, name_of(b)));
    let _ = sb;
    open_member(gi, m);
}

pub fn on_key(key: &str) -> bool {
    if key.starts_with('+') || key.starts_with('-') {
        return map_key(key);
    }
    let s = st();
    match key {
        "Escape" => {
            if s.modal.is_some() {
                s.modal = None;
                render();
                return true;
            }
            s.wiring = None;
            s.held_port = None;
            s.wire_sel = None;
            host::capture(false);
        }
        "Delete" | "Backspace" => {
            if let Some(i) = s.wire_sel {
                remove_link(i);
            } else if let Some(id) = s.prog.sel {
                prog_remove(id);
            }
        }
        _ => return false,
    }
    render();
    true
}

pub struct FloatDrag {
    sx: f64,
    sy: f64,
    l0: f64,
    t0: f64,
    moved: bool,
}

/// The floating canvas's top-left corner now, in px.
fn float_corner(s: &State) -> (f64, f64) {
    if let Some(p) = s.float_at {
        return p;
    }
    let (w, h) = float_size();
    let (vw, vh) = (host::vw(), host::vh());
    if vw < 700.0 { (vw - 12.0 - w, vh - 76.0 - h) } else { (vw - 18.0 - w, vh - 18.0 - h) }
}

fn float_clamp(p: (f64, f64)) -> (f64, f64) {
    let (w, h) = float_size();
    let (vw, vh) = (host::vw(), host::vh());
    (p.0.clamp(0.0, (vw - w).max(0.0)), p.1.clamp(0.0, (vh - h).max(0.0)))
}

fn float_begin(at: &str) {
    let s = st();
    let mut it = at.split(',');
    let x: f64 = it.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let y: f64 = it.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let (l0, t0) = float_corner(s);
    s.float_drag = Some(FloatDrag { sx: x, sy: y, l0, t0, moved: false });
    host::capture(true);
}

pub fn float_load() {
    let Some(v) = host::store_get("piii:float") else { return };
    let mut it = v.split(',');
    if let (Some(Ok(l)), Some(Ok(t))) = (it.next().map(|x| x.parse::<f64>()), it.next().map(|x| x.parse::<f64>())) {
        st().float_at = Some(float_clamp((l, t)));
    }
}

pub fn on_pointer(phase: u32, x: f64, y: f64) {
    let s = st();
    if let Some(d) = s.float_drag.as_mut() {
        match phase {
            0 => {
                if !d.moved && (x - d.sx).abs() + (y - d.sy).abs() > 4.0 {
                    d.moved = true;
                }
                if d.moved {
                    let p = (d.l0 + x - d.sx, d.t0 + y - d.sy);
                    s.float_at = Some(float_clamp(p));
                }
            }
            _ => {
                host::capture(false);
                s.float_drag = None;
                if let Some((l, t)) = s.float_at {
                    host::store_set("piii:float", &format!("{:.0},{:.0}", l, t));
                }
            }
        }
        render();
        return;
    }
    if s.held_port.is_some() {
        // a wire being made follows the pointer until a tip is clicked
        if phase == 0 {
            let g = geo(s);
            s.pend_at = ((x - g.ox) / g.k, (y - g.oy) / g.k);
            render();
        }
        return;
    }
    if s.wiring.is_some() {
        match phase {
            0 => wiring_move(x, y),
            1 => {
                host::capture(false);
                wiring_end();
            }
            _ => {
                host::capture(false);
                s.wiring = None;
            }
        }
        render();
        return;
    }
    let movable = s.drag.as_ref().map(|d| !locked(s, d.id) && d.id >= S_WELCOME).unwrap_or(false);
    let Some(d) = s.drag.as_mut() else {
        host::capture(false);
        return;
    };
    let (dx, dy) = ((x - d.sx) / d.k, (y - d.sy) / d.k);
    match phase {
        0 => {
            if !d.moved && (x - d.sx).abs() + (y - d.sy).abs() > 5.0 {
                d.moved = true;
                // a bar pulled out of its bubble is a wire: wherever it lands, what it stands for opens
                if let (Some(m), Some(gi)) = (d.member, group_of(d.id)) {
                    let at = ((x - d.sx) / d.k, (y - d.sy) / d.k);
                    s.drag = None;
                    s.wiring = Some(Wiring { from: From::Bar(gi, m as usize), at, to: None });
                    wiring_move(x, y);
                    render();
                    return;
                }
            }
            if d.moved && movable {
                for (i, (ox, oy)) in d.start.clone() {
                    set_pos(s, i, (ox + dx, oy + dy));
                }
                s.dirty = true;
            } else if d.moved && d.id == S_TITLE {
                // the title moves the view
                s.pan = (d.pan0.0 + dx, d.pan0.1 + dy);
            }
            render();
        }
        1 => {
            host::capture(false);
            let d = s.drag.take().unwrap();
            if !d.moved {
                activate(d.id, d.member);
            } else if is_unode(d.id) {
                prog_save();
            }
            render();
        }
        _ => {
            host::capture(false);
            s.drag = None;
            render();
        }
    }
}

pub fn on_resize() {
    render();
}

fn layout_save() {
    let s = st();
    let mut out = String::new();
    for (x, y) in s.offs.iter() {
        out.push_str(&format!("{:.1},{:.1};", x, y));
    }
    out.push_str(&format!("|{:.1},{:.1}|{}|{}", s.pan.0, s.pan.1, s.locked as u8, s.chained as u8));
    host::store_set("piii:layout2", &out);
    s.dirty = false;
}

fn layout_load() {
    let s = st();
    let Some(v) = host::store_get("piii:layout2") else { return };
    let parts: Vec<&str> = v.split('|').collect();
    if let Some(offs) = parts.first() {
        for (i, p) in offs.split(';').filter(|p| !p.is_empty()).enumerate().take(NSUP) {
            let mut xy = p.split(',');
            let x = xy.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let y = xy.next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
            s.offs[i] = (x, y);
        }
    }
    if let Some(p) = parts.get(1) {
        let mut xy = p.split(',');
        s.pan = (xy.next().and_then(|v| v.parse().ok()).unwrap_or(0.0), xy.next().and_then(|v| v.parse().ok()).unwrap_or(0.0));
    }
    s.locked = parts.get(2) == Some(&"1");
    s.chained = parts.get(3) != Some(&"0");
}

fn activate(id: usize, member: Option<u8>) {
    let s = st();
    match id {
        S_SAVE => layout_save(),
        S_LOCK => {
            s.locked = !s.locked;
            layout_save();
        }
        S_CHAIN => {
            s.chained = !s.chained;
            layout_save();
        }
        S_RESET => {
            s.offs = [(0.0, 0.0); NSUP];
            s.pan = (0.0, 0.0);
            s.locked = false;
            s.chained = true;
            s.open.clear();
            s.wires = START_WIRES.to_vec();
            s.passes = START_PASSES.to_vec();
            s.wire_sel = None;
            s.held_port = None;
            host::store_del("piii:layout2");
            host::store_del("piii:wires2");
            example_load(0);
        }
        S_RUN => prog_run(None),
        S_STEP => prog_run(Some(1)),
        S_CLEAR => prog_clear(),
        S_PROG | S_PALETTE | S_EXAMPLES => s.prog.sel = None,
        _ if is_unode(id) => s.prog.sel = Some(id),
        S_LEFT => s.pan.0 -= 60.0,
        S_RIGHT => s.pan.0 += 60.0,
        S_UP => s.pan.1 -= 60.0,
        S_DOWN => s.pan.1 += 60.0,
        S_TITLE => {
            if st().page != Page::Home {
                host::go("#/");
            }
        }
        S_WELCOME => host::go("#/assess"),
        S_PRIVACY => {
            s.draft = s.consent.unwrap_or([true, true, true]);
            s.consent_open = true;
        }
        S_PROJECT => host::go("#/map"),
        S_TOOLS | S_EXPLORE | S_LIBRARY | S_LAB | S_CONNECT => {
            let gi = group_of(id).unwrap_or(0);
            match member {
                Some(m) => open_member(gi, m as usize),
                None => host::go(&format!("#/g/{}", gi)),
            }
        }
        _ => {}
    }
}

fn open_member(gi: usize, m: usize) {
    let Some((_, t)) = GROUPS.get(gi).and_then(|g| g.members.get(m)) else { return };
    match t {
        Target::Hash(h) => host::go(h),
        Target::Url(u) => host::open(u),
        _ => host::go(&format!("#/i/{}/{}", gi, m)),
    }
}

// ---------------------------------------------------------------------------------------------- the open supers

/// A bubble, open: everything it holds, each a super of its own.
fn group_page(gi: usize) -> N {
    let g = &GROUPS[gi];
    el("div")
        .kid(head(g.title, "Everything in this super. Open one and it takes the canvas; the rest wait as icons."))
        .kid(el("div").cls("members").kids(g.members.iter().enumerate().map(|(m, (name, t))| -> N {
            let (href, note, ext) = match t {
                Target::Hash(h) => (h.to_string(), "", false),
                Target::Url(u) => (u.to_string(), "Opens in a new tab", true),
                Target::Words(_) => (format!("#/i/{}/{}", gi, m), "", false),
                Target::Data(_, d) => (format!("#/i/{}/{}", gi, m), *d, false),
                Target::Soon(_) => (format!("#/i/{}/{}", gi, m), "Being rewritten in Rust", false),
                Target::Tool(_) => (format!("#/i/{}/{}", gi, m), "", false),
            };
            let pien = st().t2.pien.pick.as_deref() == Some(*name);
            el("a")
                .cls("member sx-like")
                .cls_if(pien, "pien")
                .attr_if(pien, "title", "PiEN suggests this next")
                .attr("href", href)
                .attr_if(ext, "target", "_blank")
                .attr_if(ext, "rel", "noopener")
                .kid(el("span").cls("tag").text(*name))
                .kid_if(!note.is_empty(), || el("span").cls("m-note").text(note).into())
                .kid(ports())
                .into()
        })))
        .into()
}

/// The four ports of a super drawn by CSS (a window, an icon, a member).
fn ports() -> N {
    let st = |c: &'static str| -> N {
        el("svg:svg")
            .cls(c)
            .attr("viewBox", "-8 -8 16 16")
            .attr("width", "14")
            .attr("height", "14")
            .attr("aria-hidden", "true")
            .kid(el("svg:path").attr("d", star(0.0, 0.0, 7.0)))
            .into()
    };
    el("span").cls("pts").kid(st("pt l")).kid(st("pt r")).kid(st("pt t")).kid(st("pt b")).into()
}

fn item_page(s: &State, gi: usize, m: usize) -> N {
    let Some((name, t)) = GROUPS.get(gi).and_then(|g| g.members.get(m)) else { return empty("Nothing here.") };
    let body: N = match t {
        Target::Words(w) => el("div").cls("card").kid(el("p").style("font-size:16px").text(*w)).into(),
        Target::Soon(w) => el("div")
            .cls("card")
            .kid(el("p").style("font-size:16px").text(*w))
            .kid(el("p").cls("hint").style("margin-top:10px").text(
                "It ran as JavaScript in the page. The site is Rust now, and this tool is being rewritten; until then it is not available.",
            ))
            .into(),
        Target::Data(_, d) => el("div")
            .kid(el("p").cls("lede").style("margin-bottom:14px").text(*d))
            .kid(slot_view(&s.item, "Loading…", |v| data_view(v, 0)))
            .into(),
        Target::Tool(t) => tool_view(s, *t),
        _ => txt(""),
    };
    el("div").kid(head(name, GROUPS[gi].title)).kid(body).into()
}

/// What an endpoint answered, laid out: names and values; lists of records as rows.
fn data_view(v: &Value, depth: usize) -> N {
    match v {
        Value::Obj(kv) => {
            let mut dl = el("dl").cls("kv");
            let mut rest: Vec<N> = Vec::new();
            for (k, x) in kv {
                let label = k.replace('_', " ");
                match x {
                    Value::Obj(_) | Value::Arr(_) if depth < 2 => {
                        if !x.truthy() {
                            continue;
                        }
                        rest.push(el("div").style("margin-top:14px").kid(el("h3").style("margin-bottom:6px").text(label)).kid(data_view(x, depth + 1)).into());
                    }
                    Value::Null => {}
                    _ => {
                        dl = dl.kid(el("dt").text(label)).kid(el("dd").style("overflow-wrap:anywhere").text(scalar(x)));
                    }
                }
            }
            el("div").cls("card").kid(dl).kids(rest).into()
        }
        Value::Arr(a) => {
            if a.is_empty() {
                return el("p").cls("hint").text("Nothing yet.").into();
            }
            el("div")
                .cls("list")
                .kids(a.iter().take(60).map(|r| -> N {
                    match r {
                        Value::Obj(kv) => {
                            let title = r.any(&["title", "name", "author", "label", "topic"]).text();
                            let rest: Vec<String> = kv
                                .iter()
                                .filter(|(k, x)| !matches!(x, Value::Obj(_) | Value::Arr(_) | Value::Null) && !["title", "name", "author", "label", "topic"].contains(&k.as_str()))
                                .take(4)
                                .map(|(k, x)| format!("{} {}", k.replace('_', " "), scalar(x)))
                                .collect();
                            el("div")
                                .cls("item")
                                .kid(el("div").cls("grow").kid(el("div").cls("t").text(if title.is_empty() { "—".to_string() } else { title })).kid(el("div").cls("s").text(rest.join(" · "))))
                                .into()
                        }
                        x => el("div").cls("item").kid(el("div").cls("t").text(scalar(x))).into(),
                    }
                }))
                .into()
        }
        x => el("p").text(scalar(x)).into(),
    }
}

fn scalar(v: &Value) -> String {
    match v {
        Value::Num(n) => {
            if *n == (*n as i64) as f64 { num(*n, 0) } else { num(*n, 3) }
        }
        Value::Str(s) => {
            if s.len() > 140 { format!("{}…", s.chars().take(140).collect::<String>()) } else { s.clone() }
        }
        x => x.text(),
    }
}

// ---------------------------------------------------------------------------------------------- the project map

struct MapNode {
    id: &'static str,
    label: &'static str,
    kind: &'static str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    opens: &'static str,
    note: &'static str,
}

const fn mn(id: &'static str, label: &'static str, kind: &'static str, x: f64, y: f64, w: f64, h: f64, opens: &'static str, note: &'static str) -> MapNode {
    MapNode { id, label, kind, x, y, w, h, opens, note }
}

// pitechlab.sasm, `map`: the whole project as one super's inside
const MAP: [MapNode; 24] = [
    mn("prompt", "Prompt", "in", 20.0, 120.0, 150.0, 64.0, "", ""),
    mn("superai", "Super AI", "ai", 210.0, 128.0, 96.0, 48.0, "", ""),
    mn("sim", "siM", "lab", 350.0, 20.0, 74.0, 40.0, "#/i/2/4", ""),
    mn("rib", "riB", "lab", 350.0, 76.0, 74.0, 40.0, "#/i/2/4", ""),
    mn("pid", "piD", "lab", 350.0, 132.0, 74.0, 40.0, "#/i/1/6", ""),
    mn("pien", "PiEn", "lab", 350.0, 188.0, 74.0, 40.0, "#/i/2/4", ""),
    mn("llama", "Llama", "out", 470.0, 8.0, 74.0, 40.0, "", ""),
    mn("mistral", "Mistral", "out", 470.0, 58.0, 74.0, 40.0, "", ""),
    mn("qwen", "Qwen", "out", 470.0, 108.0, 74.0, 40.0, "", ""),
    mn("gemini", "Gemini", "out", 470.0, 158.0, 74.0, 40.0, "", ""),
    mn("deepseek", "DeepSeek", "out", 470.0, 208.0, 74.0, 40.0, "", ""),
    mn("judge", "Judge", "judge", 600.0, 108.0, 74.0, 40.0, "", ""),
    mn("answer", "Answer", "sink", 720.0, 70.0, 160.0, 118.0, "", ""),
    mn("live", "Live", "site", 906.0, 20.0, 74.0, 40.0, "#/live", ""),
    mn("traffic", "Traffic", "site", 906.0, 200.0, 74.0, 40.0, "#/i/1/5", ""),
    mn("manuscript", "Manuscript", "in", 20.0, 330.0, 92.0, 46.0, "#/assess", ""),
    mn("extract", "Extract", "step", 132.0, 333.0, 88.0, 40.0, "", "PDF → text"),
    mn("integrity", "Integrity", "step", 238.0, 333.0, 88.0, 40.0, "", "MDAR · RRID"),
    mn("jurors", "Jurors", "step", 344.0, 333.0, 88.0, 40.0, "", ""),
    mn("verdict", "Verdict", "judge", 450.0, 333.0, 88.0, 40.0, "", "the judge"),
    mn("pix", "piX", "step", 556.0, 333.0, 88.0, 40.0, "/ScholarPi_Whitepaper.pdf", ""),
    mn("piq", "piQ", "step", 662.0, 333.0, 88.0, 40.0, "#/library", ""),
    mn("ledger", "Ledger", "step", 768.0, 333.0, 88.0, 40.0, "#/i/1/3", ""),
    mn("explorer", "Explorer", "sink", 874.0, 333.0, 106.0, 40.0, "#/explore", ""),
];

const MAP_WIRES: [(&str, &str); 28] = [
    ("prompt", "superai"),
    ("superai", "sim"),
    ("superai", "rib"),
    ("superai", "pid"),
    ("superai", "pien"),
    ("superai", "llama"),
    ("superai", "mistral"),
    ("superai", "qwen"),
    ("superai", "gemini"),
    ("superai", "deepseek"),
    ("llama", "judge"),
    ("mistral", "judge"),
    ("qwen", "judge"),
    ("gemini", "judge"),
    ("deepseek", "judge"),
    ("judge", "answer"),
    ("sim", "answer"),
    ("manuscript", "extract"),
    ("extract", "integrity"),
    ("integrity", "jurors"),
    ("jurors", "verdict"),
    ("verdict", "pix"),
    ("pix", "piq"),
    ("piq", "ledger"),
    ("ledger", "explorer"),
    ("pix", "pid"),
    ("pix", "rib"),
    ("ledger", "live"),
];

const MAP_SECTIONS: [(&str, f64, f64, f64, f64); 3] =
    [("one prompt, every engine", 14.0, 0.0, 972.0, 270.0), ("the assessment", 14.0, 300.0, 972.0, 110.0), ("the site", 14.0, 470.0, 972.0, 100.0)];

fn map_page() -> N {
    let find = |id: &str| MAP.iter().find(|n| n.id == id);
    let mut svg = el("svg:svg").cls("map").attr("viewBox", "0 -24 1000 604").attr("role", "img").attr("aria-label", "The project map");
    for (label, x, y, w, h) in MAP_SECTIONS {
        svg = svg
            .kid(el("svg:rect").cls("m-sec").attr("x", format!("{}", x)).attr("y", format!("{}", y)).attr("width", format!("{}", w)).attr("height", format!("{}", h)))
            .kid(el("svg:text").cls("m-sec-t").attr("x", format!("{}", x + 8.0)).attr("y", format!("{}", y - 6.0)).text(label));
    }
    for (a, b) in MAP_WIRES {
        let (Some(na), Some(nb)) = (find(a), find(b)) else { continue };
        // into the node's left edge, or its top/bottom when it sits above or below
        let pa = (na.x + na.w, na.y + na.h / 2.0);
        let (pb, sb) = if nb.x > na.x + na.w {
            ((nb.x, nb.y + nb.h / 2.0), Side::L)
        } else if nb.y < na.y {
            ((nb.x + nb.w / 2.0, nb.y + nb.h), Side::B)
        } else {
            ((nb.x + nb.w / 2.0, nb.y), Side::T)
        };
        let data = sb == Side::L;
        svg = svg.kid(el("svg:path").cls(if data { "w data" } else { "w" }).attr("d", if data { curve(pa, Side::R, pb, Side::L) } else { curve(pa, Side::R, pb, sb) }));
    }
    let mut stars = String::new();
    for n in MAP.iter() {
        for (x, y) in [(n.x, n.y + n.h / 2.0), (n.x + n.w, n.y + n.h / 2.0), (n.x + n.w / 2.0, n.y), (n.x + n.w / 2.0, n.y + n.h)] {
            stars.push_str(&star(x, y, 4.5));
        }
    }
    for n in MAP.iter() {
        let mut gnode = el("svg:g").cls("m-node").cls(n.kind);
        gnode = gnode
            .kid(el("svg:rect").attr("x", format!("{}", n.x)).attr("y", format!("{}", n.y)).attr("width", format!("{}", n.w)).attr("height", format!("{}", n.h)))
            .kid(el("svg:text").attr("x", format!("{}", n.x + n.w / 2.0)).attr("y", format!("{}", n.y + n.h / 2.0 + if n.note.is_empty() { 5.0 } else { -2.0 })).attr("text-anchor", "middle").text(n.label));
        if !n.note.is_empty() {
            gnode = gnode.kid(el("svg:text").cls("m-note").attr("x", format!("{}", n.x + n.w / 2.0)).attr("y", format!("{}", n.y + n.h / 2.0 + 13.0)).attr("text-anchor", "middle").text(n.note));
        }
        if !n.opens.is_empty() {
            let a = el("svg:a").attr("href", n.opens).cls("m-link");
            let a = if n.opens.starts_with('/') { a.attr("target", "_blank") } else { a };
            svg = svg.kid(a.kid(gnode));
        } else {
            svg = svg.kid(gnode);
        }
    }
    svg = svg.kid(el("svg:path").cls("ports").attr("d", stars));
    // the site row: the supers of the canvas, as a reminder of where the map sits
    let mut x = 20.0;
    for g in GROUPS.iter() {
        let w = 150.0;
        svg = svg.kid(
            el("svg:a").attr("href", format!("#/g/{}", group_of(g.sup).unwrap_or(0))).cls("m-link").kid(
                el("svg:g")
                    .cls("m-node site")
                    .kid(el("svg:rect").attr("x", format!("{}", x)).attr("y", "500").attr("width", format!("{}", w)).attr("height", "40"))
                    .kid(el("svg:text").attr("x", format!("{}", x + w / 2.0)).attr("y", "525").attr("text-anchor", "middle").text(g.title)),
            ),
        );
        x += w + 42.0;
    }
    el("div")
        .kid(head("Project", "The whole project as one super's inside: one prompt to every engine, the assessment from manuscript to ledger, and the site."))
        .kid(el("div").cls("map-wrap").kid(svg))
        .into()
}
