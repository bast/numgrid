use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::atom;
use crate::lebedev;
use crate::radial;

#[pyfunction]
fn angular_grid(num_points: usize) -> (Vec<(f64, f64, f64)>, Vec<f64>) {
    lebedev::angular_grid(num_points)
}

#[pyfunction]
fn radial_grid(
    alpha_min: Vec<f64>,
    alpha_max: f64,
    max_l_quantum_number: usize,
    radial_precision: f64,
    proton_charge: i32,
) -> (Vec<f64>, Vec<f64>) {
    radial::radial_grid(
        alpha_min,
        alpha_max,
        max_l_quantum_number,
        radial_precision,
        proton_charge,
    )
}

#[pyfunction]
fn atom_grid_bse(
    basis_set: &str,
    radial_precision: f64,
    min_num_angular_points: usize,
    max_num_angular_points: usize,
    proton_charges: Vec<i32>,
    center_index: usize,
    center_coordinates_bohr: Vec<(f64, f64, f64)>,
    hardness: usize,
) -> (Vec<(f64, f64, f64)>, Vec<f64>) {
    atom::atom_grid_bse(
        basis_set,
        radial_precision,
        min_num_angular_points,
        max_num_angular_points,
        proton_charges,
        center_index,
        center_coordinates_bohr,
        hardness,
    )
}

#[pyfunction]
fn atom_grid(
    alpha_min: Vec<f64>,
    alpha_max: f64,
    max_l_quantum_number: usize,
    radial_precision: f64,
    min_num_angular_points: usize,
    max_num_angular_points: usize,
    proton_charges: Vec<i32>,
    center_index: usize,
    center_coordinates_bohr: Vec<(f64, f64, f64)>,
    hardness: usize,
) -> (Vec<(f64, f64, f64)>, Vec<f64>) {
    atom::atom_grid(
        alpha_min,
        alpha_max,
        max_l_quantum_number,
        radial_precision,
        min_num_angular_points,
        max_num_angular_points,
        proton_charges,
        center_index,
        center_coordinates_bohr,
        hardness,
    )
}

#[pymodule]
fn numgrid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(angular_grid, m)?)?;
    m.add_function(wrap_pyfunction!(radial_grid, m)?)?;
    m.add_function(wrap_pyfunction!(atom_grid_bse, m)?)?;
    m.add_function(wrap_pyfunction!(atom_grid, m)?)?;

    Ok(())
}