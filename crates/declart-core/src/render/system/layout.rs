//! Layered layout of one level of a system diagram.
//!
//! The items of a level (nodes and already-laid-out groups) are placed in layers along the
//! rank axis — left to right, or top to bottom — so that links run forward. Cycles are broken
//! at the edge that closes them; layers come from the longest path; long links get a dummy node
//! in every layer they pass, which reserves a corridor for them; the order inside a layer
//! comes from barycenter sweeps, keeping the ordering with the fewest crossings; and the
//! position across the layers is the closest to the neighbours' centres that keeps the order
//! and the gaps (isotonic regression). Everything is sorted by index, so the result is
//! deterministic. All sizes and positions are multiples of [`G`].

/// Grid pitch: nodes and groups sit on a grid of this many pixels so links can run along it.
pub(crate) const G: f32 = 10.0;

pub(crate) fn snap(v: f32) -> f32 {
    (v / G).round() * G
}

pub(crate) fn snap_up(v: f32) -> f32 {
    (v / G).ceil() * G
}

/// Distance between layers.
const RANK_GAP: f32 = 80.0;
/// Distance between two items of one layer.
const CROSS_GAP: f32 = 40.0;
/// Room kept for a link corridor (a dummy node) in a layer.
const CORRIDOR: f32 = 20.0;
/// Room kept around a label on the rank axis, besides its width.
const LABEL_ROOM: f32 = 40.0;

pub(crate) struct LevelEdge {
    pub a: usize,
    pub b: usize,
    /// Width of the link's label (0 when it has none) — widens the gap between layers.
    pub label_w: f32,
}

pub(crate) struct LevelLayout {
    /// Top-left of every item.
    pub pos: Vec<(f32, f32)>,
    pub w: f32,
    pub h: f32,
}

struct LNode {
    layer: usize,
    /// Extent across the rank axis.
    c: f32,
    /// Extent along the rank axis.
    r: f32,
    real: bool,
}

pub(crate) fn layout_level(sizes: &[(f32, f32)], edges: &[LevelEdge], lr: bool) -> LevelLayout {
    let n = sizes.len();
    if n == 0 {
        return LevelLayout {
            pos: vec![],
            w: 0.0,
            h: 0.0,
        };
    }

    // Unique directed edges, with the widest label per pair.
    let mut es: Vec<(usize, usize, f32)> = Vec::new();
    for e in edges {
        if e.a == e.b {
            continue;
        }
        if let Some(x) = es.iter_mut().find(|x| x.0 == e.a && x.1 == e.b) {
            x.2 = x.2.max(e.label_w);
        } else {
            es.push((e.a, e.b, e.label_w));
        }
    }

    // Break cycles: a depth-first walk in index order reverses the edge that closes a cycle.
    let mut out: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b, _) in &es {
        out[a].push(b);
    }
    for v in &mut out {
        v.sort_unstable();
        v.dedup();
    }
    let mut state = vec![0u8; n];
    let mut dag: Vec<(usize, usize)> = Vec::new();
    fn dfs(u: usize, out: &[Vec<usize>], state: &mut [u8], dag: &mut Vec<(usize, usize)>) {
        state[u] = 1;
        for &v in &out[u] {
            match state[v] {
                0 => {
                    dag.push((u, v));
                    dfs(v, out, state, dag);
                }
                1 => dag.push((v, u)),
                _ => dag.push((u, v)),
            }
        }
        state[u] = 2;
    }
    for u in 0..n {
        if state[u] == 0 {
            dfs(u, &out, &mut state, &mut dag);
        }
    }
    dag.sort_unstable();
    dag.dedup();

    // Layers: longest path from the sources; a source moves next to its nearest successor.
    let mut indeg = vec![0usize; n];
    let mut succ: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in &dag {
        indeg[b] += 1;
        succ[a].push(b);
    }
    let mut layer = vec![0usize; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut remaining = indeg.clone();
    let mut ready: Vec<usize> = (0..n).filter(|&i| remaining[i] == 0).collect();
    while !ready.is_empty() {
        ready.sort_unstable();
        let u = ready.remove(0);
        order.push(u);
        for &v in &succ[u] {
            layer[v] = layer[v].max(layer[u] + 1);
            remaining[v] -= 1;
            if remaining[v] == 0 {
                ready.push(v);
            }
        }
    }
    for &u in order.iter().rev() {
        if indeg[u] == 0 {
            if let Some(m) = succ[u].iter().map(|&v| layer[v]).min() {
                layer[u] = m - 1;
            }
        }
    }

    // The layered graph, with a dummy node per layer a long link passes.
    let mut nodes: Vec<LNode> = (0..n)
        .map(|i| {
            let (w, h) = sizes[i];
            let (r, c) = if lr { (w, h) } else { (h, w) };
            LNode {
                layer: layer[i],
                c,
                r,
                real: true,
            }
        })
        .collect();
    let mut ladj: Vec<(usize, usize)> = Vec::new();
    for &(a, b) in &dag {
        let (la, lb) = (layer[a], layer[b]);
        let mut upper = a;
        for l in (la + 1)..lb {
            let d = nodes.len();
            nodes.push(LNode {
                layer: l,
                c: CORRIDOR,
                r: 0.0,
                real: false,
            });
            ladj.push((upper, d));
            upper = d;
        }
        ladj.push((upper, b));
    }
    let n_layers = nodes.iter().map(|x| x.layer).max().unwrap_or(0) + 1;
    let mut layers: Vec<Vec<usize>> = vec![vec![]; n_layers];
    for (i, x) in nodes.iter().enumerate() {
        layers[x.layer].push(i);
    }
    let total = nodes.len();
    let mut up: Vec<Vec<usize>> = vec![vec![]; total];
    let mut down: Vec<Vec<usize>> = vec![vec![]; total];
    for &(a, b) in &ladj {
        down[a].push(b);
        up[b].push(a);
    }

    // Order inside each layer: barycenter sweeps, keeping the fewest crossings.
    let mut best = layers.clone();
    let mut best_x = crossings(&best, &ladj, &nodes);
    for sweep in 0..8 {
        if sweep % 2 == 0 {
            for l in 1..n_layers {
                reorder(&mut layers, l, &up, total);
            }
        } else {
            for l in (0..n_layers.saturating_sub(1)).rev() {
                reorder(&mut layers, l, &down, total);
            }
        }
        let x = crossings(&layers, &ladj, &nodes);
        if x < best_x {
            best_x = x;
            best = layers.clone();
        }
    }
    let layers = best;

    // Position across the layers.
    let gap_of = |a: usize, b: usize| {
        if nodes[a].real && nodes[b].real {
            CROSS_GAP
        } else {
            CORRIDOR
        }
    };
    let mut cc = vec![0.0f32; total];
    for lay in &layers {
        let len: f32 = lay.iter().map(|&i| nodes[i].c).sum::<f32>()
            + lay.windows(2).map(|w| gap_of(w[0], w[1])).sum::<f32>();
        let mut at = -len / 2.0;
        for &i in lay {
            cc[i] = at + nodes[i].c / 2.0;
            at += nodes[i].c;
            if let Some(&nx) = lay.iter().skip_while(|&&j| j != i).nth(1) {
                at += gap_of(i, nx);
            }
        }
    }
    for _ in 0..4 {
        for lay in layers.iter().skip(1) {
            settle(lay, &up, &mut cc, &nodes, &gap_of);
        }
        for lay in layers.iter().take(n_layers - 1).rev() {
            settle(lay, &down, &mut cc, &nodes, &gap_of);
        }
    }

    // Snap to the grid, keep the gaps, and move everything to start at zero.
    let mut top = vec![0.0f32; total];
    for lay in &layers {
        let mut floor = f32::MIN;
        for (k, &i) in lay.iter().enumerate() {
            let mut t = snap(cc[i] - nodes[i].c / 2.0);
            if k > 0 {
                t = t.max(floor);
            }
            top[i] = t;
            floor = t + nodes[i].c + gap_of(i, lay.get(k + 1).copied().unwrap_or(i));
        }
    }
    let min_top = (0..total).map(|i| top[i]).fold(f32::MAX, f32::min);
    for t in &mut top {
        *t -= min_top;
    }
    let cross_total = (0..total).map(|i| top[i] + nodes[i].c).fold(0.0, f32::max);

    // Position along the rank axis.
    let layer_r: Vec<f32> = layers
        .iter()
        .map(|lay| lay.iter().map(|&i| nodes[i].r).fold(0.0, f32::max))
        .collect();
    let mut gaps = vec![RANK_GAP; n_layers.saturating_sub(1)];
    if lr {
        for &(a, b, lw) in &es {
            let (la, lb) = (layer[a].min(layer[b]), layer[a].max(layer[b]));
            if lw > 0.0 && lb > la {
                gaps[la] = gaps[la].max(snap_up(lw + LABEL_ROOM));
            }
        }
    }
    let mut rank_off = vec![0.0f32; n_layers];
    for l in 1..n_layers {
        rank_off[l] = rank_off[l - 1] + layer_r[l - 1] + gaps[l - 1];
    }
    let rank_total = rank_off[n_layers - 1] + layer_r[n_layers - 1];

    let pos = (0..n)
        .map(|i| {
            let l = nodes[i].layer;
            let rp = rank_off[l] + snap((layer_r[l] - nodes[i].r) / 2.0);
            if lr {
                (rp, top[i])
            } else {
                (top[i], rp)
            }
        })
        .collect();
    let (w, h) = if lr {
        (rank_total, cross_total)
    } else {
        (cross_total, rank_total)
    };
    LevelLayout { pos, w, h }
}

/// Sorts layer `l` by the mean position of each node's neighbours in the adjacent layer.
fn reorder(layers: &mut [Vec<usize>], l: usize, adj: &[Vec<usize>], total: usize) {
    let mut pos = vec![0.0f32; total];
    for lay in layers.iter() {
        for (k, &i) in lay.iter().enumerate() {
            pos[i] = k as f32;
        }
    }
    let mut keyed: Vec<(f32, usize, usize)> = layers[l]
        .iter()
        .enumerate()
        .map(|(k, &i)| {
            let ns = &adj[i];
            let bary = if ns.is_empty() {
                k as f32
            } else {
                ns.iter().map(|&j| pos[j]).sum::<f32>() / ns.len() as f32
            };
            (bary, k, i)
        })
        .collect();
    keyed.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    layers[l] = keyed.into_iter().map(|(_, _, i)| i).collect();
}

/// Crossings between adjacent layers.
fn crossings(layers: &[Vec<usize>], ladj: &[(usize, usize)], nodes: &[LNode]) -> usize {
    let mut pos = vec![0usize; nodes.len()];
    for lay in layers {
        for (k, &i) in lay.iter().enumerate() {
            pos[i] = k;
        }
    }
    let mut count = 0;
    for (x, &(a, b)) in ladj.iter().enumerate() {
        for &(c, d) in &ladj[x + 1..] {
            if nodes[a].layer != nodes[c].layer {
                continue;
            }
            let (pa, pc) = (pos[a], pos[c]);
            let (pb, pd) = (pos[b], pos[d]);
            if (pa < pc && pb > pd) || (pa > pc && pb < pd) {
                count += 1;
            }
        }
    }
    count
}

/// Moves the nodes of one layer toward the mean centre of their neighbours in the reference
/// layer, keeping their order and gaps: the closest such placement in least squares.
fn settle(
    lay: &[usize],
    adj: &[Vec<usize>],
    cc: &mut [f32],
    nodes: &[LNode],
    gap_of: &dyn Fn(usize, usize) -> f32,
) {
    if lay.is_empty() {
        return;
    }
    let desired: Vec<f32> = lay
        .iter()
        .map(|&i| {
            let ns = &adj[i];
            if ns.is_empty() {
                cc[i]
            } else {
                ns.iter().map(|&j| cc[j]).sum::<f32>() / ns.len() as f32
            }
        })
        .collect();
    let mut offset = vec![0.0f32; lay.len()];
    for k in 1..lay.len() {
        let (a, b) = (lay[k - 1], lay[k]);
        offset[k] = offset[k - 1] + (nodes[a].c + nodes[b].c) / 2.0 + gap_of(a, b);
    }
    // Pool adjacent violators on t = desired - offset (must be non-decreasing).
    let t: Vec<f32> = (0..lay.len()).map(|k| desired[k] - offset[k]).collect();
    let mut blocks: Vec<(f32, usize, usize)> = Vec::new(); // (sum, count, start)
    for (k, &v) in t.iter().enumerate() {
        blocks.push((v, 1, k));
        while blocks.len() >= 2 {
            let (s2, c2, _) = blocks[blocks.len() - 1];
            let (s1, c1, st1) = blocks[blocks.len() - 2];
            if s1 / c1 as f32 > s2 / c2 as f32 {
                blocks.truncate(blocks.len() - 2);
                blocks.push((s1 + s2, c1 + c2, st1));
            } else {
                break;
            }
        }
    }
    for (sum, count, start) in blocks {
        let mean = sum / count as f32;
        for k in start..start + count {
            cc[lay[k]] = mean + offset[k];
        }
    }
}
