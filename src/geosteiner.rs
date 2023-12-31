use std::os::raw::{
    c_int,
    c_double,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Edge {
    p1: i32, // number of point 1
    p2: i32, // number of point 2
}

impl Edge {
    pub fn new(p1: i32, p2: i32) -> Self { Self { p1, p2 } }
}

/// A C struct that contains information on a euclidian steiner tree.
/// Because it is returned from a C function, it is very problematic to use directly.
/// Please us the rust representation `ESMT` instead.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_ESMT {
    pub length: c_double,
    pub nsps: c_int,
    pub sps: *const c_double,
    pub nedges: c_int,
    pub edges: *const c_int,
}

/// Contains information on a euclidian steiner tree.
#[derive(Clone, Debug, Serialize)]
pub struct ESMT {
    pub sps: Vec<Point>,
    pub edges: Vec<Edge>
}

impl ESMT {

    pub fn from_c_esmt(c_esmt: C_ESMT) -> Self {
        
        let mut sps = Vec::new();
        {
            let v = unsafe { std::slice::from_raw_parts(c_esmt.sps, (c_esmt.nsps*2) as usize) }.to_vec();

            for i in 0..c_esmt.nsps {
                sps.push(Point::new(v[2*i as usize], v[(2*i+1) as usize]));
            }
        }
        
        let mut edges = Vec:: new();
        {
            let v = unsafe { std::slice::from_raw_parts(c_esmt.edges, (c_esmt.nedges*2) as usize) }.to_vec();

            for i in 0..c_esmt.nedges {
                edges.push(Edge::new(v[2*i as usize], v[(2*i+1) as usize]));
            }
        }
        Self { sps, edges }
    }
}

extern "C" {
    pub fn rs_compute_esmt (nterms: c_int, terms: *const c_double) -> C_ESMT;
}

/// TODO replace L2 with L*2 when generic paramters are allowed in const expressions in stable rust.
pub fn rs_safe_compute_esmt<const L: usize, const L2: usize>(nterms: i32, terms: &[Point; L]) -> ESMT {

    let mut c_terms = [0.0; L2];
    for (i, p) in terms.iter().enumerate() {
        c_terms[i*2] = p.x;
        c_terms[i*2+1] = p.y;
    }

    let c_esmt = unsafe { rs_compute_esmt(nterms, &c_terms[0]) };
    ESMT::from_c_esmt(c_esmt)
}

/// Works the same as `rs_safe_compute_esmt` but takes a vector instead.       
pub fn rs_safe_compute_esmt_vec(terms: &Vec<Point>) -> ESMT {
    let mut c_terms = Vec::new();
    for p in terms {
        c_terms.push(p.x);
        c_terms.push(p.y);
    }

    let c_esmt = unsafe { rs_compute_esmt((c_terms.len() / 2) as i32, &c_terms[0]) };
    ESMT::from_c_esmt(c_esmt)
}
