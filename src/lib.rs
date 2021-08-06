use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyDict, PyAny};

mod footprint;
mod vulnerabilities;

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerabilities_with_footprint;
use vulnerabilities::structs::VulnerabilityFootPrint


#[pyfunction]
fn get_model<'a>(event_ids: Vec<i32>) -> Vec<VulnerabilityFootPrint> {
   let footprints = merge_event_ids_with_footprint(event_ids);
   let model = merge_vulnerabilities_with_footprint(footprints);

//    let gil = Python::acquire_gil();
//    let py = gil.python();
//    let mut buffer = Vec::new();
   
//    for i in model {
//        let placeholder = PyDict::new(py);
//        placeholder.set_item("areaperil_id", i.areaperil_id);
//        buffer.push(placeholder);
//    }
   return model
}

#[pymodule]
fn flitton_oasis_risk_modelling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_model));
    Ok(())
}