/// This example lists the properties of a DSS Storage object, basically the same
/// as done in https://github.com/dss-extensions/dss_capi/issues/125#issuecomment-1828580418
/// but using the high-level structs.

extern crate altdss;

use altdss::common::{DSSError, DSSContext};
use altdss::classic::IDSS;

fn list_storage_props(dss: &IDSS) -> Result<(), DSSError> {
    dss.Command("
        new circuit.test
        new storage.we_need_this_for_the_properties
    ".to_string())?;
    dss.ActiveCircuit.SetActiveElement("storage.we_need_this_for_the_properties".to_string())?;
    let names = dss.ActiveCircuit.ActiveCktElement.AllPropertyNames()?;
    for n in 0..names.len() {
         println!("PropIdx:{}, PropName:{}", n + 1, names[n]);
    }
    println!("{} properties", names.len());
    Ok(())
}

#[test]
fn storage_props() {
    // Create the context wrapper
    let ctx = DSSContext::prime();
    // Bind it to API structs
    let dss = IDSS::new(&ctx);

    list_storage_props(&dss).unwrap();
}
