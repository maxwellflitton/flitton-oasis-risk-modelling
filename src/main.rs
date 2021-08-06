mod footprint;
mod vulnerabilities;

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerabilities_with_footprint;


fn main() {
    println!("the modelling is running");
    let footprints = merge_event_ids_with_footprint(vec![1, 2, 3, 4, 2, 3, 1, 3, 4]);
    let vulnerability_footprint = merge_vulnerabilities_with_footprint(footprints);
    
    for i in vulnerability_footprint {
        println!("{:?}", i);
    }
}
