
extern crate ndarray;
extern crate ndarray_rand;

use ndarray::prelude::*;
use linalg::SquareMatrix;

mod common;
use common::*;

#[test]
fn eigen_vector_manual() {
    let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
    let (e, vecs) = a.clone().eigh().unwrap();
    assert!(e.all_close(&arr1(&[2.0, 2.0, 5.0]), 1.0e-7));
    for (i, v) in vecs.axis_iter(Axis(1)).enumerate() {
        let av = a.dot(&v);
        let ev = v.mapv(|x| e[i] * x);
        assert!(av.all_close(&ev, 1.0e-7));
    }
}

#[test]
fn diagonalize() {
    let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
    let (e, vecs) = a.clone().eigh().unwrap();
    let s = vecs.t().dot(&a).dot(&vecs);
    for i in 0..3 {
        is_close(e[i], s[(i, i)]);
    }
}
