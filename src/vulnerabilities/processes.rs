use std::error::Error;
use std::fs::File;
use csv;

use crate::footprint::structs::FootPrint;

use super::structs::{Vulnerability, VulnerabilityFootPrint};


pub fn read_vulnerabilities() -> Result<Vec<Vulnerability>, Box<dyn Error>> {
    let file = File::open("./vulnerability.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut buffer = Vec::new();
    
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Vulnerability = result?;
        buffer.push(record);
    }
    Ok(buffer)
}

pub fn merge_footprint_with_vulnerabilities(vulnerabilities: Vec<Vulnerability>, footprints: Vec<FootPrint>) -> Vec<VulnerabilityFootPrint> {
    let mut buffer = Vec::new();

    for vulnerability in &vulnerabilities {
        for footprint in &footprints {
            if footprint.intensity_bin_id == vulnerability.intensity_bin_id {
                buffer.push(VulnerabilityFootPrint{
                    vulnerability_id: vulnerability.vulnerability_id,
                    intensity_bin_id: vulnerability.intensity_bin_id,
                    damage_bin_id: vulnerability.damage_bin_id,
                    damage_probability: vulnerability.probability,
                    event_id: footprint.event_id,
                    areaperil_id: footprint.areaperil_id,
                    footprint_probability: footprint.probability
                });
            }
        }
    }
    return buffer
}
