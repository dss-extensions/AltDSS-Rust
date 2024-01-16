#![allow(nonstandard_style)]

extern crate altdss;

use altdss::common::{DSSError, DSSContext};
use altdss::classic::IDSS;

fn run_ieee13(dss: &IDSS) -> Result<(), DSSError> {
    // This is IEEE13 circuit code, without some commented lines. See:
    // https://github.com/dss-extensions/electricdss-tst/blob/master/Version8/Distrib/IEEETestCases/13Bus/IEEE13Nodeckt.dss
    // We could of course use a redirect command here, but it's useful 
    // to illustrate that we don't need to do it.
    dss.Command("
    Clear 
    Set DefaultBaseFrequency=60
    
    !
    ! This script is based on a script developed by Tennessee Tech Univ students
    ! Tyler Patton, Jon Wood, and David Woods, April 2009
    !
    
    new circuit.IEEE13Nodeckt 
    ~ basekv=115 pu=1.0001 phases=3 bus1=SourceBus  
    ~ Angle=30                                                         ! advance angle 30 deg so result agree with published angle
    ~ MVAsc3=20000 MVASC1=21000    ! stiffen the source to approximate inf source
    
    
    
    !SUB TRANSFORMER DEFINITION 
    ! Although this data was given, it does not appear to be used in the test case results
    ! The published test case starts at 1.0 per unit at Bus 650. To make this happen, we will change the impedance
    ! on the transformer to something tiny by dividing by 1000 using the DSS in-line RPN math
    New Transformer.Sub Phases=3 Windings=2   XHL=(8 1000 /)
    ~ wdg=1 bus=SourceBus   conn=delta  kv=115  kva=5000   %r=(.5 1000 /) 
    ~ wdg=2 bus=650             conn=wye    kv=4.16  kva=5000   %r=(.5 1000 /)  
    
    ! FEEDER 1-PHASE VOLTAGE REGULATORS
    ! Define low-impedance 2-wdg transformer
    
    New Transformer.Reg1 phases=1 bank=reg1 XHL=0.01 kVAs=[1666 1666]
    ~ Buses=[650.1 RG60.1] kVs=[2.4  2.4] %LoadLoss=0.01
    new regcontrol.Reg1  transformer=Reg1 winding=2  vreg=122  band=2  ptratio=20 ctprim=700  R=3   X=9 
    
    New Transformer.Reg2 phases=1 bank=reg1 XHL=0.01 kVAs=[1666 1666]
    ~ Buses=[650.2 RG60.2] kVs=[2.4  2.4] %LoadLoss=0.01
    new regcontrol.Reg2  transformer=Reg2 winding=2  vreg=122  band=2  ptratio=20 ctprim=700  R=3   X=9 
    
    New Transformer.Reg3 phases=1 bank=reg1 XHL=0.01 kVAs=[1666 1666]
    ~ Buses=[650.3 RG60.3] kVs=[2.4  2.4] %LoadLoss=0.01
    new regcontrol.Reg3  transformer=Reg3 winding=2  vreg=122  band=2  ptratio=20 ctprim=700  R=3   X=9 
    
    
    !TRANSFORMER DEFINITION 
    New Transformer.XFM1  Phases=3   Windings=2  XHL=2
    ~ wdg=1 bus=633       conn=Wye kv=4.16    kva=500    %r=.55 
    ~ wdg=2 bus=634       conn=Wye kv=0.480    kva=500    %r=.55
    
    
    !LINE CODES

    // these are local matrix line codes
    // corrected 9-14-2011
    New linecode.mtx601 nphases=3 BaseFreq=60 
    ~ rmatrix = (0.3465 | 0.1560 0.3375 | 0.1580 0.1535 0.3414 ) 
    ~ xmatrix = (1.0179 | 0.5017 1.0478 | 0.4236 0.3849 1.0348 ) 
    ~ units=mi 
    New linecode.mtx602 nphases=3 BaseFreq=60 
    ~ rmatrix = (0.7526 | 0.1580 0.7475 | 0.1560 0.1535 0.7436 ) 
    ~ xmatrix = (1.1814 | 0.4236 1.1983 | 0.5017 0.3849 1.2112 ) 
    ~ units=mi 
    New linecode.mtx603 nphases=2 BaseFreq=60 
    ~ rmatrix = (1.3238 | 0.2066 1.3294 ) 
    ~ xmatrix = (1.3569 | 0.4591 1.3471 ) 
    ~ units=mi 
    New linecode.mtx604 nphases=2 BaseFreq=60 
    ~ rmatrix = (1.3238 | 0.2066 1.3294 ) 
    ~ xmatrix = (1.3569 | 0.4591 1.3471 ) 
    ~ units=mi 
    New linecode.mtx605 nphases=1 BaseFreq=60 
    ~ rmatrix = (1.3292 ) 
    ~ xmatrix = (1.3475 ) 
    ~ units=mi 
    
    New Linecode.mtx606 nphases=3  Units=mi
    ~ Rmatrix=[0.791721  |0.318476  0.781649  |0.28345  0.318476  0.791721  ]
    ~ Xmatrix=[0.438352  |0.0276838  0.396697  |-0.0184204  0.0276838  0.438352  ]
    ~ Cmatrix=[383.948  |0  383.948  |0  0  383.948  ]
    New linecode.mtx607 nphases=1 BaseFreq=60 
    ~ rmatrix = (1.3425 ) 
    ~ xmatrix = (0.5124 )
    ~ cmatrix = [236] 
    ~ units=mi 
    
    
    !LOAD DEFINITIONS 
    New Load.671 Bus1=671.1.2.3  Phases=3 Conn=Delta Model=1 kV=4.16   kW=1155 kvar=660 
    New Load.634a Bus1=634.1     Phases=1 Conn=Wye  Model=1 kV=0.277  kW=160   kvar=110 
    New Load.634b Bus1=634.2     Phases=1 Conn=Wye  Model=1 kV=0.277  kW=120   kvar=90 
    New Load.634c Bus1=634.3     Phases=1 Conn=Wye  Model=1 kV=0.277  kW=120   kvar=90 
    New Load.645 Bus1=645.2       Phases=1 Conn=Wye  Model=1 kV=2.4      kW=170   kvar=125 
    New Load.646 Bus1=646.2.3    Phases=1 Conn=Delta Model=2 kV=4.16    kW=230   kvar=132 
    New Load.692 Bus1=692.3.1    Phases=1 Conn=Delta Model=5 kV=4.16    kW=170   kvar=151 
    New Load.675a Bus1=675.1    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=485   kvar=190 
    New Load.675b Bus1=675.2    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=68   kvar=60 
    New Load.675c Bus1=675.3    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=290   kvar=212 
    New Load.611 Bus1=611.3      Phases=1 Conn=Wye  Model=5 kV=2.4  kW=170   kvar=80 
    New Load.652 Bus1=652.1      Phases=1 Conn=Wye  Model=2 kV=2.4  kW=128   kvar=86 
    New Load.670a Bus1=670.1    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=17    kvar=10 
    New Load.670b Bus1=670.2    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=66    kvar=38 
    New Load.670c Bus1=670.3    Phases=1 Conn=Wye  Model=1 kV=2.4  kW=117  kvar=68 
    
    !CAPACITOR DEFINITIONS
    New Capacitor.Cap1 Bus1=675 phases=3 kVAR=600 kV=4.16 
    New Capacitor.Cap2 Bus1=611.3 phases=1 kVAR=100 kV=2.4 
    
    !Bus 670 is the concentrated point load of the distributed load on line 632 to 671 located at 1/3 the distance from node 632
    
    !LINE DEFINITIONS 
    New Line.650632    Phases=3 Bus1=RG60.1.2.3   Bus2=632.1.2.3  LineCode=mtx601 Length=2000 units=ft 
    New Line.632670    Phases=3 Bus1=632.1.2.3    Bus2=670.1.2.3  LineCode=mtx601 Length=667  units=ft    
    New Line.670671    Phases=3 Bus1=670.1.2.3    Bus2=671.1.2.3  LineCode=mtx601 Length=1333 units=ft 
    New Line.671680    Phases=3 Bus1=671.1.2.3    Bus2=680.1.2.3  LineCode=mtx601 Length=1000 units=ft 
    New Line.632633    Phases=3 Bus1=632.1.2.3    Bus2=633.1.2.3  LineCode=mtx602 Length=500  units=ft 
    New Line.632645    Phases=2 Bus1=632.3.2      Bus2=645.3.2    LineCode=mtx603 Length=500  units=ft 
    New Line.645646    Phases=2 Bus1=645.3.2      Bus2=646.3.2    LineCode=mtx603 Length=300  units=ft 
    New Line.692675    Phases=3 Bus1=692.1.2.3    Bus2=675.1.2.3  LineCode=mtx606 Length=500  units=ft 
    New Line.671684    Phases=2 Bus1=671.1.3      Bus2=684.1.3    LineCode=mtx604 Length=300  units=ft 
    New Line.684611    Phases=1 Bus1=684.3        Bus2=611.3      LineCode=mtx605 Length=300  units=ft 
    New Line.684652    Phases=1 Bus1=684.1        Bus2=652.1      LineCode=mtx607 Length=800  units=ft 
    
    
    !SWITCH DEFINITIONS 
    New Line.671692    Phases=3 Bus1=671   Bus2=692  Switch=y  r1=1e-4 r0=1e-4 x1=0.000 x0=0.000 c1=0.000 c0=0.000
    
    Set Voltagebases=[115, 4.16, .48]
    calcv
    Solve    
    ".to_string())?;

    println!("Engine version: {}", dss.Version()?);
    let circ = &dss.ActiveCircuit;
    println!("Circuit name: {}", circ.Name()?);
    println!("Number of buses: {}", circ.NumBuses()?);
    println!("Number of nodes: {}", circ.NumNodes()?);
    println!("\nNode name, node voltage (pu)");
    let node_names = circ.AllNodeNames()?;
    let node_vpu = circ.AllBusVmagPu()?;
    for b in 0..node_names.len() {
        println!("{}, {:.3}", node_names[b], node_vpu[b]);
    }
    Ok(())
}

fn main() {
    // Create the context wrapper
    let ctx = DSSContext::prime();
    // Bind it to API structs
    let dss = IDSS::new(&ctx);

    run_ieee13(&dss).unwrap();
}
