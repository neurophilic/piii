// Every super's own page is a canvas like the home one (owner, 2026-10-14: "merge this view and wiring and
// interactions … make it universal throughout the app"). Its members are supers with four tips; wires run out of an
// out into an in (flow), a control or a gate, drawn and styled as on the canvas; supers drag anywhere; a double
// click on the bare board makes a sparkle and a double right click a super of your own; a click on a wire selects
// it and Delete removes it. Each board keeps its own layout and wiring in this browser. Included into app.rs.

#[derive(Default, Clone)]
pub struct Board {
    loaded: bool,
    pos: Vec<(f64, f64)>,
    /// sparkles (label empty) and supers of your own, after the members: kind, x, y, label
    extra: Vec<(u8, f64, f64, String)>,
    links: Vec<(usize, usize, u8)>,
    /// growth of each node by the middle button
    sizes: Vec<(f64, f64)>,
    sel: Option<usize>,
    /// the super a single click selected
    picked: Option<usize>,
    /// the nodes unlocked by selecting them and clicking the lock; every other one stays put, at its size
    free: Vec<usize>,
    held: Option<usize>,
    /// where the pointer is while a wire is being made, for the wire that follows it
    at: Option<(f64, f64)>,
    note: String,
}

pub struct BoardDrag {
    gi: usize,
    node: usize,
    sx: f64,
    sy: f64,
    p0: (f64, f64),
    moved: bool,
    how: u8,
    s0: (f64, f64),
    /// unlocked (or an icon) when pressed
    free: bool,
}

const BW: f64 = 220.0;
const BH: f64 = 104.0;
const BGAP_X: f64 = 44.0;
const BGAP_Y: f64 = 48.0;
const BCOLS: usize = 4;
const SPARK: f64 = 44.0;
const USW: f64 = 190.0;
const USH: f64 = 64.0;

fn board_key(gi: usize) -> String {
    format!("piii:board:{}", gi)
}

fn board(gi: usize) -> &'static mut Board {
    let s = st();
    while s.boards.len() <= gi {
        s.boards.push(Board::default());
    }
    let b = &mut s.boards[gi];
    if !b.loaded {
        board_load(gi, b);
    }
    &mut s.boards[gi]
}

fn board_default(n: usize) -> (Vec<(f64, f64)>, Vec<(usize, usize, u8)>) {
    // as many columns as the window holds (the rail and margins aside), at most four
    // the window's inner width: the page beside the rail (70% of it when opened over the canvas), at most 1000
    let page = host::vw() - 100.0;
    let avail = (if st().peek { page * 0.7 } else { page }).min(1000.0) - 48.0 - 24.0;
    let cols = (((avail + BGAP_X) / (BW + BGAP_X)) as usize).clamp(1, BCOLS);
    let pos = (0..n).map(|i| (24.0 + (i % cols) as f64 * (BW + BGAP_X), 24.0 + (i / cols) as f64 * (BH + BGAP_Y))).collect();
    // reading order along each row, one into the next
    let links = (1..n).filter(|i| i % cols != 0).map(|i| (i - 1, i, 0)).collect();
    (pos, links)
}

fn board_load(gi: usize, b: &mut Board) {
    b.loaded = true;
    let n = GROUPS.get(gi).map(|g| g.members.len()).unwrap_or(0);
    let (pos, links) = board_default(n);
    b.pos = pos;
    b.links = links;
    b.extra.clear();
    let Some(v) = host::store_get(&board_key(gi)).and_then(|t| json::parse(&t)) else { return };
    for (i, p) in v.get("pos").arr().iter().enumerate() {
        if i < n {
            b.pos[i] = (p.arr().first().map(|x| x.f()).unwrap_or(0.0), p.arr().get(1).map(|x| x.f()).unwrap_or(0.0));
        }
    }
    b.extra = v.get("extra").arr().iter().map(|e| {
        let a = e.arr();
        (a.first().map(|x| x.f() as u8).unwrap_or(0), a.get(1).map(|x| x.f()).unwrap_or(0.0), a.get(2).map(|x| x.f()).unwrap_or(0.0), a.get(3).map(|x| x.text()).unwrap_or_default())
    }).collect();
    b.free = v.get("free").arr().iter().map(|x| x.f() as usize).collect();
    b.sizes = v.get("sizes").arr().iter().map(|p| (p.arr().first().map(|x| x.f()).unwrap_or(0.0), p.arr().get(1).map(|x| x.f()).unwrap_or(0.0))).collect();
    if !v.get("links").is_null() {
        let total = n + b.extra.len();
        b.links = v.get("links").arr().iter().filter_map(|l| {
            let a = l.arr();
            let (x, y, t) = (a.first()?.f() as usize, a.get(1)?.f() as usize, a.get(2)?.f() as u8);
            (x < total && y < total && x != y).then_some((x, y, t))
        }).collect();
    }
}

fn board_save(gi: usize) {
    let b = board(gi);
    let pos: Vec<String> = b.pos.iter().map(|(x, y)| format!("[{:.0},{:.0}]", x, y)).collect();
    let extra: Vec<String> = b.extra.iter().map(|(k, x, y, l)| format!("[{},{:.0},{:.0},{}]", k, x, y, json::quote(l))).collect();
    let links: Vec<String> = b.links.iter().map(|(a, c, t)| format!("[{},{},{}]", a, c, t)).collect();
    let sizes: Vec<String> = b.sizes.iter().map(|(w, h)| format!("[{:.0},{:.0}]", w, h)).collect();
    let free: Vec<String> = b.free.iter().map(|i| i.to_string()).collect();
    host::store_set(&board_key(gi), &format!("{{\"pos\":[{}],\"extra\":[{}],\"links\":[{}],\"sizes\":[{}],\"free\":[{}]}}", pos.join(","), extra.join(","), links.join(","), sizes.join(","), free.join(",")));
}

/// Unlocked by the lock, or an icon (under 10% of the window), which is never locked.
fn bnode_free(b: &Board, i: usize) -> bool {
    let (_, _, w, h) = bnode(b, i);
    b.free.contains(&i) || icon_area(w, h)
}

/// A node's box on the board: a member, a sparkle or a super of your own.
fn bnode(b: &Board, i: usize) -> (f64, f64, f64, f64) {
    let (gw, gh) = b.sizes.get(i).copied().unwrap_or((0.0, 0.0));
    let (x, y, w, h) = if i < b.pos.len() {
        let (x, y) = b.pos[i];
        (x, y, BW, BH)
    } else {
        match b.extra.get(i - b.pos.len()) {
            Some((0, x, y, _)) => (*x, *y, SPARK, SPARK),
            Some((_, x, y, _)) => (*x, *y, USW, USH),
            None => return (0.0, 0.0, 0.0, 0.0),
        }
    };
    // no limit either way, as on the main canvas; under 10% of the window it is an icon
    (x, y, (w + gw).max(MIN_SUPER), (h + gh).max(MIN_SUPER))
}

fn btip(b: &Board, i: usize, t: u8) -> (f64, f64) {
    let (x, y, w, h) = bnode(b, i);
    match t {
        0 => (x, y + h / 2.0),
        1 => (x + w, y + h / 2.0),
        2 => (x + w / 2.0, y),
        _ => (x + w / 2.0, y + h),
    }
}

fn side_of(t: u8) -> Side {
    match t {
        0 => Side::L,
        1 => Side::R,
        2 => Side::T,
        _ => Side::B,
    }
}

/// Does `to` already feed, through wires, into `from`? (A loop of supers says nothing.)
fn board_reaches(b: &Board, from: usize, to: usize) -> bool {
    let mut seen = alloc::vec![false; b.pos.len() + b.extra.len() + 1];
    let mut stack = alloc::vec![from];
    while let Some(n) = stack.pop() {
        if n == to {
            return true;
        }
        if n < seen.len() && !seen[n] {
            seen[n] = true;
            stack.extend(b.links.iter().filter(|l| l.0 == n).map(|l| l.1));
        }
    }
    false
}

fn board_open(gi: usize, m: usize) {
    let Some((_, t)) = GROUPS.get(gi).and_then(|g| g.members.get(m)) else { return };
    match t {
        Target::Hash(h) => host::go(h),
        Target::Url(u) => host::open(u),
        _ => host::go(&format!("#/i/{}/{}", gi, m)),
    }
}

/// The pointer, while a board super is held. True: it was the board's.
/// The open window's size as the page lays it out (before any resizing).
fn win_default() -> (f64, f64) {
    let (vw, vh) = (host::vw(), host::vh());
    if st().peek { ((vw - 92.0) * 0.7, vh * 0.7) } else { (vw - 92.0, vh - 24.0) }
}

fn board_pointer(phase: u32, x: f64, y: f64) -> bool {
    let s = st();
    if let Some((sx, sy, w0, h0, mode)) = s.win_drag {
        if phase == 0 {
            // no larger than the screen beside the rail holds, so every edge stays in view
            let (mw, mh) = ((host::vw() - 100.0).max(360.0), (host::vh() - 24.0).max(240.0));
            let w = if mode == 2 { w0.min(mw) } else { (w0 + x - sx).clamp(360.0, mw) };
            let h = if mode == 1 { h0.min(mh) } else { (h0 + y - sy).clamp(240.0, mh) };
            s.win_size = Some((w, h));
        } else {
            host::capture(false);
            s.win_drag = None;
            if let Some((w, h)) = s.win_size {
                host::store_set("piii:win", &format!("{:.0},{:.0}", w, h));
            }
        }
        render();
        return true;
    }
    let Some(d) = s.board_drag.as_mut() else { return false };
    let (gi, node) = (d.gi, d.node);
    match phase {
        0 => {
            if !d.moved && (x - d.sx).abs() + (y - d.sy).abs() > 4.0 {
                d.moved = true;
            }
            // as on every canvas: a super stays put, and keeps its size, until the lock frees them; then a drag moves
            // it and the middle button resizes it
            let open = d.free;
            if d.moved && d.how == 2 && open {
                let (gx, gy) = (d.s0.0 + x - d.sx, d.s0.1 + y - d.sy);
                let b = board(gi);
                while b.sizes.len() <= node {
                    b.sizes.push((0.0, 0.0));
                }
                b.sizes[node] = (gx, gy);
            } else if d.moved && open {
                let d = st().board_drag.as_mut().unwrap();
                let p = ((d.p0.0 + x - d.sx).max(0.0), (d.p0.1 + y - d.sy).max(0.0));
                let b = board(gi);
                if node < b.pos.len() {
                    b.pos[node] = p;
                } else if let Some(e) = b.extra.get_mut(node - b.pos.len()) {
                    e.1 = p.0;
                    e.2 = p.1;
                }
            }
        }
        _ => {
            host::capture(false);
            let moved = d.moved;
            let how = d.how;
            s.board_drag = None;
            if moved {
                board_save(gi);
            } else if phase == 1 && how != 2 && node >= board(gi).pos.len() {
                board(gi).picked = Some(node);
                let s = st();
                s.float_pick = false;
                s.win_pick = false;
            } else if phase == 1 && how != 2 && node < board(gi).pos.len() {
                // a double click opens the member, as everywhere
                let now = host::now();
                let s = st();
                if s.last_tap.0 == 10_000 + node && now - s.last_tap.2 < 450.0 {
                    s.last_tap = (usize::MAX, None, 0.0);
                    s.peek_next = s.peek;
                    board_open(gi, node);
                } else {
                    s.last_tap = (10_000 + node, None, now);
                    s.float_pick = false;
                    s.win_pick = false;
                    board(gi).picked = Some(node);
                }
            }
        }
    }
    render();
    true
}

fn board_update(a: u16, v: String) {
    if a == 50 {
        canvas_zoom(&v);
        return;
    }
    // the open window: its corner resizes it, and so does the middle button on its head
    if a == 66 {
        let s = st();
        reset_hold_begin();
        s.reset_cap = true;
        host::capture(true);
        return;
    }
    if a == 65 {
        // a click on the window's head selects it, for the lock
        let s = st();
        s.win_pick = true;
        s.float_pick = false;
        s.sup_pick = None;
        if let Page::Group(g) = s.page {
            board(g as usize).picked = None;
        }
        return;
    }
    if a == 60 || a == 62 || a == 63 || a == 64 {
        if !st().win_free {
            // the lock is on: the window keeps its size
            return;
        }
        let n: Vec<f64> = v.split(',').filter_map(|x| x.parse().ok()).collect();
        if a == 62 && n.get(2).copied() != Some(2.0) {
            return;
        }
        let s = st();
        let (w, h) = s.win_size.unwrap_or_else(|| win_default());
        let mode = match a { 63 => 1, 64 => 2, _ => 0 };
        s.win_drag = Some((n.first().copied().unwrap_or(0.0), n.get(1).copied().unwrap_or(0.0), w, h, mode));
        host::capture(true);
        return;
    }
    if a == 71 {
        if let Page::Group(g) = st().page {
            let n: Vec<f64> = v.split(',').filter_map(|x| x.parse().ok()).collect();
            let b = board(g as usize);
            if b.held.is_some() && n.len() >= 2 {
                b.at = Some((n[0], n[1]));
            }
        }
        return;
    }
    if a == 70 {
        let s = st();
        if let Ok(i) = v.parse::<usize>() {
            if i < s.results.len() {
                s.results.remove(i);
                results_save();
            }
        }
        return;
    }
    if a == 61 {
        if !st().win_free {
            return;
        }
        st().win_size = None;
        host::store_del("piii:win");
        return;
    }
    let gi = match st().page {
        Page::Group(g) => g as usize,
        _ => return,
    };
    let nums: Vec<f64> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    match a {
        // 2000 + k: the text in the k-th super of your own
        n if n >= 2000 => {
            let b = board(gi);
            if let Some(e) = b.extra.get_mut((n - 2000) as usize) {
                e.3 = v.chars().take(60).collect();
            }
            board_save(gi);
        }
        8 => board_open(gi, nums.first().copied().unwrap_or(0.0) as usize),
        // 1000 + node: pressed on a super (client x, y)
        n if n >= 1000 => {
            let node = (n - 1000) as usize;
            let b = board(gi);
            if b.held.is_some() {
                b.held = None;
                return;
            }
            let (x, y, _, _) = bnode(b, node);
            let s0 = b.sizes.get(node).copied().unwrap_or((0.0, 0.0));
            let how = nums.get(2).copied().unwrap_or(0.0) as u8;
            let free = bnode_free(b, node);
            st().board_drag = Some(BoardDrag { gi, node, sx: nums.first().copied().unwrap_or(0.0), sy: nums.get(1).copied().unwrap_or(0.0), p0: (x, y), moved: false, how, s0, free });
            host::capture(true);
        }
        // a tip: node,tip
        2 => {
            let (node, tip) = (nums.first().copied().unwrap_or(0.0) as usize, nums.get(1).copied().unwrap_or(0.0) as u8);
            let b = board(gi);
            b.sel = None;
            match b.held {
                None if tip == 1 => {
                    b.held = Some(node);
                    b.at = None;
                    b.note = "Now click a tip of another super: in, control or gate.".into();
                }
                None => b.note = "A wire starts at an out (the filled tip on the right).".into(),
                Some(from) => {
                    b.held = None;
                    if tip == 1 {
                        b.held = Some(node);
                    b.at = None;
                    } else if from == node {
                        b.note = "A super does not wire into itself.".into();
                    } else if board_reaches(b, node, from) {
                        b.note = "That would make a loop.".into();
                    } else if b.links.iter().any(|l| l.0 == from && l.1 == node && l.2 == tip) {
                        b.note = "Already wired.".into();
                    } else {
                        b.links.push((from, node, tip));
                        b.note.clear();
                        board_save(gi);
                    }
                }
            }
        }
        3 => {
            let b = board(gi);
            let i = nums.first().copied().unwrap_or(-1.0);
            b.sel = if i < 0.0 { None } else { Some(i as usize) };
            if i < 0.0 && host::now() - st().last_tap.2 > 120.0 {
                b.picked = None;
            }
            b.held = None;
        }
        // a double click on the bare board: client x, y, then x, y within it
        4 | 5 => {
            let (x, y) = (nums.get(2).copied().unwrap_or(40.0), nums.get(3).copied().unwrap_or(40.0));
            let b = board(gi);
            if a == 4 {
                b.extra.push((0, (x - SPARK / 2.0).max(0.0), (y - SPARK / 2.0).max(0.0), String::new()));
            } else {
                b.extra.push((1, (x - USW / 2.0).max(0.0), (y - USH / 2.0).max(0.0), String::new()));
            }
            board_save(gi);
        }
        7 => board_reset(gi),
        _ => {}
    }
}

fn board_reset(gi: usize) {
    host::store_del(&board_key(gi));
    let b = board(gi);
    b.loaded = false;
    b.sizes.clear();
    board_load(gi, b);
}

/// Delete or Escape on a board. True: the board used it.
fn board_key_press(key: &str) -> bool {
    let gi = match st().page {
        Page::Group(g) => g as usize,
        _ => return false,
    };
    let b = board(gi);
    match key {
        "Escape" if b.held.is_some() || b.sel.is_some() => {
            b.held = None;
            b.sel = None;
        }
        "Delete" | "Backspace" => {
            if let Some(i) = b.sel.take() {
                if i < b.links.len() {
                    b.links.remove(i);
                }
            } else {
                return false;
            }
            board_save(gi);
        }
        _ => return false,
    }
    render();
    true
}

fn group_board(gi: usize) -> N {
    let g = &GROUPS[gi];
    let b = board(gi).clone();
    let total = b.pos.len() + b.extra.len();
    let (mut w, mut h) = (0.0f64, 0.0f64);
    for i in 0..total {
        let (x, y, bw, bh) = bnode(&b, i);
        w = w.max(x + bw + 40.0);
        h = h.max(y + bh + 40.0);
    }
    let pien_pick = st().t2.pien.pick.clone();
    // wires
    let mut wires = el("svg:svg").cls("wires").attr("width", format!("{:.0}", w)).attr("height", format!("{:.0}", h)).kid(el("svg:defs").kid(arrowhead("bah-flow", "flow")).kid(arrowhead("bah-ink", "ink")));
    for (i, (from, to, tip)) in b.links.iter().enumerate() {
        let a = btip(&b, *from, 1);
        let z = btip(&b, *to, *tip);
        let d = curve(a, Side::R, z, side_of(*tip));
        let kind = match tip { 0 => "flow", 2 => "control", _ => "gate" };
        wires = wires
            .kid(el("svg:path").cls("w").cls(kind).cls_if(b.sel == Some(i), "sel").attr("d", d.clone()).attr("marker-end", if kind == "flow" { "url(#bah-flow)" } else { "url(#bah-ink)" }))
            .kid(el("svg:path").cls("w-hit").attr("data-stop", "1").attr("data-solid", "1").attr("d", d).on("click", Msg::T(Tool::Grid, 3, format!("{}", i))).kid(el("svg:title").text("A wire: click to select it, then Delete to remove it")));
    }
    // the wire being made follows the pointer
    if let (Some(h), Some(at)) = (b.held, b.at) {
        let a = btip(&b, h, 1);
        wires = wires.kid(el("svg:path").cls("w-temp").attr("d", curve(a, Side::R, at, Side::L)));
    }
    // tips
    let mut tips = el("svg:svg").cls("ports-svg board-ports").cls_if(b.held.is_some(), "holding").attr("width", format!("{:.0}", w)).attr("height", format!("{:.0}", h));
    for i in 0..total {
        for t in 0..4u8 {
            let (x, y) = btip(&b, i, t);
            let held = b.held == Some(i) && t == 1;
            let target = b.held.is_some() && b.held != Some(i) && t != 1;
            let wired = b.links.iter().any(|l| (l.0 == i && t == 1) || (l.1 == i && l.2 == t));
            tips = tips.kid(
                el("svg:g")
                    .cls("port")
                    .cls(if t == 1 { "out" } else { "tip" })
                    .cls_if(held, "on")
                    .cls_if(wired, "wired")
                    .cls_if(target, "target")
                    .attr("data-stop", "1")
                    .attr("data-solid", "1")
                    .on("click", Msg::T(Tool::Grid, 2, format!("{},{}", i, t)))
                    .kid(el("svg:circle").cls("pad").attr("cx", format!("{:.1}", x)).attr("cy", format!("{:.1}", y)).attr("r", "14"))
                    .kid(el("svg:path").cls("mark").attr("d", sparkle(x, y, 10.0)))
                    .kid(el("svg:title").text(match t { 0 => "in", 1 => "out: click, then click a tip of another super", 2 => "control", _ => "gate" })),
            );
        }
    }
    // supers
    let mut nodes: Vec<N> = Vec::new();
    for (m, (name, t)) in g.members.iter().enumerate() {
        let (x, y, bw, bh) = bnode(&b, m);
        let note = match t {
            Target::Url(_) => "Opens in a new tab",
            Target::Data(_, d) => *d,
            Target::Soon(_) => "Being rewritten in Rust",
            _ => "",
        };
        let pien = pien_pick.as_deref() == Some(*name);
        nodes.push(
            el("div")
                .cls("member bnode")
                .cls_if(b.picked == Some(m), "picked")
                .cls_if(pien, "pien")
                .attr_if(pien, "title", "PiEN suggests this next")
                .attr("role", "link")
                .attr("tabindex", "0")
                .attr("data-solid", "1")
                .style(format!("left:{:.0}px;top:{:.0}px;width:{:.0}px;height:{:.0}px", x, y, bw, bh))
                .on("pdown", Msg::TV(Tool::Grid, 1000 + m as u16))
                .on("enter", Msg::T(Tool::Grid, 8, format!("{}", m)))
                .cls_if(icon_area(bw, bh), "icon")
                .attr_if(icon_area(bw, bh), "title", name.to_string())
                .kid_if(icon_area(bw, bh), || el("span").cls("icon-g").text(glyph(name)).into())
                .kid_if(!icon_area(bw, bh), || el("span").cls("tag").text(*name).into())
                .kid_if(!icon_area(bw, bh) && !note.is_empty(), || el("span").cls("m-note").text(note).into())
                .into(),
        );
    }
    for (k, (kind, _, _, label)) in b.extra.iter().enumerate() {
        let i = b.pos.len() + k;
        let (x, y, bw, bh) = bnode(&b, i);
        let base = el("div").cls("bnode").cls_if(b.picked == Some(i), "picked").attr("data-solid", "1").style(format!("left:{:.0}px;top:{:.0}px;width:{:.0}px;height:{:.0}px", x, y, bw, bh)).on("pdown", Msg::TV(Tool::Grid, 1000 + i as u16));
        nodes.push(if *kind == 0 {
            base.cls("bspark").attr("title", "A sparkle: a junction for wires").kid(el("svg:svg").attr("viewBox", "-22 -22 44 44").attr("width", "44").attr("height", "44").kid(el("svg:path").attr("d", sparkle(0.0, 0.0, 18.0)))).into()
        } else {
            base.cls("buser").kid(
                el("input")
                    .cls("buser-in")
                    .attr("type", "text")
                    .attr("placeholder", "Type a name")
                    .attr("aria-label", "This super's name")
                    .attr("data-stop", "1")
                    .prop("value", label.clone())
                    .on("input", Msg::TV(Tool::Grid, 2000 + k as u16)),
            )
            .into()
        });
    }
    el("div")
        .kid(head(g.title, "Everything in this super, on its own canvas. Double click one to open it; wire an out into an in; double click for a sparkle, double right click for a super of your own. Every super here is locked in place. To move or resize one, click it, then the lock; click the lock again to lock it back."))
        .kid(
            el("div")
                .cls("board")
                .cls_if(b.held.is_some(), "holding")
                .on_if(b.held.is_some(), "mmove", Msg::TV(Tool::Grid, 71))
                .style(format!("height:{:.0}px;min-width:{:.0}px", h, w))
                .on("dbl", Msg::TV(Tool::Grid, 4))
                .on("dbl2", Msg::TV(Tool::Grid, 5))
                .on("click", Msg::T(Tool::Grid, 3, "-1".into()))
                .kid(wires)
                .kids(nodes)
                .kid(tips),
        )
        .kid(el("div").cls("row board-bar").kid(el("span").cls("hint").text(b.note.clone())))
        .into()
}

const BOARD_CSS: &str = "\
.board{position:relative;margin-top:6px;overflow:visible;user-select:none;-webkit-user-select:none}\
.board .wires{z-index:1}.board .w-hit{pointer-events:stroke}\
.board .bnode{position:absolute;z-index:2;box-sizing:border-box;cursor:grab;touch-action:none}\
.board .bnode.member{min-height:0}\
.board .board-ports{position:absolute;left:0;top:0;z-index:4;overflow:visible;pointer-events:none}\
.board .bspark svg path{fill:var(--paper);stroke:var(--ink);stroke-width:1.6}\
.board .buser{display:flex;align-items:center;padding:10px 14px;background:#fff;border:1.6px solid var(--ink);border-radius:18px}\
.board .buser-in{width:100%;border:none;background:none;font:inherit;font-size:15px;outline:none;cursor:text}\
.board-bar{margin-top:10px;justify-content:space-between}\
";
