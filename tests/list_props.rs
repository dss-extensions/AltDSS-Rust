// Copyright 2023 PMeira
// Copyright 2023 DSS-Extensions Contributors
// Copyright 2023 Electric Power Research Institute, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
