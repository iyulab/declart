//! Orthogonal link routing on a coarse grid.
//!
//! Every node and group edge sits on a grid line, so a link leaves a node at a grid point of its
//! border (a *slot*) and travels along grid lines. A* finds the cheapest path per link: it may not
//! enter a node, and pays for bends, for running alongside nodes, group borders and group
//! labels, for crossing another link, and a lot for running on top of another link. Links are
//! routed shortest first, in declaration order on a tie, so the result is deterministic.

use super::layout::G;
use super::Rect;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Most grid points a canvas may have — about 5000 × 5000 px — past which links are drawn
/// straight rather than routed.
const MAX_CELLS: i64 = 150_000;
const STEP: u32 = 10;
/// Weight on the distance estimate (as quarters): above 1 the search heads for the goal faster and may settle
/// for a path a little dearer than the cheapest.
const HEURISTIC_WEIGHT: u32 = 5; // in quarters: 5/4
/// The weight for a layout too big to search carefully in good time: 2.
const HURRY_WEIGHT: u32 = 8;
const BEND: u32 = 60;
/// A turn right after a turn leaves a one-step jog in the line, which reads as a slip.
const JOG: u32 = 200;
/// The run state of a segment that is long enough to turn off.
const LONG_RUN: u32 = 2;
const HALO: u32 = 20;
const GROUP_BORDER: u32 = 40;
const GROUP_BESIDE: u32 = 25;
const GROUP_LABEL: u32 = 300;
const OVERLAP: u32 = 300;
const CROSS: u32 = 80;
/// Cost of running on the line of another link, within a few steps of where it ends.
const BEYOND: u32 = 150;
const BEYOND_STEPS: i32 = 4;
/// Cost of taking a port beside a used one (1 step away) or two steps away: two links leaving
/// side by side run so close that a label cannot sit between them.
const NEXT_TO_USED_SLOT: u32 = 150;
const NEAR_USED_SLOT: u32 = 70;
const NEXT_TO_LINK: u32 = 120;

/// A grid point as (column, row).
type P = (i32, i32);

/// Directions, as (dx, dy): up, right, down, left.
const DIRS: [P; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Copy)]
struct Slot {
    /// The border point the link attaches to.
    border: P,
    /// The first free point outside the border.
    outside: P,
    /// Direction from the border outward.
    out_dir: usize,
}

/// One end of a link: a node or a group box.
pub(crate) struct End {
    pub rect: Rect,
}

pub(crate) struct Obstacles<'a> {
    /// Boxes links may not enter (nodes).
    pub nodes: &'a [Rect],
    /// Group boxes — links may cross them but should not run along their border.
    pub groups: &'a [Rect],
    /// Group label boxes — links may cross them but pay dearly.
    pub group_labels: &'a [Rect],
}

pub(crate) struct Router {
    cols: i32,
    rows: i32,
    blocked: Vec<bool>,
    base: Vec<u32>,
    h_used: Vec<u8>,
    v_used: Vec<u8>,
    /// Points just beyond the end of a routed horizontal / vertical run: a second link on the
    /// same line there would read as the first one carrying on.
    h_beyond: Vec<u8>,
    v_beyond: Vec<u8>,
    // Search tables, kept between links and cleaned up after each (only the entries a search
    // touched), so routing many links does not allocate and clear them again and again.
    dist: Vec<u32>,
    prev: Vec<u32>,
    goal: Vec<i32>,
    /// Weight on the distance estimate, in quarters.
    heuristic: u32,
}

fn to_p(v: f32) -> i32 {
    (v / G).round() as i32
}

impl Router {
    /// `None` when the canvas is too large to route on (the search tables grow with it).
    pub(crate) fn new(width: f32, height: f32, obs: &Obstacles, hurry: bool) -> Option<Self> {
        let cols = to_p(width) as i64 + 1;
        let rows = to_p(height) as i64 + 1;
        if cols * rows > MAX_CELLS {
            return None;
        }
        let (cols, rows) = (cols as i32, rows as i32);
        let n = (cols * rows) as usize;
        let mut r = Router {
            cols,
            rows,
            blocked: vec![false; n],
            base: vec![0; n],
            h_used: vec![0; n],
            v_used: vec![0; n],
            h_beyond: vec![0; n],
            v_beyond: vec![0; n],
            dist: vec![u32::MAX; n * 12],
            prev: vec![u32::MAX; n * 12],
            goal: vec![-1; n],
            heuristic: if hurry {
                HURRY_WEIGHT
            } else {
                HEURISTIC_WEIGHT
            },
        };
        for rect in obs.nodes {
            let (x0, y0, x1, y1) = r.bounds(rect);
            for j in (y0 - 1)..=(y1 + 1) {
                for i in (x0 - 1)..=(x1 + 1) {
                    let Some(k) = r.idx((i, j)) else { continue };
                    if i >= x0 && i <= x1 && j >= y0 && j <= y1 {
                        r.blocked[k] = true;
                    } else {
                        r.base[k] += HALO;
                    }
                }
            }
        }
        for rect in obs.groups {
            let (x0, y0, x1, y1) = r.bounds(rect);
            // The border itself, and the line of points on either side of it: a link there reads
            // as part of the border.
            for i in x0..=x1 {
                for y in [y0, y1] {
                    r.add_base((i, y), GROUP_BORDER);
                    r.add_base((i, y + 1), GROUP_BESIDE);
                    r.add_base((i, y - 1), GROUP_BESIDE);
                }
            }
            for j in y0..=y1 {
                for x in [x0, x1] {
                    r.add_base((x, j), GROUP_BORDER);
                    r.add_base((x + 1, j), GROUP_BESIDE);
                    r.add_base((x - 1, j), GROUP_BESIDE);
                }
            }
        }
        for rect in obs.group_labels {
            let x0 = (rect.x / G).floor() as i32;
            let y0 = (rect.y / G).floor() as i32;
            let x1 = ((rect.x + rect.w) / G).ceil() as i32;
            let y1 = ((rect.y + rect.h) / G).ceil() as i32;
            for j in y0..=y1 {
                for i in x0..=x1 {
                    r.add_base((i, j), GROUP_LABEL);
                }
            }
        }
        Some(r)
    }

    /// Puts the search tables back, cleaned of what the last search touched.
    fn release(
        &mut self,
        mut dist: Vec<u32>,
        mut prev: Vec<u32>,
        mut goal: Vec<i32>,
        touched: &[u32],
        to_slots: &[Slot],
    ) {
        for &s in touched {
            dist[s as usize] = u32::MAX;
            prev[s as usize] = u32::MAX;
        }
        for slot in to_slots {
            if let Some(k) = self.idx(slot.outside) {
                goal[k] = -1;
            }
        }
        self.dist = dist;
        self.prev = prev;
        self.goal = goal;
    }

    fn bounds(&self, rect: &Rect) -> (i32, i32, i32, i32) {
        (
            to_p(rect.x),
            to_p(rect.y),
            to_p(rect.x + rect.w),
            to_p(rect.y + rect.h),
        )
    }

    fn idx(&self, (i, j): P) -> Option<usize> {
        (i >= 0 && j >= 0 && i < self.cols && j < self.rows).then(|| (j * self.cols + i) as usize)
    }

    fn add_base(&mut self, p: P, cost: u32) {
        if let Some(k) = self.idx(p) {
            self.base[k] += cost;
        }
    }

    /// The border slots of a box: grid points on its border, away from the corners.
    fn slots(&self, rect: &Rect) -> Vec<Slot> {
        let (x0, y0, x1, y1) = self.bounds(rect);
        let inset = if x1 - x0 >= 6 && y1 - y0 >= 6 { 2 } else { 1 };
        let mut slots = Vec::new();
        for i in (x0 + inset)..=(x1 - inset) {
            slots.push(Slot {
                border: (i, y0),
                outside: (i, y0 - 1),
                out_dir: 0,
            });
        }
        for j in (y0 + inset)..=(y1 - inset) {
            slots.push(Slot {
                border: (x1, j),
                outside: (x1 + 1, j),
                out_dir: 1,
            });
        }
        for i in (x0 + inset)..=(x1 - inset) {
            slots.push(Slot {
                border: (i, y1),
                outside: (i, y1 + 1),
                out_dir: 2,
            });
        }
        for j in (y0 + inset)..=(y1 - inset) {
            slots.push(Slot {
                border: (x0, j),
                outside: (x0 - 1, j),
                out_dir: 3,
            });
        }
        slots
            .into_iter()
            .filter(|s| self.idx(s.outside).is_some_and(|k| !self.blocked[k]))
            .collect()
    }

    /// Routes one link from `from` to `to`. `from_taken` / `to_taken` track which slots of each
    /// end are in use (0 free, 1 next to a used one, 2 used). Returns the polyline in pixels.
    pub(crate) fn route(
        &mut self,
        from: &End,
        from_taken: &mut Vec<u8>,
        to: &End,
        to_taken: &mut Vec<u8>,
    ) -> Option<Vec<(f32, f32)>> {
        let from_slots = self.slots(&from.rect);
        let to_slots = self.slots(&to.rect);
        if from_taken.len() != from_slots.len() {
            *from_taken = vec![0; from_slots.len()];
        }
        if to_taken.len() != to_slots.len() {
            *to_taken = vec![0; to_slots.len()];
        }
        // A node whose slots are all taken shares them rather than leave a link unrouted.
        for taken in [&mut *from_taken, &mut *to_taken] {
            if taken.iter().all(|&s| s >= 2) {
                taken.iter_mut().for_each(|s| *s = 1);
            }
        }

        // Goal points: the free points just outside the target's slots.
        let mut goal = std::mem::take(&mut self.goal);
        for (s, slot) in to_slots.iter().enumerate() {
            if to_taken[s] < 2 {
                if let Some(k) = self.idx(slot.outside) {
                    goal[k] = s as i32;
                }
            }
        }
        let (tx0, ty0, tx1, ty1) = self.bounds(&to.rect);
        let weight = self.heuristic;
        let heur = |p: P| -> u32 {
            let dx = (tx0 - p.0).max(0).max(p.0 - tx1);
            let dy = (ty0 - p.1).max(0).max(p.1 - ty1);
            (dx + dy) as u32 * STEP * weight / 4
        };

        let mut dist = std::mem::take(&mut self.dist);
        let mut prev = std::mem::take(&mut self.prev);
        let mut touched: Vec<u32> = Vec::new();
        let mut heap: BinaryHeap<Reverse<(u32, u32, u32)>> = BinaryHeap::new();
        for (s, slot) in from_slots.iter().enumerate() {
            if from_taken[s] >= 2 {
                continue;
            }
            let Some(k) = self.idx(slot.outside) else {
                continue;
            };
            let g = slot_cost(from_taken[s]);
            let st = ((k * 4 + slot.out_dir) * 3 + LONG_RUN as usize) as u32;
            if g < dist[st as usize] {
                dist[st as usize] = g;
                touched.push(st);
                heap.push(Reverse((g + heur(slot.outside), g, st)));
            }
        }

        let mut found: Option<u32> = None;
        while let Some(Reverse((_, g, st))) = heap.pop() {
            if g > dist[st as usize] {
                continue;
            }
            let k = (st / 12) as usize;
            let dir = ((st / 3) % 4) as usize;
            // Steps taken on the segment being walked, less one, up to a long run: 0 and 1 are a
            // segment too short to turn off without leaving a jog.
            let run = (st % 3) as usize;
            let short = run < LONG_RUN as usize;
            if goal[k] >= 0 {
                found = Some(st);
                break;
            }
            let p: P = ((k as i32) % self.cols, (k as i32) / self.cols);
            for (nd, d) in DIRS.iter().enumerate() {
                if nd == (dir + 2) % 4 {
                    continue; // no U-turns
                }
                let q: P = (p.0 + d.0, p.1 + d.1);
                let Some(qk) = self.idx(q) else { continue };
                if self.blocked[qk] {
                    continue;
                }
                let mut cost = STEP + self.base[qk];
                if nd != dir {
                    cost += BEND;
                    if short {
                        cost += JOG;
                    }
                }
                let horizontal = nd % 2 == 1;
                if horizontal {
                    if self.h_used[qk] > 0 || self.h_used[k] > 0 {
                        cost += OVERLAP;
                    }
                    if self.v_used[qk] > 0 {
                        cost += CROSS;
                    }
                    if self.h_beyond[qk] > 0 {
                        cost += BEYOND;
                    }
                } else {
                    if self.v_used[qk] > 0 || self.v_used[k] > 0 {
                        cost += OVERLAP;
                    }
                    if self.h_used[qk] > 0 {
                        cost += CROSS;
                    }
                    if self.v_beyond[qk] > 0 {
                        cost += BEYOND;
                    }
                }
                // Keep parallel links apart, so a label fits between two of them.
                let apart = |dx: i32, dy: i32| -> u32 {
                    let mut c = 0;
                    for s in [-2, -1, 1, 2] {
                        let (i, j) = (q.0 + dx * s, q.1 + dy * s);
                        if let Some(k) = self.idx((i, j)) {
                            let used = if horizontal {
                                self.h_used[k]
                            } else {
                                self.v_used[k]
                            };
                            if used > 0 {
                                c += NEXT_TO_LINK;
                            }
                        }
                    }
                    c
                };
                cost += if horizontal { apart(0, 1) } else { apart(1, 0) };
                if goal[qk] >= 0 {
                    let slot = &to_slots[goal[qk] as usize];
                    // The last step into the border runs against `out_dir`.
                    if nd != (slot.out_dir + 2) % 4 {
                        cost += BEND;
                        if nd != dir && short {
                            cost += JOG;
                        }
                    }
                    cost += slot_cost(to_taken[goal[qk] as usize]);
                }
                let ng = g + cost;
                let next_run = if nd == dir {
                    (run + 1).min(LONG_RUN as usize)
                } else {
                    0
                };
                let ns = ((qk * 4 + nd) * 3 + next_run) as u32;
                if ng < dist[ns as usize] {
                    dist[ns as usize] = ng;
                    touched.push(ns);
                    prev[ns as usize] = st;
                    heap.push(Reverse((ng + heur(q), ng, ns)));
                }
            }
        }
        let Some(end) = found else {
            self.release(dist, prev, goal, &touched, &to_slots);
            return None;
        };
        let to_slot = goal[(end / 12) as usize] as usize;

        // Walk back to the start.
        let mut points: Vec<P> = Vec::new();
        let mut cur = end;
        loop {
            let k = (cur / 12) as usize;
            points.push(((k as i32) % self.cols, (k as i32) / self.cols));
            if prev[cur as usize] == u32::MAX {
                break;
            }
            cur = prev[cur as usize];
        }
        points.reverse();
        self.release(dist, prev, goal, &touched, &to_slots);

        let start_k = self.idx(points[0]).expect("start inside grid");
        let from_slot = from_slots
            .iter()
            .enumerate()
            .find(|(s, sl)| from_taken[*s] < 2 && self.idx(sl.outside) == Some(start_k))
            .map(|(s, _)| s)
            .expect("start is an outside point of a source slot");

        let mut poly: Vec<P> = vec![from_slots[from_slot].border];
        poly.extend(points.iter().copied());
        poly.push(to_slots[to_slot].border);

        // Mark usage and slots.
        for w in poly.windows(2) {
            let (a, b) = (w[0], w[1]);
            let horizontal = a.1 == b.1;
            for p in [a, b] {
                if let Some(k) = self.idx(p) {
                    if horizontal {
                        self.h_used[k] = self.h_used[k].saturating_add(1);
                    } else {
                        self.v_used[k] = self.v_used[k].saturating_add(1);
                    }
                }
            }
        }
        // The stretch beyond each end of a run, on its line.
        for w in poly.windows(2) {
            let (a, b) = (w[0], w[1]);
            if a == b {
                continue;
            }
            let horizontal = a.1 == b.1;
            let (lo, hi) = if horizontal {
                (a.0.min(b.0), a.0.max(b.0))
            } else {
                (a.1.min(b.1), a.1.max(b.1))
            };
            for e in 1..=BEYOND_STEPS {
                for at in [lo - e, hi + e] {
                    let p = if horizontal { (at, a.1) } else { (a.0, at) };
                    if let Some(k) = self.idx(p) {
                        if horizontal {
                            self.h_beyond[k] = self.h_beyond[k].saturating_add(1);
                        } else {
                            self.v_beyond[k] = self.v_beyond[k].saturating_add(1);
                        }
                    }
                }
            }
        }
        take(from_taken, &from_slots, from_slot);
        take(to_taken, &to_slots, to_slot);

        Some(simplify(&poly))
    }
}

/// What taking a port costs: 0 free, 1 next to a used one, 3 two steps from one, 2 used.
fn slot_cost(state: u8) -> u32 {
    match state {
        1 => NEXT_TO_USED_SLOT,
        3 => NEAR_USED_SLOT,
        _ => 0,
    }
}

/// Marks slot `s` used, and the slots near it on the same side as discouraged.
fn take(state: &mut [u8], slots: &[Slot], s: usize) {
    state[s] = 2;
    for (t, mark) in [
        (s.wrapping_sub(2), 3),
        (s.wrapping_sub(1), 1),
        (s + 1, 1),
        (s + 2, 3),
    ] {
        if t < slots.len() && slots[t].out_dir == slots[s].out_dir && state[t] != 2 {
            // The nearer mark wins over the farther one.
            state[t] = if state[t] == 1 { 1 } else { mark };
        }
    }
}

/// Drops points in the middle of a straight run and converts to pixels.
fn simplify(poly: &[P]) -> Vec<(f32, f32)> {
    let mut out: Vec<P> = Vec::new();
    for &p in poly {
        if out.last() == Some(&p) {
            continue;
        }
        while out.len() >= 2 {
            let (a, b) = (out[out.len() - 2], out[out.len() - 1]);
            let same_line = (a.0 == b.0 && b.0 == p.0) || (a.1 == b.1 && b.1 == p.1);
            if same_line {
                out.pop();
            } else {
                break;
            }
        }
        out.push(p);
    }
    out.iter()
        .map(|&(i, j)| (i as f32 * G, j as f32 * G))
        .collect()
}
