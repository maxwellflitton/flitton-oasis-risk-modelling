use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyDict, PyAny};

mod footprint;
mod vulnerabilities;

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerabilities_with_footprint;
use vulnerabilities::structs::VulnerabilityFootPrint;


#[derive(Debug)]
enum Value {
    Float(f32),
    Int(i32),
}

fn main() {
    let mut map = HashMap::new();

    map.insert("a", Value::Str("1"));
    map.insert("b", Value::Int(2));

    for (key, value) in &map {
        println!("{}: {:?}", key, value);
    }
}


#[pyfunction]
fn get_model<'a>(event_ids: Vec<i32>, mut base_path: String) -> Vec<HashMap<String, Value>> {
    let footprints = merge_event_ids_with_footprint(event_ids, base_path.clone());
    let model = merge_vulnerabilities_with_footprint(footprints, base_path);

   let mut buffer = Vec::new();
   
   for i in model {
       let mut placeholder = HashMap::new();
       placeholder.insert(String::from("areaperil_id"), Value::Int(i.areaperil_id));
       buffer.push(placeholder);
   }
   return buffer
}

#[pymodule]
fn flitton_oasis_risk_modelling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_model));
    Ok(())
}