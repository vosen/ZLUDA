//! cuDNN's backend (graph) API: descriptors hold their attributes as set; finalizing an operation graph takes a
//! snapshot of its tensors and operations, heuristics offer one engine when every operation is supported, and an
//! execution plan lays out its workspace. Execution runs the operations in dependency order in f32 on packed
//! tensors: f32 tensors that are packed already are used in place, others (f16, bf16, strided) through f32 copies in
//! the workspace, like virtual tensors and by-value scalars.
//!
//! Operations: 2-D convolution forward, backward data and backward filter (any group count, symmetric padding),
//! matmul (2-D or batched 3-D, with broadcast batches), every pointwise mode but atan2 and index generation, and
//! reductions, on tensors of up to 4 dimensions.

use super::*;
use std::collections::HashMap;

/// The element size of a backend attribute type.
fn elem_size(ty: u32) -> Option<usize> {
    Some(match ty {
        // HANDLE, INT64, DOUBLE, VOID_PTR, BACKEND_DESCRIPTOR
        0 | 3 | 5 | 6 | 15 => 8,
        // BOOLEAN, CHAR
        2 | 24 => 1,
        // FRACTION: two int64
        26 => 16,
        1 | 4 | 7..=14 | 16..=23 | 25 | 27..=29 => 4,
        _ => return None,
    })
}

const TYPE_DATA_TYPE: u32 = 1;
const TYPE_BOOLEAN: u32 = 2;
const TYPE_INT64: u32 = 3;
const TYPE_FLOAT: u32 = 4;
const TYPE_DOUBLE: u32 = 5;
const TYPE_BACKEND_DESCRIPTOR: u32 = 15;
const TYPE_CHAR: u32 = 24;

mod kind {
    pub const POINTWISE: u32 = 0;
    pub const CONVOLUTION: u32 = 1;
    pub const ENGINE: u32 = 2;
    pub const ENGINECFG: u32 = 3;
    pub const ENGINEHEUR: u32 = 4;
    pub const EXECUTION_PLAN: u32 = 5;
    pub const CONV_FWD: u32 = 10;
    pub const CONV_BWD_FILTER: u32 = 11;
    pub const CONV_BWD_DATA: u32 = 12;
    pub const OP_POINTWISE: u32 = 13;
    pub const OPERATIONGRAPH: u32 = 15;
    pub const VARIANT_PACK: u32 = 16;
    pub const TENSOR: u32 = 17;
    pub const MATMUL: u32 = 18;
    pub const OP_MATMUL: u32 = 19;
    pub const REDUCTION: u32 = 21;
    pub const OP_REDUCTION: u32 = 22;
}

mod attr {
    pub const PW_MODE: u32 = 0;
    pub const PW_RELU_LOWER_CLIP: u32 = 3;
    pub const PW_RELU_UPPER_CLIP: u32 = 4;
    pub const PW_RELU_LOWER_CLIP_SLOPE: u32 = 5;
    pub const PW_ELU_ALPHA: u32 = 6;
    pub const PW_SOFTPLUS_BETA: u32 = 7;
    pub const PW_SWISH_BETA: u32 = 8;
    pub const CONV_MODE: u32 = 101;
    pub const CONV_DILATIONS: u32 = 102;
    pub const CONV_FILTER_STRIDES: u32 = 103;
    pub const CONV_POST_PADDINGS: u32 = 104;
    pub const CONV_PRE_PADDINGS: u32 = 105;
    pub const CONV_SPATIAL_DIMS: u32 = 106;
    pub const HEUR_GRAPH: u32 = 201;
    pub const HEUR_RESULTS: u32 = 202;
    pub const CFG_ENGINE: u32 = 300;
    pub const CFG_INTERMEDIATE_INFO: u32 = 301;
    pub const CFG_KNOB_CHOICES: u32 = 302;
    pub const CFG_WORKSPACE_SIZE: u32 = 303;
    pub const CFG_SHARED_MEMORY_USED: u32 = 304;
    pub const PLAN_ENGINE_CONFIG: u32 = 401;
    pub const PLAN_WORKSPACE_SIZE: u32 = 402;
    pub const PLAN_COMPUTED_INTERMEDIATE_UIDS: u32 = 403;
    pub const PLAN_RUN_ONLY_INTERMEDIATE_UIDS: u32 = 404;
    pub const PLAN_JSON: u32 = 405;
    // conv forward: alpha, beta, desc, w, x, y; backward data: alpha, beta, desc, w, dx, dy; backward filter:
    // alpha, beta, desc, dw, x, dy
    pub const CONV_FWD: u32 = 700;
    pub const CONV_BWD_DATA: u32 = 706;
    pub const CONV_BWD_FILTER: u32 = 712;
    pub const OP_PW_DESC: u32 = 750;
    pub const OP_PW_X: u32 = 751;
    pub const OP_PW_B: u32 = 752;
    pub const OP_PW_Y: u32 = 753;
    pub const OP_PW_ALPHA1: u32 = 754;
    pub const OP_PW_ALPHA2: u32 = 755;
    pub const OP_PW_DX: u32 = 756;
    pub const OP_PW_DY: u32 = 757;
    pub const OP_PW_T: u32 = 758;
    pub const GRAPH_OPS: u32 = 801;
    pub const GRAPH_ENGINE_COUNT: u32 = 802;
    pub const TENSOR_BYTE_ALIGNMENT: u32 = 900;
    pub const TENSOR_DATA_TYPE: u32 = 901;
    pub const TENSOR_DIMENSIONS: u32 = 902;
    pub const TENSOR_STRIDES: u32 = 903;
    pub const TENSOR_VECTOR_COUNT: u32 = 904;
    pub const TENSOR_VECTORIZED_DIMENSION: u32 = 905;
    pub const TENSOR_UNIQUE_ID: u32 = 906;
    pub const TENSOR_IS_VIRTUAL: u32 = 907;
    pub const TENSOR_IS_BY_VALUE: u32 = 908;
    pub const VP_UIDS: u32 = 1000;
    pub const VP_POINTERS: u32 = 1001;
    pub const VP_WORKSPACE: u32 = 1003;
    pub const ENGINE_GRAPH: u32 = 1300;
    pub const ENGINE_GLOBAL_INDEX: u32 = 1301;
    pub const ENGINE_KNOB_INFO: u32 = 1302;
    pub const ENGINE_NUMERICAL_NOTE: u32 = 1303;
    pub const ENGINE_LAYOUT_INFO: u32 = 1304;
    pub const ENGINE_BEHAVIOR_NOTE: u32 = 1305;
    pub const OP_MATMUL_A: u32 = 1520;
    pub const OP_MATMUL_B: u32 = 1521;
    pub const OP_MATMUL_C: u32 = 1522;
    pub const REDUCTION_OPERATOR: u32 = 1600;
    pub const OP_REDUCTION_X: u32 = 1610;
    pub const OP_REDUCTION_Y: u32 = 1611;
    pub const OP_REDUCTION_DESC: u32 = 1612;
}

#[derive(Clone, Debug)]
struct Attr {
    ty: u32,
    count: i64,
    data: Vec<u8>,
}

/// A backend descriptor.
#[derive(Clone, Default)]
pub struct Desc {
    kind: u32,
    attrs: HashMap<u32, Attr>,
    finalized: bool,
    graph: Option<Graph>,
    plan: Option<Plan>,
    /// Descriptors this one hands out (the engine of a configuration from heuristics).
    owned: Vec<Box<Desc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TensorInfo {
    pub ty: u32,
    pub dims: Vec<i64>,
    pub strides: Vec<i64>,
    pub virt: bool,
    pub by_value: bool,
}

impl TensorInfo {
    pub fn elements(&self) -> i64 {
        self.dims.iter().product()
    }

    /// Whether the strides are the packed row-major ones.
    pub fn packed(&self) -> bool {
        let mut s = 1;
        for (d, st) in self.dims.iter().zip(&self.strides).rev() {
            if *d != 1 && *st != s {
                return false;
            }
            s *= d;
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// dir 0 forward (x, w -> y), 1 backward data (dy, w -> dx in `x`), 2 backward filter (x, dy -> dw in `w`).
    Conv {
        dir: u8,
        x: i64,
        w: i64,
        y: i64,
        pad: [i64; 2],
        stride: [i64; 2],
        dilation: [i64; 2],
        cross: bool,
        alpha: f32,
        beta: f32,
    },
    Pointwise {
        mode: u32,
        x: i64,
        b: Option<i64>,
        t: Option<i64>,
        y: i64,
        p: [f32; 3],
        alpha1: f32,
        alpha2: f32,
    },
    Matmul {
        a: i64,
        b: i64,
        c: i64,
    },
    Reduction {
        op: u32,
        x: i64,
        y: i64,
    },
}

impl Op {
    fn inputs(&self) -> Vec<i64> {
        match *self {
            Op::Conv { dir: 0, x, w, .. } => vec![x, w],
            Op::Conv { dir: 1, w, y, .. } => vec![w, y],
            Op::Conv { x, y, .. } => vec![x, y],
            Op::Pointwise { x, b, t, .. } => [Some(x), b, t].into_iter().flatten().collect(),
            Op::Matmul { a, b, .. } => vec![a, b],
            Op::Reduction { x, .. } => vec![x],
        }
    }

    fn output(&self) -> i64 {
        match *self {
            Op::Conv { dir: 0, y, .. } => y,
            Op::Conv { dir: 1, x, .. } => x,
            Op::Conv { w, .. } => w,
            Op::Pointwise { y, .. } => y,
            Op::Matmul { c, .. } => c,
            Op::Reduction { y, .. } => y,
        }
    }
}

/// An operation graph: its tensors by UID and its operations in dependency order, and why it cannot run if so.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Graph {
    pub tensors: HashMap<i64, TensorInfo>,
    pub ops: Vec<Op>,
    pub unsupported: Option<String>,
}

/// Where a tensor's f32 packed data lives during execution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Slot {
    /// The variant pack's pointer (f32 and packed).
    Direct,
    /// This byte offset in the workspace.
    Workspace(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub graph: Graph,
    pub slots: HashMap<i64, Slot>,
    pub workspace: usize,
}

type R<T> = Result<T, cudnnError_t>;

const F32: u32 = 0;
const F16: u32 = 2;
const BF16: u32 = 9;

impl Desc {
    fn attr(&self, name: u32) -> Option<&Attr> {
        self.attrs.get(&name)
    }

    fn i64s(&self, name: u32) -> Option<Vec<i64>> {
        let a = self.attr(name)?;
        match a.ty {
            TYPE_INT64 => Some(a.data.chunks(8).map(|c| i64::from_le_bytes(c.try_into().unwrap())).collect()),
            23 => Some(a.data.chunks(4).map(|c| i32::from_le_bytes(c.try_into().unwrap()) as i64).collect()),
            _ => None,
        }
    }

    fn u32_(&self, name: u32) -> Option<u32> {
        let a = self.attr(name)?;
        (a.data.len() >= 4).then(|| u32::from_le_bytes(a.data[..4].try_into().unwrap()))
    }

    fn bool_(&self, name: u32) -> bool {
        self.attr(name).is_some_and(|a| a.data.first().is_some_and(|&b| b != 0))
    }

    /// A FLOAT or DOUBLE scalar.
    fn f64_(&self, name: u32) -> Option<f64> {
        let a = self.attr(name)?;
        match a.ty {
            TYPE_FLOAT => Some(f32::from_le_bytes(a.data[..4].try_into().ok()?) as f64),
            TYPE_DOUBLE => Some(f64::from_le_bytes(a.data[..8].try_into().ok()?)),
            _ => None,
        }
    }

    fn ptrs(&self, name: u32) -> Vec<usize> {
        self.attr(name).map_or(vec![], |a| a.data.chunks(8).map(|c| usize::from_le_bytes(c.try_into().unwrap())).collect())
    }

    unsafe fn descs(&self, name: u32) -> R<Vec<&Desc>> {
        self.ptrs(name).into_iter().map(|p| (p as *const Desc).as_ref().ok_or(cudnnError_t::BAD_PARAM)).collect()
    }

    unsafe fn desc(&self, name: u32) -> R<&Desc> {
        self.descs(name)?.into_iter().next().ok_or(cudnnError_t::BAD_PARAM)
    }

    fn set_raw(&mut self, name: u32, ty: u32, count: i64, data: Vec<u8>) {
        self.attrs.insert(name, Attr { ty, count, data });
    }
}

pub fn create(k: u32) -> R<Box<Desc>> {
    match k {
        kind::POINTWISE
        | kind::CONVOLUTION
        | kind::ENGINE
        | kind::ENGINECFG
        | kind::ENGINEHEUR
        | kind::EXECUTION_PLAN
        | kind::CONV_FWD
        | kind::CONV_BWD_FILTER
        | kind::CONV_BWD_DATA
        | kind::OP_POINTWISE
        | kind::OPERATIONGRAPH
        | kind::VARIANT_PACK
        | kind::TENSOR
        | kind::MATMUL
        | kind::OP_MATMUL
        | kind::REDUCTION
        | kind::OP_REDUCTION
        // knob choices, intermediate info, layout info: stored, never needed
        | 6..=9 => Ok(Box::new(Desc { kind: k, ..Default::default() })),
        _ => Err(cudnnError_t::NOT_SUPPORTED),
    }
}

pub unsafe fn set_attribute(d: &mut Desc, name: u32, ty: u32, count: i64, data: *const c_void) -> R<()> {
    if d.finalized {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let size = elem_size(ty).ok_or(cudnnError_t::BAD_PARAM)?;
    if count < 0 || (count > 0 && data.is_null()) {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let bytes = std::slice::from_raw_parts(data as *const u8, size * count as usize).to_vec();
    d.set_raw(name, ty, count, bytes);
    Ok(())
}

fn tensor_info(t: &Desc) -> R<(i64, TensorInfo)> {
    if t.kind != kind::TENSOR {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let uid = t.i64s(attr::TENSOR_UNIQUE_ID).and_then(|v| v.first().copied()).ok_or(cudnnError_t::BAD_PARAM)?;
    let dims = t.i64s(attr::TENSOR_DIMENSIONS).ok_or(cudnnError_t::BAD_PARAM)?;
    let strides = t.i64s(attr::TENSOR_STRIDES).ok_or(cudnnError_t::BAD_PARAM)?;
    let ty = t.u32_(attr::TENSOR_DATA_TYPE).ok_or(cudnnError_t::BAD_PARAM)?;
    if dims.len() != strides.len() || dims.is_empty() || dims.iter().any(|&d| d < 1) {
        return Err(cudnnError_t::BAD_PARAM);
    }
    Ok((uid, TensorInfo { ty, dims, strides, virt: t.bool_(attr::TENSOR_IS_VIRTUAL), by_value: t.bool_(attr::TENSOR_IS_BY_VALUE) }))
}

/// Each dimension of `a` equals `y`'s or is 1 (both padded to 4 dimensions in front).
fn broadcastable(a: &TensorInfo, y: &TensorInfo) -> bool {
    let (pa, py) = (pad4(&a.dims), pad4(&y.dims));
    pa.iter().zip(&py).all(|(&x, &z)| x == z || x == 1)
}

/// Dimensions padded to 4 in front with 1s.
pub fn pad4(d: &[i64]) -> [i64; 4] {
    let mut r = [1; 4];
    let off = 4 - d.len().min(4);
    for (i, &v) in d.iter().take(4).enumerate() {
        r[off + i] = v;
    }
    r
}

/// The packed strides of padded dimensions, 0 where `of` (the operand's own dimensions) is 1 and `like` is not.
pub fn bstrides(of: &[i64; 4]) -> [i64; 4] {
    let mut s = [0; 4];
    let mut acc = 1;
    for i in (0..4).rev() {
        s[i] = if of[i] == 1 { 0 } else { acc };
        acc *= of[i];
    }
    s
}

const SUPPORTED_PW: [u32; 49] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 100, 101, 102, 103, 104, 105, 106, 107, 200, 201, 202, 203, 204, 205,
    206, 207, 300, 301, 302, 303, 304, 305, 400, 401, 402, 601,
];

/// Reads an operation descriptor into `ops`, its tensors into `tensors`.
unsafe fn read_op(o: &Desc, tensors: &mut HashMap<i64, TensorInfo>) -> R<Result<Op, String>> {
    let mut tensor = |name: u32| -> R<i64> {
        let (uid, info) = tensor_info(o.desc(name)?)?;
        tensors.insert(uid, info);
        Ok(uid)
    };
    Ok(Ok(match o.kind {
        kind::CONV_FWD | kind::CONV_BWD_DATA | kind::CONV_BWD_FILTER => {
            let (dir, base) = match o.kind {
                kind::CONV_FWD => (0, attr::CONV_FWD),
                kind::CONV_BWD_DATA => (1, attr::CONV_BWD_DATA),
                _ => (2, attr::CONV_BWD_FILTER),
            };
            let alpha = o.f64_(base).unwrap_or(1.0) as f32;
            let beta = o.f64_(base + 1).unwrap_or(0.0) as f32;
            let c = o.desc(base + 2)?;
            // the three tensors in attribute order
            let (t3, t4, t5) = (tensor(base + 3)?, tensor(base + 4)?, tensor(base + 5)?);
            // forward: w, x, y; backward data: w, dx, dy; backward filter: dw, x, dy
            let (x, w, y) = (t4, t3, t5);
            let spatial = c.i64s(attr::CONV_SPATIAL_DIMS).and_then(|v| v.first().copied()).unwrap_or(2);
            let get2 = |n: u32, default: i64| -> Option<[i64; 2]> {
                match c.i64s(n) {
                    None => Some([default; 2]),
                    Some(v) if v.len() == 2 => Some([v[0], v[1]]),
                    _ => None,
                }
            };
            let (Some(pre), Some(post), Some(stride), Some(dilation)) =
                (get2(attr::CONV_PRE_PADDINGS, 0), get2(attr::CONV_POST_PADDINGS, 0), get2(attr::CONV_FILTER_STRIDES, 1), get2(attr::CONV_DILATIONS, 1))
            else {
                return Ok(Err("only 2-D convolutions".into()));
            };
            if spatial != 2 {
                return Ok(Err("only 2-D convolutions".into()));
            }
            if pre != post {
                return Ok(Err("asymmetric convolution padding".into()));
            }
            let cross = c.u32_(attr::CONV_MODE).unwrap_or(1) == 1;
            Op::Conv { dir, x, w, y, pad: pre, stride, dilation, cross, alpha, beta }
        }
        kind::OP_POINTWISE => {
            let pw = o.desc(attr::OP_PW_DESC)?;
            let mode = pw.u32_(attr::PW_MODE).ok_or(cudnnError_t::BAD_PARAM)?;
            let backward = (200..300).contains(&mode);
            let x = tensor(attr::OP_PW_X)?;
            let (b, y) = if backward {
                (Some(tensor(attr::OP_PW_DY)?), tensor(attr::OP_PW_DX)?)
            } else {
                let b = if o.attr(attr::OP_PW_B).is_some() { Some(tensor(attr::OP_PW_B)?) } else { None };
                (b, tensor(attr::OP_PW_Y)?)
            };
            let t = if o.attr(attr::OP_PW_T).is_some() { Some(tensor(attr::OP_PW_T)?) } else { None };
            let f = |n: u32, d: f64| pw.f64_(n).unwrap_or(d) as f32;
            let p = match mode {
                100 | 200 => [f(attr::PW_RELU_LOWER_CLIP, 0.0), f(attr::PW_RELU_UPPER_CLIP, f32::MAX as f64), f(attr::PW_RELU_LOWER_CLIP_SLOPE, 0.0)],
                103 | 203 => [f(attr::PW_ELU_ALPHA, 1.0), 0.0, 0.0],
                105 | 205 => [f(attr::PW_SOFTPLUS_BETA, 1.0), 0.0, 0.0],
                106 | 206 => [f(attr::PW_SWISH_BETA, 1.0), 0.0, 0.0],
                _ => [0.0; 3],
            };
            if !SUPPORTED_PW.contains(&mode) {
                return Ok(Err(format!("pointwise mode {mode}")));
            }
            let alpha1 = o.f64_(attr::OP_PW_ALPHA1).unwrap_or(1.0) as f32;
            let alpha2 = o.f64_(attr::OP_PW_ALPHA2).unwrap_or(1.0) as f32;
            Op::Pointwise { mode, x, b, t, y, p, alpha1, alpha2 }
        }
        kind::OP_MATMUL => Op::Matmul { a: tensor(attr::OP_MATMUL_A)?, b: tensor(attr::OP_MATMUL_B)?, c: tensor(attr::OP_MATMUL_C)? },
        kind::OP_REDUCTION => {
            let r = o.desc(attr::OP_REDUCTION_DESC)?;
            let op = r.u32_(attr::REDUCTION_OPERATOR).ok_or(cudnnError_t::BAD_PARAM)?;
            Op::Reduction { op, x: tensor(attr::OP_REDUCTION_X)?, y: tensor(attr::OP_REDUCTION_Y)? }
        }
        k => return Ok(Err(format!("operation descriptor type {k}"))),
    }))
}

/// Why `g` cannot run, if it cannot.
pub fn check(g: &Graph) -> Option<String> {
    for (uid, t) in &g.tensors {
        if ![F32, F16, BF16].contains(&t.ty) {
            return Some(format!("tensor {uid}: data type {}", t.ty));
        }
        if t.dims.len() > 4 {
            return Some(format!("tensor {uid}: {} dimensions", t.dims.len()));
        }
    }
    for op in &g.ops {
        let t = |u: i64| &g.tensors[&u];
        match op {
            Op::Conv { x, w, y, .. } => {
                if [*x, *w, *y].iter().any(|u| t(*u).dims.len() != 4) {
                    return Some("convolutions take 4-D tensors".into());
                }
            }
            Op::Pointwise { x, b, t: tt, y, .. } => {
                let yi = t(*y);
                if [Some(*x), *b, *tt].into_iter().flatten().any(|u| !broadcastable(t(u), yi)) {
                    return Some("pointwise operands that do not broadcast".into());
                }
            }
            Op::Matmul { a, b, c } => {
                let (a, b, c) = (t(*a), t(*b), t(*c));
                if a.dims.len() != b.dims.len() || a.dims.len() != c.dims.len() || !(2..=3).contains(&a.dims.len()) {
                    return Some("matmul takes 2-D or 3-D tensors".into());
                }
                let n = a.dims.len();
                let (m, k, nn) = (a.dims[n - 2], a.dims[n - 1], b.dims[n - 1]);
                if b.dims[n - 2] != k || c.dims[n - 2] != m || c.dims[n - 1] != nn {
                    return Some("matmul shapes".into());
                }
                if n == 3 && !(a.dims[0] == c.dims[0] || a.dims[0] == 1) || n == 3 && !(b.dims[0] == c.dims[0] || b.dims[0] == 1) {
                    return Some("matmul batches".into());
                }
            }
            Op::Reduction { op, x, y } => {
                if *op > 8 {
                    return Some(format!("reduction operator {op}"));
                }
                let (px, py) = (pad4(&t(*x).dims), pad4(&t(*y).dims));
                if t(*x).dims.len() != t(*y).dims.len() || px.iter().zip(&py).any(|(a, b)| a != b && *b != 1) {
                    return Some("reduction shapes".into());
                }
            }
        }
    }
    None
}

/// The operations in dependency order (producers first).
pub fn order(ops: Vec<Op>) -> Option<Vec<Op>> {
    let mut left = ops;
    let mut done: Vec<Op> = Vec::new();
    while !left.is_empty() {
        let produced: Vec<i64> = left.iter().map(Op::output).collect();
        let pos = left.iter().position(|o| o.inputs().iter().all(|i| !produced.contains(i)))?;
        done.push(left.remove(pos));
    }
    Some(done)
}

unsafe fn build_graph(d: &Desc) -> R<Graph> {
    let mut tensors = HashMap::new();
    let mut ops = Vec::new();
    let mut unsupported = None;
    for o in d.descs(attr::GRAPH_OPS)? {
        match read_op(o, &mut tensors)? {
            Ok(op) => ops.push(op),
            Err(why) => unsupported = unsupported.or(Some(why)),
        }
    }
    if ops.is_empty() && unsupported.is_none() {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let ops = match order(ops.clone()) {
        Some(o) => o,
        None => {
            unsupported = unsupported.or(Some("a cycle".into()));
            ops
        }
    };
    let mut g = Graph { tensors, ops, unsupported };
    if g.unsupported.is_none() {
        g.unsupported = check(&g);
    }
    Ok(g)
}

/// The workspace slots: packed f32 tensors of the variant pack in place, everything else in the workspace.
pub fn layout(g: &Graph) -> Plan {
    let mut slots = HashMap::new();
    let mut off = 0usize;
    let mut uids: Vec<&i64> = g.tensors.keys().collect();
    uids.sort();
    for uid in uids {
        let t = &g.tensors[uid];
        if !t.virt && !t.by_value && t.ty == F32 && t.packed() {
            slots.insert(*uid, Slot::Direct);
        } else {
            slots.insert(*uid, Slot::Workspace(off));
            off += (4 * t.elements() as usize).div_ceil(256) * 256;
        }
    }
    Plan { graph: g.clone(), slots, workspace: off }
}

pub unsafe fn finalize(d: &mut Desc) -> R<()> {
    match d.kind {
        kind::TENSOR => {
            tensor_info(d)?;
        }
        kind::OPERATIONGRAPH => d.graph = Some(build_graph(d)?),
        kind::ENGINEHEUR => {
            d.desc(attr::HEUR_GRAPH)?.graph.as_ref().ok_or(cudnnError_t::BAD_PARAM)?;
        }
        kind::ENGINE => {
            let g = d.desc(attr::ENGINE_GRAPH)?.graph.clone().ok_or(cudnnError_t::BAD_PARAM)?;
            if d.i64s(attr::ENGINE_GLOBAL_INDEX).and_then(|v| v.first().copied()).unwrap_or(0) != 0 {
                return Err(cudnnError_t::NOT_SUPPORTED);
            }
            if g.unsupported.is_some() {
                return Err(cudnnError_t::NOT_SUPPORTED);
            }
            d.graph = Some(g);
        }
        kind::ENGINECFG => {
            d.graph = Some(d.desc(attr::CFG_ENGINE)?.graph.clone().ok_or(cudnnError_t::BAD_PARAM)?);
        }
        kind::EXECUTION_PLAN => {
            let g = d.desc(attr::PLAN_ENGINE_CONFIG)?.graph.clone().ok_or(cudnnError_t::BAD_PARAM)?;
            d.plan = Some(layout(&g));
        }
        kind::VARIANT_PACK => {
            if d.ptrs(attr::VP_UIDS).len() != d.ptrs(attr::VP_POINTERS).len() {
                return Err(cudnnError_t::BAD_PARAM);
            }
        }
        _ => {}
    }
    d.finalized = true;
    Ok(())
}

/// Writes `values` (of `ty` elements) to a GetAttribute's output.
unsafe fn give(ty: u32, values: &[u8], n: i64, req: i64, count: *mut i64, out: *mut c_void) -> R<()> {
    if let Some(c) = count.as_mut() {
        *c = n;
    }
    let size = elem_size(ty).ok_or(cudnnError_t::BAD_PARAM)?;
    if !out.is_null() && req > 0 {
        let k = (n.min(req) as usize) * size;
        std::ptr::copy_nonoverlapping(values.as_ptr(), out as *mut u8, k);
    }
    Ok(())
}

unsafe fn give_i64(v: i64, req: i64, count: *mut i64, out: *mut c_void) -> R<()> {
    give(TYPE_INT64, &v.to_le_bytes(), 1, req, count, out)
}

/// Copies `src` into the caller's descriptors `out` (backend descriptors are given back by value).
unsafe fn give_descs(src: &[&Desc], req: i64, count: *mut i64, out: *mut c_void) -> R<()> {
    if let Some(c) = count.as_mut() {
        *c = src.len() as i64;
    }
    if out.is_null() {
        return Ok(());
    }
    let outs = std::slice::from_raw_parts(out as *const *mut Desc, req.max(0) as usize);
    for (dst, s) in outs.iter().zip(src) {
        let dst = dst.as_mut().ok_or(cudnnError_t::BAD_PARAM)?;
        if dst.kind != s.kind {
            return Err(cudnnError_t::BAD_PARAM);
        }
        *dst = (*s).clone();
    }
    Ok(())
}

pub unsafe fn get_attribute(d: &mut Desc, name: u32, ty: u32, req: i64, count: *mut i64, out: *mut c_void) -> R<()> {
    match (d.kind, name) {
        (kind::ENGINEHEUR, attr::HEUR_RESULTS) => {
            let g = d.desc(attr::HEUR_GRAPH)?.graph.clone().ok_or(cudnnError_t::NOT_INITIALIZED)?;
            let graph_ptr = d.ptrs(attr::HEUR_GRAPH)[0];
            if g.unsupported.is_some() {
                return give_descs(&[], req, count, out);
            }
            // one configuration of engine 0
            let mut engine = Desc { kind: kind::ENGINE, finalized: true, graph: Some(g.clone()), ..Default::default() };
            engine.set_raw(attr::ENGINE_GRAPH, TYPE_BACKEND_DESCRIPTOR, 1, graph_ptr.to_le_bytes().to_vec());
            engine.set_raw(attr::ENGINE_GLOBAL_INDEX, TYPE_INT64, 1, 0i64.to_le_bytes().to_vec());
            let engine = Box::new(engine);
            let mut cfg = Desc { kind: kind::ENGINECFG, finalized: true, graph: Some(g), ..Default::default() };
            cfg.set_raw(attr::CFG_ENGINE, TYPE_BACKEND_DESCRIPTOR, 1, (&*engine as *const Desc as usize).to_le_bytes().to_vec());
            d.owned.push(engine);
            give_descs(&[&cfg], req, count, out)?;
            // the copy's engine must outlive it: the caller's configuration owns one too
            if !out.is_null() && req > 0 {
                let dst = &mut **(out as *const *mut Desc);
                let own = Box::new((*d.owned.last().unwrap().as_ref()).clone());
                dst.set_raw(attr::CFG_ENGINE, TYPE_BACKEND_DESCRIPTOR, 1, (&*own as *const Desc as usize).to_le_bytes().to_vec());
                dst.owned.push(own);
            }
            Ok(())
        }
        (kind::OPERATIONGRAPH, attr::GRAPH_ENGINE_COUNT) => give_i64(1, req, count, out),
        (kind::ENGINE, attr::ENGINE_GLOBAL_INDEX) => give_i64(d.i64s(name).and_then(|v| v.first().copied()).unwrap_or(0), req, count, out),
        (kind::ENGINE, attr::ENGINE_KNOB_INFO | attr::ENGINE_NUMERICAL_NOTE | attr::ENGINE_BEHAVIOR_NOTE | attr::ENGINE_LAYOUT_INFO) => {
            give(ty, &[], 0, req, count, out)
        }
        (kind::ENGINECFG, attr::CFG_KNOB_CHOICES | attr::CFG_INTERMEDIATE_INFO) if d.attr(name).is_none() => give(ty, &[], 0, req, count, out),
        (kind::ENGINECFG, attr::CFG_WORKSPACE_SIZE) => {
            let w = d.graph.as_ref().map_or(0, |g| layout(g).workspace);
            give_i64(w as i64, req, count, out)
        }
        (kind::ENGINECFG, attr::CFG_SHARED_MEMORY_USED) => give_i64(0, req, count, out),
        (kind::EXECUTION_PLAN, attr::PLAN_WORKSPACE_SIZE) => give_i64(d.plan.as_ref().ok_or(cudnnError_t::NOT_INITIALIZED)?.workspace as i64, req, count, out),
        (kind::EXECUTION_PLAN, attr::PLAN_COMPUTED_INTERMEDIATE_UIDS | attr::PLAN_RUN_ONLY_INTERMEDIATE_UIDS) => give(ty, &[], 0, req, count, out),
        (kind::EXECUTION_PLAN, attr::PLAN_JSON) => {
            let json = b"{\"engine\":0,\"knobs\":[]}\0";
            give(TYPE_CHAR, json, json.len() as i64, req, count, out)
        }
        (kind::TENSOR, attr::TENSOR_BYTE_ALIGNMENT) if d.attr(name).is_none() => give_i64(16, req, count, out),
        (kind::TENSOR, attr::TENSOR_VECTOR_COUNT) if d.attr(name).is_none() => give_i64(1, req, count, out),
        (kind::TENSOR, attr::TENSOR_VECTORIZED_DIMENSION) if d.attr(name).is_none() => give_i64(-1, req, count, out),
        (kind::TENSOR, attr::TENSOR_IS_VIRTUAL | attr::TENSOR_IS_BY_VALUE) if d.attr(name).is_none() => give(TYPE_BOOLEAN, &[0], 1, req, count, out),
        _ => {
            let a = d.attr(name).ok_or(cudnnError_t::BAD_PARAM)?.clone();
            if a.ty == TYPE_BACKEND_DESCRIPTOR {
                let src = d.descs(name)?;
                return give_descs(&src, req, count, out);
            }
            if ty != a.ty && !(ty == TYPE_DOUBLE && a.ty == TYPE_FLOAT) {
                return Err(cudnnError_t::BAD_PARAM);
            }
            give(a.ty, &a.data, a.count, req, count, out)
        }
    }
}

// ---- execution

/// The 4-D view of a packed f32 tensor of `dims`.
fn packed_view(dims: &[i64]) -> View {
    let d = pad4(dims);
    let mut s = [1; 4];
    for i in (0..3).rev() {
        s[i] = s[i + 1] * d[i + 1];
    }
    View { dims: d, strides: s }
}

/// The user's 4-D view of a tensor.
fn user_view(t: &TensorInfo) -> View {
    let d = pad4(&t.dims);
    let mut s = [0; 4];
    let off = 4 - t.dims.len();
    for (i, &st) in t.strides.iter().enumerate() {
        s[off + i] = st;
    }
    View { dims: d, strides: s }
}

/// y = x through the pointwise kernel (identity), with any strides.
unsafe fn copy_f32(h: &Handle, x: *mut c_void, xv: &View, y: *mut c_void, yv: &View) -> R<()> {
    pw(h, 21, x, xv.strides, None, None, y, yv, [0.0; 3], 1.0, 1.0)
}

#[allow(clippy::too_many_arguments)]
unsafe fn pw(
    h: &Handle,
    mode: u32,
    mut x: *mut c_void,
    xs: [i64; 4],
    b: Option<(*mut c_void, [i64; 4])>,
    t: Option<(*mut c_void, [i64; 4])>,
    mut y: *mut c_void,
    yv: &View,
    p: [f32; 3],
    mut alpha1: f32,
    mut alpha2: f32,
) -> R<()> {
    // unused operands read x with no stride
    let (mut b, bs) = b.unwrap_or((x, [0; 4]));
    let (mut t, ts) = t.unwrap_or((x, [0; 4]));
    let mut total = yv.total();
    let mut a = Args::shape(yv);
    a.0.extend(xs);
    a.0.extend(bs);
    a.0.extend(ts);
    let mut a = a.strides(yv);
    a.push(mode as i64);
    let [mut p0, mut p1, mut p2] = p;
    let mut params = vec![arg(&mut x), arg(&mut b), arg(&mut t), arg(&mut y), arg(&mut total)];
    params.extend(a.0.iter_mut().map(|v| arg(v)));
    params.extend([arg(&mut p0), arg(&mut p1), arg(&mut p2), arg(&mut alpha1), arg(&mut alpha2)]);
    dnn_launch(h, c"agx_dnn_pw_graph", &mut params, total)
}

/// The f32 conversion of a tensor of type `ty` between `src` and `dst` (`into` f32 or out of it).
unsafe fn convert_tensor(h: &Handle, ty: u32, into: bool, user: *mut c_void, uv: &View, f: *mut c_void, fv: &View) -> R<()> {
    let kind = match ty {
        F16 => 0,
        BF16 => 1,
        _ => return if into { copy_f32(h, user, uv, f, fv) } else { copy_f32(h, f, fv, user, uv) },
    };
    if into {
        convert(h, c"agx_dnn_to_f32", user, uv, f, fv, kind)
    } else {
        convert(h, c"agx_dnn_from_f32", f, fv, user, uv, kind)
    }
}

fn i32s(v: &[i64]) -> Vec<i32> {
    v.iter().map(|&x| x as i32).collect()
}

pub unsafe fn execute(hraw: cudnnHandle_t, plan: &Desc, vp: &Desc) -> R<()> {
    let h = obj::<Handle>(hraw)?;
    let p = plan.plan.as_ref().ok_or(cudnnError_t::BAD_PARAM)?;
    let uids = vp.i64s(attr::VP_UIDS).unwrap_or_default();
    let ptrs = vp.ptrs(attr::VP_POINTERS);
    let user: HashMap<i64, usize> = uids.into_iter().zip(ptrs).collect();
    let ws = vp.ptrs(attr::VP_WORKSPACE).first().copied().unwrap_or(0);
    if p.workspace > 0 && ws == 0 {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let g = &p.graph;
    let f32_ptr = |uid: i64| -> R<*mut c_void> {
        match p.slots[&uid] {
            Slot::Direct => user.get(&uid).map(|&a| a as *mut c_void).ok_or(cudnnError_t::BAD_PARAM),
            Slot::Workspace(o) => Ok((ws + o) as *mut c_void),
        }
    };
    let produced: Vec<i64> = g.ops.iter().map(Op::output).collect();
    // inputs (and outputs, which a beta may read) into their f32 slots
    let mut uids: Vec<&i64> = g.tensors.keys().collect();
    uids.sort();
    for &uid in &uids {
        let t = &g.tensors[uid];
        if t.virt || p.slots[uid] == Slot::Direct {
            continue;
        }
        let up = *user.get(uid).ok_or(cudnnError_t::BAD_PARAM)?;
        if t.by_value {
            // a host scalar
            let v = match t.ty {
                F32 => *(up as *const f32),
                F16 => f16_to_f32(*(up as *const u16)),
                _ => f32::from_bits((*(up as *const u16) as u32) << 16),
            };
            engine::write(f32_ptr(*uid)?, &v as *const f32 as *const c_void, 4).map_err(status)?;
            continue;
        }
        convert_tensor(h, t.ty, true, up as *mut c_void, &user_view(t), f32_ptr(*uid)?, &packed_view(&t.dims))?;
    }
    for op in &g.ops {
        let t = |u: i64| &g.tensors[&u];
        match *op {
            Op::Conv { dir, x, w, y, pad, stride, dilation, cross, alpha, beta } => {
                let f32_ty = cudnnDataType_t::CUDNN_DATA_FLOAT.0;
                let tx = Tensor { ty: f32_ty, dims: i32s(&t(x).dims), strides: packed_strides(&i32s(&t(x).dims), false) };
                let ty_ = Tensor { ty: f32_ty, dims: i32s(&t(y).dims), strides: packed_strides(&i32s(&t(y).dims), false) };
                let tw = Filter { ty: f32_ty, format: cudnnTensorFormat_t::CUDNN_TENSOR_NCHW.0, dims: i32s(&t(w).dims) };
                let groups = (t(x).dims[1] / t(w).dims[1]) as i32;
                let c = Conv {
                    pad: [pad[0] as i32, pad[1] as i32],
                    stride: [stride[0] as i32, stride[1] as i32],
                    dilation: [dilation[0] as i32, dilation[1] as i32],
                    mode: cross as u32,
                    groups,
                    ..Conv::default()
                };
                let s = kernel_shape(&tx, &tw, &c, &ty_)?;
                let (xp, wp, yp) = (f32_ptr(x)?, f32_ptr(w)?, f32_ptr(y)?);
                let (a, b) = (alpha, beta);
                match dir {
                    0 => run(
                        hraw,
                        "fwd",
                        s,
                        false,
                        s[0] as u64 * s[4] as u64 * s[7] as u64 * s[8] as u64,
                        xp,
                        wp,
                        yp,
                        &a as *const f32 as _,
                        &b as *const f32 as _,
                    )?,
                    1 => run(
                        hraw,
                        "bwd_data",
                        s,
                        false,
                        s[0] as u64 * s[1] as u64 * s[2] as u64 * s[3] as u64,
                        xp,
                        wp,
                        yp,
                        &a as *const f32 as _,
                        &b as *const f32 as _,
                    )?,
                    _ => run(
                        hraw,
                        "bwd_filter",
                        s,
                        false,
                        s[4] as u64 * (s[1] / s[15]) as u64 * s[5] as u64 * s[6] as u64,
                        xp,
                        wp,
                        yp,
                        &a as *const f32 as _,
                        &b as *const f32 as _,
                    )?,
                }
            }
            Op::Pointwise { mode, x, b, t: tt, y, p: params, alpha1, alpha2 } => {
                let yv = packed_view(&t(y).dims);
                let strides = |u: i64| bstrides(&pad4(&t(u).dims));
                let bo = match b {
                    Some(u) => Some((f32_ptr(u)?, strides(u))),
                    None => None,
                };
                let to = match tt {
                    Some(u) => Some((f32_ptr(u)?, strides(u))),
                    None => None,
                };
                pw(h, mode, f32_ptr(x)?, strides(x), bo, to, f32_ptr(y)?, &yv, params, alpha1, alpha2)?;
            }
            Op::Matmul { a, b, c } => {
                let (ta, tb, tc) = (t(a), t(b), t(c));
                let n = ta.dims.len();
                let (m, k, nn) = (ta.dims[n - 2], ta.dims[n - 1], tb.dims[n - 1]);
                let (batch, ba, bb) = if n == 3 { (tc.dims[0], ta.dims[0], tb.dims[0]) } else { (1, 1, 1) };
                // row-major C = A B is column-major C^T = B^T A^T
                let gm = engine::Gemm {
                    types: agx_libs::Types::S,
                    ta: false,
                    tb: false,
                    m: nn as u64,
                    n: m as u64,
                    k: k as u64,
                    alpha: 1.0,
                    beta: 0.0,
                    a: f32_ptr(b)?,
                    lda: nn as u64,
                    stride_a: if bb == 1 { 0 } else { k * nn },
                    b: f32_ptr(a)?,
                    ldb: k as u64,
                    stride_b: if ba == 1 { 0 } else { m * k },
                    c: f32_ptr(c)?,
                    ldc: nn as u64,
                    stride_c: m * nn,
                    batch: batch as u64,
                };
                engine::gemm(&gm, h.stream).map_err(status)?;
            }
            Op::Reduction { op, x, y } => {
                let (px, py) = (pad4(&t(x).dims), pad4(&t(y).dims));
                let (xv, yv) = (packed_view(&t(x).dims), packed_view(&t(y).dims));
                let mut e = [1i64; 4];
                for i in 0..4 {
                    if py[i] == 1 && px[i] > 1 {
                        e[i] = px[i];
                    }
                }
                let (mut xp, mut yp) = (f32_ptr(x)?, f32_ptr(y)?);
                let mut total = yv.total();
                let mut a = Args::shape(&yv).strides(&xv).strides(&yv);
                a.0.extend(e);
                a.push(op as i64);
                let mut params = vec![arg(&mut xp), arg(&mut yp), arg(&mut total)];
                params.extend(a.0.iter_mut().map(|v| arg(v)));
                dnn_launch(h, c"agx_dnn_reduce_graph", &mut params, total)?;
            }
        }
    }
    // outputs back
    for &uid in &uids {
        let t = &g.tensors[uid];
        if t.virt || t.by_value || p.slots[uid] == Slot::Direct || !produced.contains(uid) {
            continue;
        }
        let up = *user.get(uid).ok_or(cudnnError_t::BAD_PARAM)?;
        convert_tensor(h, t.ty, false, up as *mut c_void, &user_view(t), f32_ptr(*uid)?, &packed_view(&t.dims))?;
    }
    Ok(())
}

fn f16_to_f32(h: u16) -> f32 {
    agx_libs::f16_to_f32(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn set(d: &mut Desc, name: u32, ty: u32, v: &[i64]) {
        set_attribute(d, name, ty, v.len() as i64, v.as_ptr() as *const c_void).unwrap();
    }

    unsafe fn tensor(uid: i64, dims: &[i64], virt: bool) -> Box<Desc> {
        let mut t = create(kind::TENSOR).unwrap();
        set(&mut t, attr::TENSOR_UNIQUE_ID, TYPE_INT64, &[uid]);
        set(&mut t, attr::TENSOR_DIMENSIONS, TYPE_INT64, dims);
        let st = bstrides(&pad4(dims));
        let mut s: Vec<i64> = st[4 - dims.len()..].to_vec();
        // packed strides even for dimensions of 1
        let mut acc = 1;
        for i in (0..dims.len()).rev() {
            s[i] = acc;
            acc *= dims[i];
        }
        set(&mut t, attr::TENSOR_STRIDES, TYPE_INT64, &s);
        let ty = [F32 as i32];
        set_attribute(&mut t, attr::TENSOR_DATA_TYPE, TYPE_DATA_TYPE, 1, ty.as_ptr() as *const c_void).unwrap();
        let v = [virt as u8];
        set_attribute(&mut t, attr::TENSOR_IS_VIRTUAL, TYPE_BOOLEAN, 1, v.as_ptr() as *const c_void).unwrap();
        finalize(&mut t).unwrap();
        t
    }

    fn ptr(d: &Desc) -> i64 {
        d as *const Desc as i64
    }

    #[test]
    fn matmul_bias_relu_graph() {
        unsafe {
            let (a, b, c, bias, out) = (
                tensor(1, &[2, 3, 4], false),
                tensor(2, &[2, 4, 5], false),
                tensor(3, &[2, 3, 5], true),
                tensor(4, &[1, 1, 5], false),
                tensor(5, &[2, 3, 5], false),
            );
            let mut mm = create(kind::OP_MATMUL).unwrap();
            set(&mut mm, attr::OP_MATMUL_A, TYPE_BACKEND_DESCRIPTOR, &[ptr(&a)]);
            set(&mut mm, attr::OP_MATMUL_B, TYPE_BACKEND_DESCRIPTOR, &[ptr(&b)]);
            set(&mut mm, attr::OP_MATMUL_C, TYPE_BACKEND_DESCRIPTOR, &[ptr(&c)]);
            finalize(&mut mm).unwrap();
            let mut pwd = create(kind::POINTWISE).unwrap();
            let mode = [0u32];
            set_attribute(&mut pwd, attr::PW_MODE, 14, 1, mode.as_ptr() as *const c_void).unwrap();
            finalize(&mut pwd).unwrap();
            let mut add = create(kind::OP_POINTWISE).unwrap();
            set(&mut add, attr::OP_PW_DESC, TYPE_BACKEND_DESCRIPTOR, &[ptr(&pwd)]);
            set(&mut add, attr::OP_PW_X, TYPE_BACKEND_DESCRIPTOR, &[ptr(&c)]);
            set(&mut add, attr::OP_PW_B, TYPE_BACKEND_DESCRIPTOR, &[ptr(&bias)]);
            set(&mut add, attr::OP_PW_Y, TYPE_BACKEND_DESCRIPTOR, &[ptr(&out)]);
            finalize(&mut add).unwrap();
            let mut g = create(kind::OPERATIONGRAPH).unwrap();
            // given out of order: the graph sorts them
            set(&mut g, attr::GRAPH_OPS, TYPE_BACKEND_DESCRIPTOR, &[ptr(&add), ptr(&mm)]);
            finalize(&mut g).unwrap();
            let gr = g.graph.clone().unwrap();
            assert_eq!(gr.unsupported, None);
            assert!(matches!(gr.ops[0], Op::Matmul { a: 1, b: 2, c: 3 }));
            // heuristics give one configuration; a plan lays out the virtual tensor in the workspace
            let mut heur = create(kind::ENGINEHEUR).unwrap();
            set(&mut heur, attr::HEUR_GRAPH, TYPE_BACKEND_DESCRIPTOR, &[ptr(&g)]);
            finalize(&mut heur).unwrap();
            let mut cfg = create(kind::ENGINECFG).unwrap();
            let mut n = 0;
            let mut outs = [&mut *cfg as *mut Desc];
            get_attribute(&mut heur, attr::HEUR_RESULTS, TYPE_BACKEND_DESCRIPTOR, 1, &mut n, outs.as_mut_ptr() as *mut c_void).unwrap();
            assert_eq!(n, 1);
            let mut eng = create(kind::ENGINE).unwrap();
            let mut e_out = [&mut *eng as *mut Desc];
            get_attribute(&mut cfg, attr::CFG_ENGINE, TYPE_BACKEND_DESCRIPTOR, 1, &mut n, e_out.as_mut_ptr() as *mut c_void).unwrap();
            let mut idx = -1i64;
            get_attribute(&mut eng, attr::ENGINE_GLOBAL_INDEX, TYPE_INT64, 1, &mut n, &mut idx as *mut i64 as *mut c_void).unwrap();
            assert_eq!(idx, 0);
            let mut plan = create(kind::EXECUTION_PLAN).unwrap();
            set(&mut plan, attr::PLAN_ENGINE_CONFIG, TYPE_BACKEND_DESCRIPTOR, &[ptr(&cfg)]);
            finalize(&mut plan).unwrap();
            let mut ws = 0i64;
            get_attribute(&mut plan, attr::PLAN_WORKSPACE_SIZE, TYPE_INT64, 1, &mut n, &mut ws as *mut i64 as *mut c_void).unwrap();
            assert_eq!(ws, 256);
            let p = plan.plan.as_ref().unwrap();
            assert_eq!(p.slots[&3], Slot::Workspace(0));
            assert_eq!(p.slots[&1], Slot::Direct);
        }
    }

    #[test]
    fn unsupported_graphs_have_no_engine() {
        unsafe {
            let (x, y) = (tensor(1, &[2, 3], false), tensor(2, &[2, 3], false));
            let mut pwd = create(kind::POINTWISE).unwrap();
            let mode = [23u32];
            set_attribute(&mut pwd, attr::PW_MODE, 14, 1, mode.as_ptr() as *const c_void).unwrap();
            finalize(&mut pwd).unwrap();
            let mut op = create(kind::OP_POINTWISE).unwrap();
            set(&mut op, attr::OP_PW_DESC, TYPE_BACKEND_DESCRIPTOR, &[ptr(&pwd)]);
            set(&mut op, attr::OP_PW_X, TYPE_BACKEND_DESCRIPTOR, &[ptr(&x)]);
            set(&mut op, attr::OP_PW_Y, TYPE_BACKEND_DESCRIPTOR, &[ptr(&y)]);
            finalize(&mut op).unwrap();
            let mut g = create(kind::OPERATIONGRAPH).unwrap();
            set(&mut g, attr::GRAPH_OPS, TYPE_BACKEND_DESCRIPTOR, &[ptr(&op)]);
            finalize(&mut g).unwrap();
            assert!(g.graph.as_ref().unwrap().unsupported.is_some());
            let mut heur = create(kind::ENGINEHEUR).unwrap();
            set(&mut heur, attr::HEUR_GRAPH, TYPE_BACKEND_DESCRIPTOR, &[ptr(&g)]);
            finalize(&mut heur).unwrap();
            let mut n = 5;
            get_attribute(&mut heur, attr::HEUR_RESULTS, TYPE_BACKEND_DESCRIPTOR, 0, &mut n, std::ptr::null_mut()).unwrap();
            assert_eq!(n, 0);
        }
    }

    #[test]
    fn strides_and_views() {
        assert_eq!(pad4(&[3, 5]), [1, 1, 3, 5]);
        assert_eq!(bstrides(&[2, 1, 4, 1]), [4, 0, 1, 0]);
        let t = TensorInfo { ty: F32, dims: vec![2, 3, 4], strides: vec![12, 4, 1], virt: false, by_value: false };
        assert!(t.packed());
        assert!(!TensorInfo { strides: vec![1, 2, 6], ..t.clone() }.packed());
        assert_eq!(user_view(&t), View { dims: [1, 2, 3, 4], strides: [0, 12, 4, 1] });
        assert_eq!(order(vec![Op::Reduction { op: 0, x: 2, y: 3 }, Op::Reduction { op: 0, x: 1, y: 2 }]).unwrap()[0], Op::Reduction { op: 0, x: 1, y: 2 });
    }
}
