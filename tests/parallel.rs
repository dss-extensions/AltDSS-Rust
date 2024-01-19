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

/// This example uses multiple DSS engines in the same process.
/// It shows how to pass the extra contexts to the threads, 
/// initialize the new DSS instances, consume inputs from
/// a queue and return the results in a channel.
/// 
/// This example assumes the `electricdss-tst` repo is cloned
/// side-by-side with the current working directory.
///
/// Also note the call to `dss.Set_AllowChangeDir(false)`.

extern crate altdss;

use altdss::common::{DSSError, DSSContext};
use altdss::classic::{IDSS, ICircuit, SolveModes, ControlModes};
use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::process; // for exit

const REDIRECT_COMMAND: &str = "redirect ./electricdss-tst/Version8/Distrib/EPRITestCircuits/ckt5/Master_ckt5.dss";

fn solve_scenario(circ: &ICircuit, loadmult: f64) -> Result<(f64, f64), DSSError> {
    // Solve a simple snapshot to reset most of the general state
    circ.Solution.Set_Mode(SolveModes::SnapShot)?;
    circ.Solution.Set_ControlMode(ControlModes::Off)?;
    circ.Solution.Set_dblHour(0.0)?;
    circ.Solution.Set_LoadMult(1.0)?;
    circ.Solution.Solve()?;

    circ.Solution.Set_ControlMode(ControlModes::Off)?;
    circ.Solution.Set_Mode(SolveModes::Daily)?;
    circ.Solution.Set_StepsizeMin(15.0)?;
    circ.Solution.Set_Number(1)?;
    circ.Solution.Set_LoadMult(loadmult)?;
    circ.Meters.ResetAll()?;

    let mut all_converged = true;
    for _ in 0..96 {
        circ.Solution.Solve()?;
        if !circ.Solution.Get_Converged()? {
            all_converged = false;
        }
    }

    let mut losses_kWh: f64 = f64::NAN;
    if all_converged {
        circ.Meters.First()?;
        // Currently, we need to find the target quantity in the output array.
        // "Zone Losses kWh" is expected to be 12 here, but the position can change
        // across OpenDSS versions.
        // We should have a better way to address specific quantities in the future.
        let reg_names = circ.Meters.RegisterNames()?;
        let target_reg_name = "Zone Losses kWh".to_string();
        let quantity_idx = reg_names.iter().position(|val| *val == target_reg_name).unwrap();
        let totals = circ.Meters.Totals()?;
        losses_kWh = totals[quantity_idx]; 
    }

    Ok((loadmult, losses_kWh))
}

fn run_parallel(dss: &IDSS, num_threads: usize) -> Result<(), DSSError> {
    dss.Set_AllowChangeDir(false)?;

    let mut children = vec![];
    let (tx, rx) = channel();
    const NUM_STEPS: i32 = 90;
    const MULT_MIN: f64 = 0.5;
    const MULT_MAX: f64 = 1.2;
    const MULT_STEP: f64 = (MULT_MAX - MULT_MIN) / NUM_STEPS as f64;
    let shared_inputs = Arc::new(Mutex::new({let mut inputs = VecDeque::new();
        for i in 0..NUM_STEPS {
            inputs.push_back(MULT_MIN + (i as f64) * MULT_STEP);
        }
        inputs
    }));
    println!("Starting {} thread(s)...", num_threads);
    let start = Instant::now();
    for _ in 0..num_threads {
        let tx = tx.clone();
        let ctx = dss.NewContext()?;
        let thread_shared_inputs = Arc::clone(&shared_inputs);
        children.push(thread::spawn(move || {
            let engine = IDSS::new(&ctx);
            engine.Command(REDIRECT_COMMAND.to_string()).unwrap();
            loop {
                let input = thread_shared_inputs.lock().unwrap().pop_front();
                match input {
                    Some(loadmult) => tx.send(solve_scenario(&engine.ActiveCircuit, loadmult).unwrap()).unwrap(),
                    None => break,
                }
            } 
        }));
    }

    println!("Waiting...");
    for child in children {
        let _ = child.join();
    }

    let total_time = start.elapsed().as_secs_f64();
    println!("Total time for {num_threads} thread(s): {total_time:.2} s");

    println!("Results:");
    let mut avg_losses: f64 = 0.0;
    for _ in 0..NUM_STEPS {
        let (_loadmult, losses) = rx.recv().unwrap();
        // println!("{loadmult}, {losses}");
        avg_losses += losses;
    }
    avg_losses /= NUM_STEPS as f64;
    println!("Average losses using {num_threads} thread(s): {avg_losses:.3} kWh");
    Ok(())
}

#[test]
fn parallel() {
    // Create the context wrapper
    let ctx = DSSContext::prime();
    // Bind it to the API structs
    let dss = IDSS::new(&ctx);

    // First, let's try to run the script. If it doesn't exist, we get the error
    // before trying the parallel stuff.
    let res = dss.Command(REDIRECT_COMMAND.to_string());
    if res.is_err() {
        println!("Error: could not run the sample script. Ensure electricdss-tst is available side by side with the altdss-rust folder.");
        let err = res.unwrap_err();
        println!("DSS ERROR MESSAGE: {}", err.message);
        process::exit(1);
    }
    dss.ClearAll().unwrap();

    let max_num_threads = thread::available_parallelism().unwrap().get();
    run_parallel(&dss, 1).unwrap();
    
    // For large circuits, typically hyperthreads are not that useful.
    if max_num_threads > 4 {
        run_parallel(&dss, max_num_threads / 2).unwrap();
    }

    run_parallel(&dss, max_num_threads).unwrap();
}
