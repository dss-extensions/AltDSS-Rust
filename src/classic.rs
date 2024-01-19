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


#![allow(non_snake_case)]
#![allow(unused_parens)]

use crate::dss_capi;
use crate::common::{DSSContext, DSSError};
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::transmute;
use num_complex::Complex;

#[allow(non_snake_case)]

//TODO: for enums, avoid transmute: https://stackoverflow.com/a/76785380

#[repr(i32)]
pub enum ActionCodes {
	none = 0,
	Open = 1,
	Close = 2,
	Reset = 3,
	Lock = 4,
	Unlock = 5,
	TapUp = 6,
	TapDown = 7,
}

/// Event codes used by the event callback system
///
/// Legacy events are the events present the classic OpenDSS COM implementation,
/// while the rest are extensions added here.
#[repr(i32)]
pub enum AltDSSEvent {
	Legacy_InitControls = 0,
	Legacy_CheckControls = 1,
	Legacy_StepControls = 2,
	Clear = 3,
	ReprocessBuses = 4,
	BuildSystemY = 5,
}

#[repr(i32)]
pub enum AutoAddTypes {
	AddGen = 1,
	AddCap = 2,
}

#[repr(i32)]
pub enum CapControlModes {
	Current = 0,
	Voltage = 1,
	KVAR = 2,
	Time = 3,
	PF = 4,
}

#[repr(i32)]
pub enum CktModels {
	Multiphase = 0,
	PositiveSeq = 1,
}

#[repr(i32)]
pub enum ControlModes {
	Static = 0,
	Event = 1,
	Time = 2,
	Multirate = 3,
	Off = -1,
}

/// Transformer Core Type
#[repr(i32)]
pub enum CoreType {
	shell = 0,
	one_phase = 1,
	three_leg = 3,
	four_leg = 4,
	five_leg = 5,
	core_1_phase = 9,
}

#[repr(i32)]
pub enum DSSCompatFlags {
	NoSolverFloatChecks = 1,
	BadPrecision = 2,
	InvControl9611 = 4,
	SaveCalcVoltageBases = 8,
	ActiveLine = 16,
	NoPropertyTracking = 32,
	SkipSideEffects = 64,
}

#[repr(i32)]
pub enum DSSJSONFlags {
	Full = 1,
	SkipRedundant = 2,
	EnumAsInt = 4,
	FullNames = 8,
	Pretty = 16,
	ExcludeDisabled = 32,
	SkipDSSClass = 64,
	LowercaseKeys = 128,
	IncludeDefaultObjs = 256,
	SkipTimestamp = 512,
	SkipBuses = 1024,
}

/// This enum is used in the PropertyNameStyle property to control the naming convention.
/// Currently, this only affects capitalization, i.e., if your software already uses case
/// insensitive string comparisons for the property names, this is not useful. Otherwise,
/// you can use `Legacy` to use the older names.
#[repr(i32)]
pub enum DSSPropertyNameStyle {
	Modern = 0,
	Lowercase = 1,
	Legacy = 2,
}

#[repr(i32)]
pub enum GeneratorStatus {
	Variable = 0,
	Fixed = 1,
}

#[repr(i32)]
pub enum LineUnits {
	none = 0,
	Miles = 1,
	kFt = 2,
	km = 3,
	meter = 4,
	ft = 5,
	inch = 6,
	cm = 7,
	mm = 8,
}

#[repr(i32)]
pub enum LoadModels {
	ConstPQ = 1,
	ConstZ = 2,
	Motor = 3,
	CVR = 4,
	ConstI = 5,
	ConstPFixedQ = 6,
	ConstPFixedX = 7,
	ZIPV = 8,
}

#[repr(i32)]
pub enum LoadStatus {
	Variable = 0,
	Fixed = 1,
	Exempt = 2,
}

#[repr(i32)]
pub enum MonitorModes {
	VI = 0,
	Power = 1,
	Taps = 2,
	States = 3,
	Sequence = 16,
	Magnitude = 32,
	PosOnly = 64,
}

/// Overcurrent Protection Device Type
#[repr(i32)]
pub enum OCPDevType {
	none = 0,
	Fuse = 1,
	Recloser = 2,
	Relay = 3,
}

/// Deprecated. Please use instead:
/// - AutoAddTypes
/// - CktModels
/// - ControlModes
/// - SolutionLoadModels
/// - SolutionAlgorithms
/// - RandomModes
#[repr(i32)]
pub enum Options {
	PowerFlow = 1,
	Admittance = 2,
	NormalSolve = 0,
	LogNormal = 3,
	ControlOFF = -1,
}

#[repr(i32)]
pub enum RandomModes {
	Gaussian = 1,
	Uniform = 2,
	LogNormal = 3,
}

#[repr(i32)]
pub enum SolutionAlgorithms {
	NormalSolve = 0,
	NewtonSolve = 1,
}

#[repr(i32)]
pub enum SolutionLoadModels {
	PowerFlow = 1,
	Admittance = 2,
}

#[repr(i32)]
pub enum SolveModes {
	SnapShot = 0,
	Daily = 1,
	Yearly = 2,
	Monte1 = 3,
	LD1 = 4,
	PeakDay = 5,
	DutyCycle = 6,
	Direct = 7,
	MonteFault = 8,
	FaultStudy = 9,
	Monte2 = 10,
	Monte3 = 11,
	LD2 = 12,
	AutoAdd = 13,
	Dynamic = 14,
	Harmonic = 15,
	Time = 16,
	HarmonicT = 17,
}

#[repr(i32)]
pub enum SparseSolverOptions {
	ReuseNothing = 0,
	ReuseCompressedMatrix = 1,
	ReuseSymbolicFactorization = 2,
	ReuseNumericFactorization = 3,
	AlwaysResetYPrimInvalid = 268435456,
}

#[repr(i32)]
pub enum YMatrixModes {
	SeriesOnly = 1,
	WholeMatrix = 2,
}

fn bool_to_u16(v: bool) -> u16 {
    if v {
        1
    } else {
        0
    }
}

pub struct IBus<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IBus <'a> {
}
impl<'a> IBus<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    /// Returns an array with the names of all PCE connected to the active bus
    pub fn AllPCEatBus(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Bus_Get_AllPCEatBus(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Returns an array with the names of all PDE connected to the active bus
    pub fn AllPDEatBus(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Bus_Get_AllPDEatBus(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn GetUniqueNodeNumber(&self, StartNumber: i32) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_GetUniqueNodeNumber(self.ctx_ptr, StartNumber) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Refreshes the Zsc matrix for the active bus.
    pub fn ZscRefresh(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Bus_ZscRefresh(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Indicates whether a coordinate has been defined for this bus
    pub fn Coorddefined(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Bus_Get_Coorddefined(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Complex Double array of Sequence Voltages (0, 1, 2) at this Bus.
    pub fn CplxSeqVoltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_CplxSeqVoltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Accumulated customer outage durations
    pub fn Cust_Duration(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_Cust_Duration(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Annual number of customer-interruptions from this bus
    pub fn Cust_Interrupts(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_Cust_Interrupts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Distance from energymeter (if non-zero)
    pub fn Distance(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_Distance(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Average interruption duration, hr.
    pub fn Int_Duration(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_Int_Duration(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Short circuit currents at bus; Complex Array.
    pub fn Isc(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Isc_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Accumulated failure rate downstream from this bus; faults per year
    pub fn Lambda(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_Lambda(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total numbers of customers served downline from this bus
    pub fn N_Customers(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_N_Customers(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of interruptions this bus per year
    pub fn N_interrupts(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_N_interrupts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Name of Bus
    pub fn Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Bus_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Integer Array of Node Numbers defined at the bus in same order as the voltages.
    pub fn Nodes(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Nodes_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// Number of Nodes this bus.
    pub fn NumNodes(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_NumNodes(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Integer ID of the feeder section in which this bus is located.
    pub fn SectionID(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_SectionID(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Double Array of sequence voltages at this bus. Magnitudes only.
    pub fn SeqVoltages(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_SeqVoltages_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Total length of line downline from this bus, in miles. For recloser siting algorithm.
    pub fn TotalMiles(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_TotalMiles(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// For 2- and 3-phase buses, returns array of complex numbers represetin L-L voltages in volts. Returns -1.0 for 1-phase bus. If more than 3 phases, returns only first 3.
    pub fn VLL(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_VLL_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of doubles containing voltages in Magnitude (VLN), angle (degrees)
    pub fn VMagAngle(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_VMagAngle_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Open circuit voltage; Complex array.
    pub fn Voc(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Voc_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array of voltages at this bus.
    pub fn Voltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Voltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array of Ysc matrix at bus. Column by column.
    pub fn YscMatrix(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_YscMatrix_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex Zero-Sequence short circuit impedance at bus.
    pub fn Zsc0(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Zsc0_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Complex Positive-Sequence short circuit impedance at bus.
    pub fn Zsc1(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_Zsc1_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Complex array of Zsc matrix at bus. Column by column.
    pub fn ZscMatrix(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_ZscMatrix_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Base voltage at bus in kV
    pub fn kVBase(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_kVBase(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Returns Complex array of pu L-L voltages for 2- and 3-phase buses. Returns -1.0 for 1-phase bus. If more than 3 phases, returns only 3 phases.
    pub fn puVLL(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_puVLL_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of doubles containing voltage magnitude, angle (degrees) pairs in per unit
    pub fn puVmagAngle(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_puVmagAngle_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex Array of pu voltages at the bus.
    pub fn puVoltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_puVoltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of doubles (complex) containing the complete 012 Zsc matrix.
    /// Only available after Zsc is computed, either through the "ZscRefresh" command, or running a "FaultStudy" solution.
    /// Only available for buses with 3 nodes.
    pub fn ZSC012Matrix(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Bus_Get_ZSC012Matrix_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// X Coordinate for bus (double)
    pub fn Get_x(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_x(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_x(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Bus_Set_x(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Y coordinate for bus(double)
    pub fn Get_y(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Bus_Get_y(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_y(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Bus_Set_y(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// List of strings: Full Names of LOAD elements connected to the active bus.
    pub fn LoadList(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Bus_Get_LoadList(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// List of strings: Full Names of LINE elements connected to the active bus.
    pub fn LineList(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Bus_Get_LineList(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

}

pub struct ICNData<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ICNData <'a> {
}
impl<'a> ICNData<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all CNData names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CNData_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of CNData objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first CNData active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active CNData by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CNData_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active CNData.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_CNData_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next CNData active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active CNData; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active CNData by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Emergency ampere rating
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Normal Ampere rating
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rdc(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Rdc(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rdc(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_Rdc(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Rac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_Rac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_GMRac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GMRac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_GMRac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRUnits(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_GMRUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_GMRUnits(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_GMRUnits(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_Radius(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Radius(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Radius(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_Radius(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_RadiusUnits(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_RadiusUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_RadiusUnits(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_RadiusUnits(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_ResistanceUnits(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_ResistanceUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_ResistanceUnits(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_ResistanceUnits(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_Diameter(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_Diameter(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Diameter(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_Diameter(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_EpsR(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_EpsR(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EpsR(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_EpsR(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_InsLayer(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_InsLayer(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_InsLayer(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_InsLayer(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaIns(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_DiaIns(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaIns(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_DiaIns(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaCable(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_DiaCable(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaCable(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_DiaCable(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_k(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_k(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_k(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_k(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaStrand(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_DiaStrand(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaStrand(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_DiaStrand(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GmrStrand(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_GmrStrand(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GmrStrand(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_GmrStrand(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_RStrand(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CNData_Get_RStrand(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RStrand(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CNData_Set_RStrand(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ICapacitors<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ICapacitors <'a> {
}
impl<'a> ICapacitors<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Capacitor names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Capacitors_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Capacitor objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Capacitor active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Capacitor by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Capacitors_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Capacitor.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Capacitors_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Capacitor active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Capacitor; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Capacitor by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn AddStep(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Capacitors_AddStep(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Open(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Open(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SubtractStep(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Capacitors_SubtractStep(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Steps available in cap bank to be switched ON.
    pub fn AvailableSteps(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_AvailableSteps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Delta connection or wye?
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Capacitors_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Number of steps (default 1) for distributing and switching the total bank kVAR.
    pub fn Get_NumSteps(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_NumSteps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NumSteps(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_NumSteps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// A array of  integer [0..numsteps-1] indicating state of each step. If the read value is -1 an error has occurred.
    pub fn Get_States(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Get_States_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    pub fn Set_States(&self, value: &[i32]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_States(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Bank kV rating. Use LL for 2 or 3 phases, or actual can rating for 1 phase.
    pub fn Get_kV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_kV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_kV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Total bank KVAR, distributed equally among phases and steps.
    pub fn Get_kvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Capacitors_Get_kvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Capacitors_Set_kvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ICktElement<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
    pub Properties: IDSSProperty<'a>,
}

unsafe impl<'a> Send for ICktElement <'a> {
}
impl<'a> ICktElement<'a> {

    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
            Properties: IDSSProperty::new(&ctx),
        }
    }
    
    /// Value as return and error code in Code parameter. For PCElement, get the value of a variable by name. If Code>0 then no variable by this name or not a PCelement.
    pub fn Get_Variable(&self, varName: String, Code: *mut i32) -> Result<f64, DSSError> {
        let varName_c = CString::new(varName).unwrap();
        let result = unsafe { dss_capi::ctx_CktElement_Get_Variable(self.ctx_ptr, varName_c.as_ptr(), Code) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Value as return and error code in Code parameter. For PCElement, get the value of a variable by integer index. If Code>0 then no variable by this index or not a PCelement.
    pub fn Get_Variablei(&self, Idx: i32, Code: *mut i32) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_Variablei(self.ctx_ptr, Idx, Code) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Value as return and error code in Code parameter. For PCElement, get the value of a variable by integer index. If Code>0 then no variable by this index or not a PCelement.
    pub fn Get_VariableByIndex(&self, Idx: i32, Code: *mut i32) -> Result<f64, DSSError> {
        self.Get_Variablei(Idx, Code)
    }

    /// Value as return and error code in Code parameter. For PCElement, get the value of a variable by name. If Code>0 then no variable by this name or not a PCelement.
    pub fn Get_VariableByName(&self, Name: String, Code: *mut i32) -> Result<f64, DSSError> {
        self.Get_Variable(Name, Code)
    }

    /// Set the Value of a variable by index if a PCElement. If Code>0 then no variable by this index or not a PCelement.
    pub fn Set_VariableByIndex(&self, Idx: i32, Code: *mut i32, Value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Set_Variablei(self.ctx_ptr, Idx, Code, Value); }
        self.ctx.DSSError()
    }

    /// Set the Value of a variable by name if a PCElement. If Code>0 then no variable by this name or not a PCelement.
    pub fn Set_VariableByName(&self, varName: String, Code: *mut i32, Value: f64) -> Result<(), DSSError> {
        let varName_c = CString::new(varName).unwrap();
        unsafe { dss_capi::ctx_CktElement_Set_Variable(self.ctx_ptr, varName_c.as_ptr(), Code, Value); }
        self.ctx.DSSError()
    }

    pub fn Close(&self, Term: i32, Phs: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Close(self.ctx_ptr, Term, Phs) };
        self.ctx.DSSError()
    }

    /// Full name of the i-th controller attached to this element. Ex: str = Controller(2).  See NumControls to determine valid index range
    pub fn Controller(&self, idx: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CktElement_Get_Controller(self.ctx_ptr, idx)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn IsOpen(&self, Term: i32, Phs: i32) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_IsOpen(self.ctx_ptr, Term, Phs) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Open(&self, Term: i32, Phs: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Open(self.ctx_ptr, Term, Phs) };
        self.ctx.DSSError()
    }

    /// Array containing all property names of the active device.
    pub fn AllPropertyNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CktElement_Get_AllPropertyNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of strings listing all the published state variable names.
    /// Valid only for PCElements.
    pub fn AllVariableNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CktElement_Get_AllVariableNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of doubles. Values of state variables of active element if PC element.
    /// Valid only for PCElements.
    pub fn AllVariableValues(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_AllVariableValues_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of strings. Get  Bus definitions to which each terminal is connected.
    pub fn Get_BusNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CktElement_Get_BusNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn Set_BusNames(&self, value: &[String]) -> Result<(), DSSError> {
        let (_value_cstrs, value_c) = self.ctx.PrepareStringArray(value);
        unsafe { dss_capi::ctx_CktElement_Set_BusNames(self.ctx_ptr, value_c.as_ptr() as *mut *const c_char, value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Complex double array of Sequence Currents for all conductors of all terminals of active circuit element.
    pub fn CplxSeqCurrents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_CplxSeqCurrents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex double array of Sequence Voltage for all terminals of active circuit element.
    pub fn CplxSeqVoltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_CplxSeqVoltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array of currents into each conductor of each terminal
    pub fn Currents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Currents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Currents in magnitude, angle (degrees) format as a array of doubles.
    pub fn CurrentsMagAng(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_CurrentsMagAng_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Display name of the object (not necessarily unique)
    pub fn Get_DisplayName(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CktElement_Get_DisplayName(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DisplayName(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_CktElement_Set_DisplayName(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Emergency Ampere Rating for PD elements
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Boolean indicating that element is currently in the circuit.
    pub fn Get_Enabled(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_Get_Enabled(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Enabled(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Set_Enabled(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Name of the Energy Meter this element is assigned to.
    pub fn EnergyMeter(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CktElement_Get_EnergyMeter(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// globally unique identifier for this object
    pub fn GUID(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CktElement_Get_GUID(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Pointer to this object
    pub fn Handle(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_Handle(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// True if a recloser, relay, or fuse controlling this ckt element. OCP = Overcurrent Protection
    pub fn HasOCPDevice(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_Get_HasOCPDevice(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// This element has a SwtControl attached.
    pub fn HasSwitchControl(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_Get_HasSwitchControl(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// This element has a CapControl or RegControl attached.
    pub fn HasVoltControl(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_Get_HasVoltControl(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total losses in the element: two-element double array (complex), in VA (watts, vars)
    pub fn Losses(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Losses_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Full Name of Active Circuit Element
    pub fn Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CktElement_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Array of integer containing the node numbers (representing phases, for example) for each conductor of each terminal.
    pub fn NodeOrder(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_NodeOrder_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// Normal ampere rating for PD Elements
    pub fn Get_NormalAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NormalAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormalAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CktElement_Set_NormalAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of Conductors per Terminal
    pub fn NumConductors(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NumConductors(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of controls connected to this device.
    /// Use to determine valid range for index into Controller array.
    pub fn NumControls(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NumControls(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Phases
    pub fn NumPhases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NumPhases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Properties this Circuit Element.
    pub fn NumProperties(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NumProperties(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Terminals this Circuit Element
    pub fn NumTerminals(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_NumTerminals(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Index into Controller list of OCP Device controlling this CktElement
    pub fn OCPDevIndex(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_OCPDevIndex(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// 0=None; 1=Fuse; 2=Recloser; 3=Relay;  Type of OCP controller device
    pub fn OCPDevType(&self) -> Result<OCPDevType, DSSError> {
        let result = unsafe { dss_capi::ctx_CktElement_Get_OCPDevType(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    /// Complex array of losses (kVA) by phase
    pub fn PhaseLosses(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_PhaseLosses_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array of powers (kVA) into each conductor of each terminal
    pub fn Powers(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Powers_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Residual currents for each terminal: (magnitude, angle in degrees)
    pub fn Residuals(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Residuals_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Double array of symmetrical component currents (magnitudes only) into each 3-phase terminal
    pub fn SeqCurrents(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_SeqCurrents_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex array of sequence powers (kW, kvar) into each 3-phase teminal
    pub fn SeqPowers(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_SeqPowers_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Double array of symmetrical component voltages (magnitudes only) at each 3-phase terminal
    pub fn SeqVoltages(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_SeqVoltages_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex array of voltages at terminals
    pub fn Voltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Voltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Voltages at each conductor in magnitude, angle form as array of doubles.
    pub fn VoltagesMagAng(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_VoltagesMagAng_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// YPrim matrix, column order, complex numbers
    pub fn Yprim(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_Yprim_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Returns true if the current active element is isolated.
    /// Note that this only fetches the current value. See also the Topology interface.
    ///
    /// (API Extension)
    pub fn IsIsolated(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CktElement_Get_IsIsolated(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Returns an array with the total powers (complex, kVA) at ALL terminals of the active circuit element.
    pub fn TotalPowers(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_TotalPowers_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of integers, a copy of the internal NodeRef of the CktElement.
    pub fn NodeRef(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_CktElement_Get_NodeRef_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }
}

pub struct IGenerators<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IGenerators <'a> {
}
impl<'a> IGenerators<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Generator names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Generators_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Generator objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Generator active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Generator by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Generators_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Generator.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Generators_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Generator active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Generator; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Generator by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Indicates whether the generator is forced ON regardles of other dispatch criteria.
    pub fn Get_ForcedON(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Generators_Get_ForcedON(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ForcedON(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_ForcedON(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Generator Model
    pub fn Get_Model(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Model(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Model(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Model(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Power factor (pos. = producing vars). Updates kvar based on present kW value.
    pub fn Get_PF(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_PF(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PF(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_PF(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of phases
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of Names of all generator energy meter registers
    pub fn RegisterNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Generators_Get_RegisterNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of valus in generator energy meter registers.
    pub fn RegisterValues(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Generators_Get_RegisterValues_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Vmaxpu for generator model
    pub fn Get_Vmaxpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Vmaxpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vmaxpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Vmaxpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Vminpu for Generator model
    pub fn Get_Vminpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Vminpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vminpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Vminpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Voltage base for the active generator, kV
    pub fn Get_kV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_kV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_kV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// kVA rating of the generator
    pub fn Get_kVArated(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_kVArated(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kVArated(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_kVArated(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// kW output for the active generator. kvar is updated for current power factor.
    pub fn Get_kW(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_kW(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kW(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_kW(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// kvar output for the active generator. Updates power factor based on present kW value.
    pub fn Get_kvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_kvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_kvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the loadshape for a daily generation profile.
    ///
    /// (API Extension)
    pub fn Get_daily(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Generators_Get_daily(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_daily(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Generators_Set_daily(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of the loadshape for a duty cycle simulation.
    ///
    /// (API Extension)
    pub fn Get_duty(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Generators_Get_duty(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_duty(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Generators_Set_duty(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of yearly loadshape
    ///
    /// (API Extension)
    pub fn Get_Yearly(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Generators_Get_Yearly(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Yearly(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Generators_Set_Yearly(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Response to dispatch multipliers: Fixed=1 (dispatch multipliers do not apply), Variable=0 (follows curves).
    ///
    /// Related enumeration: GeneratorStatus
    ///
    /// (API Extension)
    pub fn Get_Status(&self) -> Result<GeneratorStatus, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Status(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Status(&self, value: GeneratorStatus) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Status(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Generator connection. True/1 if delta connection, False/0 if wye.
    ///
    /// (API Extension)
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Generators_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// kVA rating of electrical machine. Applied to machine or inverter definition for Dynamics mode solutions.
    ///
    /// (API Extension)
    pub fn Get_kva(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_kva(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kva(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_kva(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// An arbitrary integer number representing the class of Generator so that Generator values may be segregated by class.
    ///
    /// (API Extension)
    pub fn Get_Class(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Generators_Get_Class_(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Class(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Generators_Set_Class_(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Bus to which the Generator is connected. May include specific node specification.
    ///
    /// (API Extension)
    pub fn Get_Bus1(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Generators_Get_Bus1(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Bus1(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Generators_Set_Bus1(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }
}

pub struct ILines<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILines <'a> {
}
impl<'a> ILines<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Line names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Lines_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Line objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Line active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Line by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Line.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Line active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Line; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Line by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn New(&self, Name: String) -> Result<i32, DSSError> {
        let Name_c = CString::new(Name).unwrap();
        let result = unsafe { dss_capi::ctx_Lines_New(self.ctx_ptr, Name_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Name of bus for terminal 1.
    pub fn Get_Bus1(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_Bus1(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Bus1(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_Bus1(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of bus for terminal 2.
    pub fn Get_Bus2(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_Bus2(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Bus2(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_Bus2(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Zero Sequence capacitance, nanofarads per unit length.
    pub fn Get_C0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_C0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_C0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_C0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Positive Sequence capacitance, nanofarads per unit length.
    pub fn Get_C1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_C1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_C1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_C1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Cmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Lines_Get_Cmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Cmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Cmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Emergency (maximum) ampere rating of Line.
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Line geometry code
    pub fn Get_Geometry(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_Geometry(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Geometry(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_Geometry(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Length of line section in units compatible with the LineCode definition.
    pub fn Get_Length(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Length(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Length(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Length(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of LineCode object that defines the impedances.
    pub fn Get_LineCode(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_LineCode(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LineCode(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_LineCode(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Normal ampere rating of Line.
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of customers on this line section.
    pub fn NumCust(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_NumCust(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets Parent of the active Line to be the active line. Returns 0 if no parent or action fails.
    pub fn Parent(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Parent(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Phases, this Line element.
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Zero Sequence resistance, ohms per unit length.
    pub fn Get_R0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_R0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_R0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Positive Sequence resistance, ohms per unit length.
    pub fn Get_R1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_R1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_R1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Earth return resistance value used to compute line impedances at power frequency
    pub fn Get_Rg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Rg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Rg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Earth Resistivity, m-ohms
    pub fn Get_Rho(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Rho(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rho(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Rho(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Resistance matrix (full), ohms per unit length. Array of doubles.
    pub fn Get_Rmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Lines_Get_Rmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Rmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Rmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Line spacing code
    pub fn Get_Spacing(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Lines_Get_Spacing(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Spacing(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Lines_Set_Spacing(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Total Number of customers served from this line section.
    pub fn TotalCust(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_TotalCust(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Get_Units(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Units(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Units(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Units(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Zero Sequence reactance ohms per unit length.
    pub fn Get_X0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_X0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_X0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_X0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Positive Sequence reactance, ohms per unit length.
    pub fn Get_X1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_X1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_X1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_X1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Earth return reactance value used to compute line impedances at power frequency
    pub fn Get_Xg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_Xg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Xg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reactance matrix (full), ohms per unit length. Array of doubles.
    pub fn Get_Xmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Lines_Get_Xmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Xmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Yprimitive for the active line object (complex array).
    pub fn Get_Yprim(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Lines_Get_Yprim_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    pub fn Set_Yprim(&self, value: &[Complex<f64>]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_Yprim(self.ctx_ptr, &value[0].re, 2 * value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Delivers the rating for the current season (in Amps)  if the "SeasonalRatings" option is active
    pub fn SeasonRating(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Lines_Get_SeasonRating(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets/gets the Line element switch status. Setting it has side-effects to the line parameters.
    ///
    /// (API Extension)
    pub fn Get_IsSwitch(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Lines_Get_IsSwitch(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsSwitch(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Lines_Set_IsSwitch(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }
}

pub struct ISettings<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ISettings <'a> {
}
impl<'a> ISettings<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// {True | False*} Designates whether to allow duplicate names of objects
    ///
    /// **NOTE**: for DSS-Extensions, we are considering removing this option in a future
    /// release since it has performance impacts even when not used.
    pub fn Get_AllowDuplicates(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Settings_Get_AllowDuplicates(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllowDuplicates(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_AllowDuplicates(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// List of Buses or (File=xxxx) syntax for the AutoAdd solution mode.
    pub fn Get_AutoBusList(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Settings_Get_AutoBusList(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AutoBusList(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Settings_Set_AutoBusList(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// {dssMultiphase (0) * | dssPositiveSeq (1) } Indicate if the circuit model is positive sequence.
    pub fn Get_CktModel(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_CktModel(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CktModel(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_CktModel(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// {True | False*} Denotes whether to trace the control actions to a file.
    pub fn Get_ControlTrace(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Settings_Get_ControlTrace(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ControlTrace(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_ControlTrace(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Per Unit maximum voltage for Emergency conditions.
    pub fn Get_EmergVmaxpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_EmergVmaxpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergVmaxpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_EmergVmaxpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Per Unit minimum voltage for Emergency conditions.
    pub fn Get_EmergVminpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_EmergVminpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergVminpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_EmergVminpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Integer array defining which energy meter registers to use for computing losses
    pub fn Get_LossRegs(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Settings_Get_LossRegs_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    pub fn Set_LossRegs(&self, value: &[i32]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_LossRegs(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Weighting factor applied to Loss register values.
    pub fn Get_LossWeight(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_LossWeight(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LossWeight(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_LossWeight(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Per Unit maximum voltage for Normal conditions.
    pub fn Get_NormVmaxpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_NormVmaxpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormVmaxpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_NormVmaxpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Per Unit minimum voltage for Normal conditions.
    pub fn Get_NormVminpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_NormVminpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormVminpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_NormVminpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of LoadShape object that serves as the source of price signal data for yearly simulations, etc.
    pub fn Get_PriceCurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Settings_Get_PriceCurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PriceCurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Settings_Set_PriceCurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Price Signal for the Circuit
    pub fn Get_PriceSignal(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_PriceSignal(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PriceSignal(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_PriceSignal(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Gets value of trapezoidal integration flag in energy meters. Defaults to `False`.
    pub fn Get_Trapezoidal(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Settings_Get_Trapezoidal(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Trapezoidal(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_Trapezoidal(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Array of Integers defining energy meter registers to use for computing UE
    pub fn Get_UEregs(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Settings_Get_UEregs_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    pub fn Set_UEregs(&self, value: &[i32]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_UEregs(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Weighting factor applied to UE register values.
    pub fn Get_UEweight(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_UEweight(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_UEweight(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_UEweight(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles defining the legal voltage bases in kV L-L
    pub fn Get_VoltageBases(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Settings_Get_VoltageBases_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_VoltageBases(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_VoltageBases(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// {True | False*}  Locks Zones on energy meters to prevent rebuilding if a circuit change occurs.
    pub fn Get_ZoneLock(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Settings_Get_ZoneLock(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ZoneLock(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_ZoneLock(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    pub fn Set_AllocationFactors(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_AllocationFactors(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Controls whether the terminals are checked when updating the currents in Load component. Defaults to True.
    /// If the loads are guaranteed to have their terminals closed throughout the simulation, this can be set to False to save some time.
    ///
    /// (API Extension)
    pub fn Get_LoadsTerminalCheck(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Settings_Get_LoadsTerminalCheck(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LoadsTerminalCheck(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_LoadsTerminalCheck(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Controls whether `First`/`Next` iteration includes or skips disabled circuit elements.
    /// The default behavior from OpenDSS is to skip those. The user can still activate the element by name or index.
    ///
    /// The default value for IterateDisabled is 0, keeping the original behavior.
    /// Set it to 1 (or `True`) to include disabled elements.
    /// Other numeric values are reserved for other potential behaviors.
    ///
    /// (API Extension)
    pub fn Get_IterateDisabled(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Settings_Get_IterateDisabled(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IterateDisabled(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Settings_Set_IterateDisabled(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IActiveClass<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IActiveClass <'a> {
}
impl<'a> IActiveClass<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Returns name of active class.
    pub fn ActiveClassName(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ActiveClass_Get_ActiveClassName(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Array of strings consisting of all element names in the active class.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_ActiveClass_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of elements in Active Class. Same as NumElements Property.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ActiveClass_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets first element in the active class to be the active DSS object. If object is a CktElement, ActiveCktELment also points to this element. Returns 0 if none.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ActiveClass_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Name of the Active Element of the Active Class
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ActiveClass_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_ActiveClass_Set_Name(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Sets next element in active class to be the active DSS object. If object is a CktElement, ActiveCktElement also points to this element.  Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ActiveClass_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of elements in this class. Same as Count property.
    pub fn NumElements(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ActiveClass_Get_NumElements(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the name of the parent class of the active class
    pub fn ActiveClassParent(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ActiveClass_Get_ActiveClassParent(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
    /// Returns the data (as a list) of all elements from the active class as a JSON-encoded string.
    ///
    /// The `options` parameter contains bit-flags to toggle specific features.
    /// See `Obj_ToJSON` (C-API) for more.
    ///
    /// Additionally, the `ExcludeDisabled` flag can be used to excluded disabled elements from the output.
    ///
    /// (API Extension)
    pub fn ToJSON(&self, options: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ActiveClass_ToJSON(self.ctx_ptr, options)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct ICapControls<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ICapControls <'a> {
}
impl<'a> ICapControls<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all CapControl names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CapControls_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of CapControl objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first CapControl active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active CapControl by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CapControls_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active CapControl.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_CapControls_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next CapControl active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active CapControl; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active CapControl by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Transducer ratio from pirmary current to control current.
    pub fn Get_CTratio(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_CTratio(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CTratio(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_CTratio(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the Capacitor that is controlled.
    pub fn Get_Capacitor(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CapControls_Get_Capacitor(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Capacitor(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_CapControls_Set_Capacitor(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    pub fn Get_DeadTime(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_DeadTime(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DeadTime(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_DeadTime(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay [s] to switch on after arming.  Control may reset before actually switching.
    pub fn Get_Delay(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Delay(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Delay(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_Delay(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay [s] before switching off a step. Control may reset before actually switching.
    pub fn Get_DelayOff(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_DelayOff(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DelayOff(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_DelayOff(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Type of automatic controller.
    pub fn Get_Mode(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Mode(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Mode(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_Mode(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of the element that PT and CT are connected to.
    pub fn Get_MonitoredObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_CapControls_Get_MonitoredObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_CapControls_Set_MonitoredObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number on the element that PT and CT are connected to.
    pub fn Get_MonitoredTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_MonitoredTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_MonitoredTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Threshold to switch off a step. See Mode for units.
    pub fn Get_OFFSetting(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_OFFSetting(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_OFFSetting(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_OFFSetting(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Threshold to arm or switch on a step.  See Mode for units.
    pub fn Get_ONSetting(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_ONSetting(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ONSetting(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_ONSetting(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Transducer ratio from primary feeder to control voltage.
    pub fn Get_PTratio(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_PTratio(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PTratio(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_PTratio(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Enables Vmin and Vmax to override the control Mode
    pub fn Get_UseVoltOverride(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_CapControls_Get_UseVoltOverride(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_UseVoltOverride(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_UseVoltOverride(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// With VoltOverride, swtich off whenever PT voltage exceeds this level.
    pub fn Get_Vmax(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Vmax(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vmax(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_Vmax(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// With VoltOverride, switch ON whenever PT voltage drops below this level.
    pub fn Get_Vmin(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_CapControls_Get_Vmin(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vmin(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CapControls_Set_Vmin(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ICircuit<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
    pub Buses: IBus<'a>,
    pub CktElements: ICktElement<'a>,
    pub ActiveElement: ICktElement<'a>,
    pub Solution: ISolution<'a>,
    pub ActiveBus: IBus<'a>,
    pub Generators: IGenerators<'a>,
    pub Meters: IMeters<'a>,
    pub Monitors: IMonitors<'a>,
    pub Settings: ISettings<'a>,
    pub Lines: ILines<'a>,
    pub CtrlQueue: ICtrlQueue<'a>,
    pub Loads: ILoads<'a>,
    pub ActiveCktElement: ICktElement<'a>,
    pub ActiveDSSElement: IDSSElement<'a>,
    pub ActiveClass: IActiveClass<'a>,
    pub CapControls: ICapControls<'a>,
    pub RegControls: IRegControls<'a>,
    pub SwtControls: ISwtControls<'a>,
    pub Transformers: ITransformers<'a>,
    pub Capacitors: ICapacitors<'a>,
    pub Topology: ITopology<'a>,
    pub Sensors: ISensors<'a>,
    pub XYCurves: IXYCurves<'a>,
    pub PDElements: IPDElements<'a>,
    pub Reclosers: IReclosers<'a>,
    pub Relays: IRelays<'a>,
    pub LoadShapes: ILoadShapes<'a>,
    pub Fuses: IFuses<'a>,
    // pub DSSim_Coms: IDSSimComs<'a>,
    pub PVSystems: IPVSystems<'a>,
    pub Vsources: IVsources<'a>,
    pub LineCodes: ILineCodes<'a>,
    pub LineGeometries: ILineGeometries<'a>,
    pub LineSpacings: ILineSpacings<'a>,
    pub WireData: IWireData<'a>,
    pub CNData: ICNData<'a>,
    pub TSData: ITSData<'a>,
    pub Reactors: IReactors<'a>,
    pub ReduceCkt: IReduceCkt<'a>,
    pub Storages: IStorages<'a>,
    pub GICSources: IGICSources<'a>,
    pub Parallel: IParallel<'a>,
}

unsafe impl<'a> Send for ICircuit <'a> {
}
impl<'a> ICircuit<'a> {

    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
            Buses: IBus::new(&ctx),
            CktElements: ICktElement::new(&ctx),
            ActiveElement: ICktElement::new(&ctx),
            Solution: ISolution::new(&ctx),
            ActiveBus: IBus::new(&ctx),
            Generators: IGenerators::new(&ctx),
            Meters: IMeters::new(&ctx),
            Monitors: IMonitors::new(&ctx),
            Settings: ISettings::new(&ctx),
            Lines: ILines::new(&ctx),
            CtrlQueue: ICtrlQueue::new(&ctx),
            Loads: ILoads::new(&ctx),
            ActiveCktElement: ICktElement::new(&ctx),
            ActiveDSSElement: IDSSElement::new(&ctx),
            ActiveClass: IActiveClass::new(&ctx),
            CapControls: ICapControls::new(&ctx),
            RegControls: IRegControls::new(&ctx),
            SwtControls: ISwtControls::new(&ctx),
            Transformers: ITransformers::new(&ctx),
            Capacitors: ICapacitors::new(&ctx),
            Topology: ITopology::new(&ctx),
            Sensors: ISensors::new(&ctx),
            XYCurves: IXYCurves::new(&ctx),
            PDElements: IPDElements::new(&ctx),
            Reclosers: IReclosers::new(&ctx),
            Relays: IRelays::new(&ctx),
            LoadShapes: ILoadShapes::new(&ctx),
            Fuses: IFuses::new(&ctx),
            // DSSim_Coms: IDSSimComs::new(&ctx),
            PVSystems: IPVSystems::new(&ctx),
            Vsources: IVsources::new(&ctx),
            LineCodes: ILineCodes::new(&ctx),
            LineGeometries: ILineGeometries::new(&ctx),
            LineSpacings: ILineSpacings::new(&ctx),
            WireData: IWireData::new(&ctx),
            CNData: ICNData::new(&ctx),
            TSData: ITSData::new(&ctx),
            Reactors: IReactors::new(&ctx),
            ReduceCkt: IReduceCkt::new(&ctx),
            Storages: IStorages::new(&ctx),
            GICSources: IGICSources::new(&ctx),
            Parallel: IParallel::new(&ctx),
        }
    }
    
    /// Activates and returns a bus by its (zero-based) index. 
    /// Returns a reference to the existing ActiveBus.
    pub fn Get_Buses(&self, idx: i32) -> Result<&IBus, DSSError> {
        if (unsafe { dss_capi::ctx_Circuit_SetActiveBusi(self.ctx_ptr, idx) < 0 }) {
            let res = self.ctx.DSSError();
            match res {
                Err(e) => return Err(e),
                Ok(()) => return Err(DSSError {
                    number: 0,
                    message: "Could not activate bus".to_string()
                })
            }
        }
        self.ctx.DSSError()?;
        Ok(&self.ActiveBus)
    }

    /// Activates and returns a bus by its name.
    pub fn get_Buses(&self, name: String) -> Result<&IBus, DSSError> {
        let name_c = CString::new(name).unwrap();
        if (unsafe { dss_capi::ctx_Circuit_SetActiveBus(self.ctx_ptr, name_c.as_ptr()) < 0 }) {
            let res = self.ctx.DSSError();
            match res {
                Err(e) => return Err(e),
                Ok(()) => return Err(DSSError {
                    number: 0,
                    message: "Could not activate bus".to_string()
                })
            }
        }
        self.ctx.DSSError()?;
        Ok(&self.ActiveBus)
    }

    /// Activates and returns a CktElement by its global (zero-based) index.
    pub fn get_CktElementsi(&self, idx: i32) -> Result<&ICktElement, DSSError> {
        unsafe { dss_capi::ctx_Circuit_SetCktElementIndex(self.ctx_ptr, idx); }
        self.ctx.DSSError()?;
        Ok(&self.ActiveCktElement)
    }

    /// Activates and returns a CktElement by its full name (e.g. "load.abc").
    pub fn get_CktElements(&self, fullName: String) -> Result<&ICktElement, DSSError> {
        let fullName_c = CString::new(fullName).unwrap();
        unsafe { dss_capi::ctx_Circuit_SetCktElementName(self.ctx_ptr, fullName_c.as_ptr()); }
        self.ctx.DSSError()?;
        Ok(&self.ActiveCktElement)
    }

    pub fn Capacity(&self, Start: f64, Increment: f64) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_Capacity(self.ctx_ptr, Start, Increment) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Disable(&self, Name: String) -> Result<(), DSSError> {
        let Name_c = CString::new(Name).unwrap();
        unsafe { dss_capi::ctx_Circuit_Disable(self.ctx_ptr, Name_c.as_ptr()) };
        self.ctx.DSSError()
    }

    pub fn Enable(&self, Name: String) -> Result<(), DSSError> {
        let Name_c = CString::new(Name).unwrap();
        unsafe { dss_capi::ctx_Circuit_Enable(self.ctx_ptr, Name_c.as_ptr()) };
        self.ctx.DSSError()
    }

    pub fn EndOfTimeStepUpdate(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Circuit_EndOfTimeStepUpdate(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn FirstElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_FirstElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn FirstPCElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_FirstPCElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn FirstPDElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_FirstPDElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Returns an array of doubles representing the distances to parent EnergyMeter. Sequence of array corresponds to other node ByPhase properties.
    pub fn AllNodeDistancesByPhase(&self, Phase: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeDistancesByPhase_GR(self.ctx_ptr, Phase) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Return array of strings of the node names for the By Phase criteria. Sequence corresponds to other ByPhase properties.
    pub fn AllNodeNamesByPhase(&self, Phase: i32) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeNamesByPhase(self.ctx_ptr, &mut data, &mut cnt[0], Phase) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Returns Array of doubles represent voltage magnitudes for nodes on the specified phase.
    pub fn AllNodeVmagByPhase(&self, Phase: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeVmagByPhase_GR(self.ctx_ptr, Phase) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Returns array of per unit voltage magnitudes for each node by phase
    pub fn AllNodeVmagPUByPhase(&self, Phase: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeVmagPUByPhase_GR(self.ctx_ptr, Phase) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn NextElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_NextElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn NextPCElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_NextPCElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn NextPDElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_NextPDElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Sample(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Circuit_Sample(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SaveSample(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Circuit_SaveSample(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SetActiveBus(&self, BusName: String) -> Result<i32, DSSError> {
        let BusName_c = CString::new(BusName).unwrap();
        let result = unsafe { dss_capi::ctx_Circuit_SetActiveBus(self.ctx_ptr, BusName_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn SetActiveBusi(&self, BusIndex: i32) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_SetActiveBusi(self.ctx_ptr, BusIndex) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn SetActiveClass(&self, ClassName: String) -> Result<i32, DSSError> {
        let ClassName_c = CString::new(ClassName).unwrap();
        let result = unsafe { dss_capi::ctx_Circuit_SetActiveClass(self.ctx_ptr, ClassName_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn SetActiveElement(&self, FullName: String) -> Result<i32, DSSError> {
        let FullName_c = CString::new(FullName).unwrap();
        let result = unsafe { dss_capi::ctx_Circuit_SetActiveElement(self.ctx_ptr, FullName_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn UpdateStorage(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Circuit_UpdateStorage(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Returns distance from each bus to parent EnergyMeter. Corresponds to sequence in AllBusNames.
    pub fn AllBusDistances(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllBusDistances_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of strings containing names of all buses in circuit (see AllNodeNames).
    pub fn AllBusNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Circuit_Get_AllBusNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of magnitudes (doubles) of voltages at all buses
    pub fn AllBusVmag(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllBusVmag_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Double Array of all bus voltages (each node) magnitudes in Per unit
    pub fn AllBusVmagPu(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllBusVmagPu_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex array of all bus, node voltages from most recent solution
    pub fn AllBusVolts(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllBusVolts_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of total losses (complex) in each circuit element
    pub fn AllElementLosses(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllElementLosses_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of strings containing Full Name of all elements.
    pub fn AllElementNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Circuit_Get_AllElementNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Returns an array of distances from parent EnergyMeter for each Node. Corresponds to AllBusVMag sequence.
    pub fn AllNodeDistances(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeDistances_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of strings containing full name of each node in system in same order as returned by AllBusVolts, etc.
    pub fn AllNodeNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Circuit_Get_AllNodeNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Complex total line losses in the circuit
    pub fn LineLosses(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_LineLosses_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Total losses in active circuit, complex number (two-element array of double).
    pub fn Losses(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_Losses_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Name of the active circuit.
    pub fn Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Circuit_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total number of Buses in the circuit.
    pub fn NumBuses(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_Get_NumBuses(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of CktElements in the circuit.
    pub fn NumCktElements(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_Get_NumCktElements(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total number of nodes in the circuit.
    pub fn NumNodes(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_Get_NumNodes(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets Parent PD element, if any, to be the active circuit element and returns index>0; Returns 0 if it fails or not applicable.
    pub fn ParentPDElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Circuit_Get_ParentPDElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Complex losses in all transformers designated to substations.
    pub fn SubstationLosses(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_SubstationLosses_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// System Y matrix (after a solution has been performed).
    /// This is deprecated as it returns a dense matrix. Only use it for small systems.
    /// For large-scale systems, prefer YMatrix.GetCompressedYMatrix.
    pub fn SystemY(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_SystemY_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Total power (complex), kVA delivered to the circuit
    pub fn TotalPower(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_TotalPower_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    /// Array of doubles containing complex injection currents for the present solution. It is the "I" vector of I=YV
    pub fn YCurrents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_YCurrents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Array of strings containing the names of the nodes in the same order as the Y matrix
    pub fn YNodeOrder(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Circuit_Get_YNodeOrder(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Complex array of actual node voltages in same order as SystemY matrix.
    pub fn YNodeVarray(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Circuit_Get_YNodeVarray_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }
    /// Returns data for all objects and basic circuit properties as a JSON-encoded string.
    ///
    /// The JSON data is organized using the JSON schema proposed at
    /// https://github.com/dss-extensions/AltDSS-Schema
    ///
    /// The `options` parameter contains bit-flags to toggle specific features.
    /// See the enum `DSSJSONFlags` or `Obj_ToJSON` (C-API) for more.
    ///
    /// (API Extension)
    pub fn ToJSON(&self, options: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Circuit_ToJSON(self.ctx_ptr, options)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct ICtrlQueue<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ICtrlQueue <'a> {
}
impl<'a> ICtrlQueue<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    pub fn ClearActions(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_ClearActions(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn ClearQueue(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_ClearQueue(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Delete(&self, ActionHandle: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_Delete(self.ctx_ptr, ActionHandle) };
        self.ctx.DSSError()
    }

    pub fn DoAllQueue(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_DoAllQueue(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Show(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_Show(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Code for the active action. Long integer code to tell the control device what to do
    pub fn ActionCode(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Get_ActionCode(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Handle (User defined) to device that must act on the pending action.
    pub fn DeviceHandle(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Get_DeviceHandle(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Actions on the current actionlist (that have been popped off the control queue by CheckControlActions)
    pub fn NumActions(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Get_NumActions(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }
    /// Push a control action onto the DSS control queue by time, action code, and device handle (user defined). Returns Control Queue handle.
    pub fn Push(&self, Hour: i32, Seconds: f64, ActionCode: i32, DeviceHandle: i32) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Push(self.ctx_ptr, Hour, Seconds, ActionCode, DeviceHandle) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Pops next action off the action list and makes it the active action. Returns zero if none.
    pub fn PopAction(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Get_PopAction(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Array of strings containing the entire queue in CSV format
    pub fn Queue(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_CtrlQueue_Get_Queue(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of items on the OpenDSS control Queue
    pub fn QueueSize(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_CtrlQueue_Get_QueueSize(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Action(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_CtrlQueue_Set_Action(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IDSSElement<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
    pub Properties: IDSSProperty<'a>,
}

unsafe impl<'a> Send for IDSSElement <'a> {
}
impl<'a> IDSSElement<'a> {

    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
            Properties: IDSSProperty::new(&ctx),
        }
    }

    /// Array of strings containing the names of all properties for the active DSS object.
    pub fn AllPropertyNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_DSSElement_Get_AllPropertyNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Full Name of Active DSS Object (general element or circuit element).
    pub fn Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSSElement_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Properties for the active DSS object.
    pub fn NumProperties(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSSElement_Get_NumProperties(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }
    /// Returns the properties of the active DSS object as a JSON-encoded string.
    ///
    /// The `options` parameter contains bit-flags to toggle specific features.
    /// See `Obj_ToJSON` (C-API) for more.
    ///
    /// (API Extension)
    pub fn ToJSON(&self, options: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSSElement_ToJSON(self.ctx_ptr, options)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct IDSSProgress<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IDSSProgress <'a> {
}
impl<'a> IDSSProgress<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSSProgress_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Show(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSSProgress_Show(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Set_Caption(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_DSSProgress_Set_Caption(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    pub fn Set_PctProgress(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSSProgress_Set_PctProgress(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IDSSProperty<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IDSSProperty <'a> {
}
impl<'a> IDSSProperty<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    pub fn Set_idx(&self, key: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSSProperty_Set_Index(self.ctx_ptr, key); }
        self.ctx.DSSError()
    }

    pub fn Set_Name(&self, key: String) -> Result<(), DSSError> {
        let key_c = CString::new(key).unwrap();
        unsafe { dss_capi::ctx_DSSProperty_Set_Name(self.ctx_ptr, key_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Description of the property.
    pub fn Description(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSSProperty_Get_Description(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Name of Property
    pub fn Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSSProperty_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Get_Val(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSSProperty_Get_Val(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Val(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_DSSProperty_Set_Val(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }
}

pub struct IDSS_Executive<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IDSS_Executive <'a> {
}
impl<'a> IDSS_Executive<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Get i-th command
    pub fn Command(&self, i: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Executive_Get_Command(self.ctx_ptr, i)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get help string for i-th command
    pub fn CommandHelp(&self, i: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Executive_Get_CommandHelp(self.ctx_ptr, i)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get i-th option
    pub fn Option(&self, i: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Executive_Get_Option(self.ctx_ptr, i)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get help string for i-th option
    pub fn OptionHelp(&self, i: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Executive_Get_OptionHelp(self.ctx_ptr, i)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get present value of i-th option
    pub fn OptionValue(&self, i: i32) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Executive_Get_OptionValue(self.ctx_ptr, i)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of DSS Executive Commands
    pub fn NumCommands(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Executive_Get_NumCommands(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of DSS Executive Options
    pub fn NumOptions(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Executive_Get_NumOptions(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct IError<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IError <'a> {
}
impl<'a> IError<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Description of error for last operation
    pub fn Description(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Error_Get_Description(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Error Number (returns current value and then resets to zero)
    pub fn Number(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Error_Get_Number(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// EarlyAbort controls whether all errors halts the DSS script processing (Compile/Redirect), defaults to True.
    ///
    /// (API Extension)
    pub fn Get_EarlyAbort(&self) -> Result<bool, DSSError> {
        let result = (unsafe { dss_capi::ctx_Error_Get_EarlyAbort(self.ctx_ptr) } != 0);
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EarlyAbort(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Error_Set_EarlyAbort(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Controls whether the extended error mechanism is used. Defaults to True.
    ///
    /// Extended errors are errors derived from checks across the API to ensure
    /// a valid state. Although many of these checks are already present in the
    /// original/official COM interface, the checks do not produce any error
    /// message. An error value can be returned by a function but this value
    /// can, for many of the functions, be a valid value. As such, the user
    /// has no means to detect an invalid API call.
    ///
    /// Extended errors use the Error interface to provide a more clear message
    /// and should help users, especially new users, to find usage issues earlier.
    ///
    /// At Python level, an exception is raised when an error is detected through
    /// the Error interface.
    ///
    /// The current default state is ON. For compatibility, the user can turn it
    /// off to restore the previous behavior.
    ///
    /// (API Extension)
    pub fn Get_ExtendedErrors(&self) -> Result<bool, DSSError> {
        let result = (unsafe { dss_capi::ctx_Error_Get_ExtendedErrors(self.ctx_ptr) } != 0);
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ExtendedErrors(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Error_Set_ExtendedErrors(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }
}

pub struct IFuses<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IFuses <'a> {
}
impl<'a> IFuses<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Fuse names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Fuses_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Fuse objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Fuse active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Fuse by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Fuses_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Fuse.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Fuses_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Fuse active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Fuse; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Fuse by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Close all phases of the fuse.
    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Current state of the fuses. TRUE if any fuse on any phase is blown. Else FALSE.
    pub fn IsBlown(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Fuses_IsBlown(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Manual opening of all phases of the fuse.
    pub fn Open(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Open(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Reset fuse to normal state.
    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// A fixed delay time in seconds added to the fuse blowing time determined by the TCC curve. Default is 0.
    /// This represents a fuse clear or other delay.
    pub fn Get_Delay(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_Delay(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Delay(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Set_Delay(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of the circuit element to which the fuse is connected.
    pub fn Get_MonitoredObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Fuses_Get_MonitoredObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Fuses_Set_MonitoredObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number to which the fuse is connected.
    pub fn Get_MonitoredTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_MonitoredTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Set_MonitoredTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of phases, this fuse.
    pub fn NumPhases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_NumPhases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Multiplier or actual amps for the TCCcurve object. Defaults to 1.0.
    /// Multiply current values of TCC curve by this to get actual amps.
    pub fn Get_RatedCurrent(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_RatedCurrent(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RatedCurrent(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Set_RatedCurrent(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of the circuit element switch that the fuse controls.
    /// Defaults to the MonitoredObj.
    pub fn Get_SwitchedObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Fuses_Get_SwitchedObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Fuses_Set_SwitchedObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Number of the terminal of the controlled element containing the switch controlled by the fuse.
    pub fn Get_SwitchedTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Fuses_Get_SwitchedTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Fuses_Set_SwitchedTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the TCCcurve object that determines fuse blowing.
    pub fn Get_TCCcurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Fuses_Get_TCCcurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TCCcurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Fuses_Set_TCCcurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Array of strings indicating the state of each phase of the fuse.
    pub fn Get_State(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Fuses_Get_State(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn Set_State(&self, value: &[String]) -> Result<(), DSSError> {
        let (_value_cstrs, value_c) = self.ctx.PrepareStringArray(value);
        unsafe { dss_capi::ctx_Fuses_Set_State(self.ctx_ptr, value_c.as_ptr() as *mut *const c_char, value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Array of strings indicating the normal state of each phase of the fuse.
    pub fn Get_NormalState(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Fuses_Get_NormalState(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn Set_NormalState(&self, value: &[String]) -> Result<(), DSSError> {
        let (_value_cstrs, value_c) = self.ctx.PrepareStringArray(value);
        unsafe { dss_capi::ctx_Fuses_Set_NormalState(self.ctx_ptr, value_c.as_ptr() as *mut *const c_char, value.len() as i32) };
        self.ctx.DSSError()
    }
}

pub struct IISources<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IISources <'a> {
}
impl<'a> IISources<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all ISource names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_ISources_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of ISource objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first ISource active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active ISource by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ISources_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active ISource.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_ISources_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next ISource active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active ISource; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active ISource by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ISources_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Magnitude of the ISource in amps
    pub fn Get_Amps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_Amps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Amps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ISources_Set_Amps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Phase angle for ISource, degrees
    pub fn Get_AngleDeg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_AngleDeg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AngleDeg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ISources_Set_AngleDeg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// The present frequency of the ISource, Hz
    pub fn Get_Frequency(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_ISources_Get_Frequency(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Frequency(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ISources_Set_Frequency(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ILineCodes<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILineCodes <'a> {
}
impl<'a> ILineCodes<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all LineCode names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_LineCodes_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of LineCode objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first LineCode active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active LineCode by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_LineCodes_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active LineCode.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_LineCodes_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next LineCode active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active LineCode; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active LineCode by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Zero-sequence capacitance, nF per unit length
    pub fn Get_C0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_C0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_C0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_C0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Positive-sequence capacitance, nF per unit length
    pub fn Get_C1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_C1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_C1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_C1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Capacitance matrix, nF per unit length
    pub fn Get_Cmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Get_Cmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Cmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_Cmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Emergency ampere rating
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Flag denoting whether impedance data were entered in symmetrical components
    pub fn IsZ1Z0(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_LineCodes_Get_IsZ1Z0(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Normal Ampere rating
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of Phases
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Zero-Sequence Resistance, ohms per unit length
    pub fn Get_R0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_R0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_R0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Positive-sequence resistance ohms per unit length
    pub fn Get_R1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_R1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_R1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Resistance matrix, ohms per unit length
    pub fn Get_Rmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Get_Rmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Rmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_Rmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_Units(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_Units(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Units(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_Units(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Zero Sequence Reactance, Ohms per unit length
    pub fn Get_X0(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_X0(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_X0(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_X0(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Posiive-sequence reactance, ohms per unit length
    pub fn Get_X1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineCodes_Get_X1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_X1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_X1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reactance matrix, ohms per unit length
    pub fn Get_Xmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Get_Xmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineCodes_Set_Xmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }
}

pub struct IMonitors<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IMonitors <'a> {
}
impl<'a> IMonitors<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    // TODO: Implement AsMatrix someday

    /// Array of float64 for the specified channel (usage: MyArray = DSSMonitor.Channel(i)).
    /// A Save or SaveAll should be executed first. Done automatically by most standard solution modes.
    /// Channels start at index 1.
    pub fn Channel(&self, index: i32) -> Result<Box::<[f64]>, DSSError> {
        //TODO: use the better implementation
        unsafe { dss_capi::ctx_Monitors_Get_Channel_GR(self.ctx_ptr, index); }
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of strings with all Monitor names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Monitors_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Monitor objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Monitor active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Monitor by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Monitors_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Monitor.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Monitors_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Monitor active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Monitor; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Monitor by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Process(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Process(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn ProcessAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_ProcessAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn ResetAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_ResetAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Sample(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Sample(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SampleAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_SampleAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Save(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Save(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SaveAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_SaveAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Show(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Show(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Byte Array containing monitor stream values. Make sure a "save" is done first (standard solution modes do this automatically)
    pub fn ByteStream(&self) -> Result<Box::<[i8]>, DSSError> {
        unsafe { dss_capi::ctx_Monitors_Get_ByteStream_GR(self.ctx_ptr) };
        self.ctx.GetInt8ArrayGR()
    }

    /// Full object name of element being monitored.
    pub fn Get_Element(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Monitors_Get_Element(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Element(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Monitors_Set_Element(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of CSV file associated with active Monitor.
    pub fn FileName(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Monitors_Get_FileName(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Monitor File Version (integer)
    pub fn FileVersion(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_FileVersion(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Header string;  Array of strings containing Channel names
    pub fn Header(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Monitors_Get_Header(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Set Monitor mode (bitmask integer - see DSS Help)
    pub fn Get_Mode(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_Mode(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Mode(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Set_Mode(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of Channels in the active Monitor
    pub fn NumChannels(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_NumChannels(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Size of each record in ByteStream (Integer). Same as NumChannels.
    pub fn RecordSize(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_RecordSize(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Samples in Monitor at Present
    pub fn SampleCount(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_SampleCount(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Terminal number of element being monitored.
    pub fn Get_Terminal(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Monitors_Get_Terminal(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Terminal(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Monitors_Set_Terminal(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles containing frequency values for harmonics mode solutions; Empty for time mode solutions (use dblHour)
    pub fn dblFreq(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Monitors_Get_dblFreq_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of doubles containing time value in hours for time-sampled monitor values; Empty if frequency-sampled values for harmonics solution (see dblFreq)
    pub fn dblHour(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Monitors_Get_dblHour_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }
}

pub struct IParser<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IParser <'a> {
}
impl<'a> IParser<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Use this property to parse a Matrix token in OpenDSS format.  Returns square matrix of order specified. Order same as default Fortran order: column by column.
    pub fn Matrix(&self, ExpectedOrder: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Parser_Get_Matrix_GR(self.ctx_ptr, ExpectedOrder) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Use this property to parse a matrix token specified in lower triangle form. Symmetry is forced.
    pub fn SymMatrix(&self, ExpectedOrder: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Parser_Get_SymMatrix_GR(self.ctx_ptr, ExpectedOrder) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Returns token as array of doubles. For parsing quoted array syntax.
    pub fn Vector(&self, ExpectedSize: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Parser_Get_Vector_GR(self.ctx_ptr, ExpectedSize) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn ResetDelimiters(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parser_ResetDelimiters(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Default is FALSE. If TRUE parser automatically advances to next token after DblValue, IntValue, or StrValue. Simpler when you don't need to check for parameter names.
    pub fn Get_AutoIncrement(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Parser_Get_AutoIncrement(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AutoIncrement(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parser_Set_AutoIncrement(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Get/Set String containing the the characters for Quoting in OpenDSS scripts. Matching pairs defined in EndQuote. Default is "'([{.
    pub fn Get_BeginQuote(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_BeginQuote(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_BeginQuote(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Parser_Set_BeginQuote(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// String to be parsed. Loading this string resets the Parser to the beginning of the line. Then parse off the tokens in sequence.
    pub fn Get_CmdString(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_CmdString(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CmdString(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Parser_Set_CmdString(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Return next parameter as a double.
    pub fn DblValue(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Parser_Get_DblValue(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// String defining hard delimiters used to separate token on the command string. Default is , and =. The = separates token name from token value. These override whitesspace to separate tokens.
    pub fn Get_Delimiters(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_Delimiters(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Delimiters(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Parser_Set_Delimiters(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// String containing characters, in order, that match the beginning quote characters in BeginQuote. Default is "')]}
    pub fn Get_EndQuote(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_EndQuote(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EndQuote(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Parser_Set_EndQuote(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Return next parameter as a long integer.
    pub fn IntValue(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parser_Get_IntValue(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get next token and return tag name (before = sign) if any. See AutoIncrement.
    pub fn NextParam(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_NextParam(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Return next parameter as a string
    pub fn StrValue(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_StrValue(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/set the characters used for White space in the command string.  Default is blank and Tab.
    pub fn Get_WhiteSpace(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Parser_Get_WhiteSpace(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_WhiteSpace(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Parser_Set_WhiteSpace(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }
}

pub struct IReduceCkt<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IReduceCkt <'a> {
}
impl<'a> IReduceCkt<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Zmag (ohms) for Reduce Option for Z of short lines
    pub fn Get_Zmag(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_ReduceCkt_Get_Zmag(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Zmag(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_Set_Zmag(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Keep load flag for Reduction options that remove branches
    pub fn Get_KeepLoad(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_ReduceCkt_Get_KeepLoad(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_KeepLoad(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_Set_KeepLoad(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Edit String for RemoveBranches functions
    pub fn Get_EditString(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ReduceCkt_Get_EditString(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EditString(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_ReduceCkt_Set_EditString(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Start element for Remove Branch function
    pub fn Get_StartPDElement(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ReduceCkt_Get_StartPDElement(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_StartPDElement(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_ReduceCkt_Set_StartPDElement(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of Energymeter to use for reduction
    pub fn Get_EnergyMeter(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_ReduceCkt_Get_EnergyMeter(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EnergyMeter(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_ReduceCkt_Set_EnergyMeter(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Save present (reduced) circuit
    /// Filename is listed in the Text Result interface
    pub fn SaveCircuit(&self, CktName: String) -> Result<(), DSSError> {
        let CktName_c = CString::new(CktName).unwrap();
        unsafe { dss_capi::ctx_ReduceCkt_SaveCircuit(self.ctx_ptr, CktName_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Do Default Reduction algorithm
    pub fn DoDefault(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoDefault(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Do ShortLines algorithm: Set Zmag first if you don't want the default
    pub fn DoShortLines(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoShortLines(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Reduce Dangling Algorithm; branches with nothing connected
    pub fn DoDangling(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoDangling(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn DoLoopBreak(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoLoopBreak(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn DoParallelLines(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoParallelLines(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn DoSwitches(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoSwitches(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Do1phLaterals(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_Do1phLaterals(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn DoBranchRemove(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ReduceCkt_DoBranchRemove(self.ctx_ptr) };
        self.ctx.DSSError()
    }
}

pub struct ISolution<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ISolution <'a> {
}
impl<'a> ISolution<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    pub fn BuildYMatrix(&self, BuildOption: i32, AllocateVI: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_BuildYMatrix(self.ctx_ptr, BuildOption, AllocateVI) };
        self.ctx.DSSError()
    }

    pub fn CheckControls(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_CheckControls(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn CheckFaultStatus(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_CheckFaultStatus(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Cleanup(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Cleanup(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn DoControlActions(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_DoControlActions(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn FinishTimeStep(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_FinishTimeStep(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn InitSnap(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_InitSnap(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SampleControlDevices(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SampleControlDevices(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Sample_DoControlActions(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Sample_DoControlActions(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Solve(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Solve(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SolveDirect(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolveDirect(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SolveNoControl(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolveNoControl(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SolvePflow(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolvePflow(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SolvePlusControl(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolvePlusControl(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SolveSnap(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolveSnap(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Type of device to add in AutoAdd Mode: {dssGen (Default) | dssCap}
    pub fn Get_AddType(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_AddType(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AddType(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_AddType(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Base Solution algorithm: {dssNormalSolve | dssNewtonSolve}
    pub fn Get_Algorithm(&self) -> Result<SolutionAlgorithms, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Algorithm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Algorithm(&self, value: SolutionAlgorithms) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Algorithm(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Capacitor kvar for adding capacitors in AutoAdd mode
    pub fn Get_Capkvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Capkvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Capkvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Capkvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Flag indicating the control actions are done.
    pub fn Get_ControlActionsDone(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Solution_Get_ControlActionsDone(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ControlActionsDone(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_ControlActionsDone(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Value of the control iteration counter
    pub fn Get_ControlIterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_ControlIterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ControlIterations(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_ControlIterations(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// {dssStatic* | dssEvent | dssTime}  Modes for control devices
    pub fn Get_ControlMode(&self) -> Result<ControlModes, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_ControlMode(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_ControlMode(&self, value: ControlModes) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_ControlMode(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Flag to indicate whether the circuit solution converged
    pub fn Get_Converged(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Solution_Get_Converged(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Converged(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Converged(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Default daily load shape (defaults to "Default")
    pub fn Get_DefaultDaily(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Solution_Get_DefaultDaily(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DefaultDaily(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Solution_Set_DefaultDaily(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Default Yearly load shape (defaults to "Default")
    pub fn Get_DefaultYearly(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Solution_Get_DefaultYearly(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DefaultYearly(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Solution_Set_DefaultYearly(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Array of strings containing the Event Log
    pub fn EventLog(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Solution_Get_EventLog(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Set the Frequency for next solution
    pub fn Get_Frequency(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Frequency(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Frequency(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Frequency(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Default Multiplier applied to generators (like LoadMult)
    pub fn Get_GenMult(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_GenMult(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GenMult(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_GenMult(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// PF for generators in AutoAdd mode
    pub fn Get_GenPF(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_GenPF(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GenPF(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_GenPF(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Generator kW for AutoAdd mode
    pub fn Get_GenkW(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_GenkW(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GenkW(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_GenkW(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set Hour for time series solutions.
    pub fn Get_Hour(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Hour(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Hour(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Hour(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/Set the Solution.IntervalHrs variable used for devices that integrate / custom solution algorithms
    pub fn Get_IntervalHrs(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_IntervalHrs(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IntervalHrs(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_IntervalHrs(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of iterations taken for last solution. (Same as Totaliterations)
    pub fn Iterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Iterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Load-Duration Curve name for LD modes
    pub fn Get_LDCurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Solution_Get_LDCurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LDCurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Solution_Set_LDCurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Load Model: {dssPowerFlow (default) | dssAdmittance}
    pub fn Get_LoadModel(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_LoadModel(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LoadModel(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_LoadModel(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Default load multiplier applied to all non-fixed loads
    pub fn Get_LoadMult(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_LoadMult(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LoadMult(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_LoadMult(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Maximum allowable control iterations
    pub fn Get_MaxControlIterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_MaxControlIterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MaxControlIterations(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_MaxControlIterations(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Max allowable iterations.
    pub fn Get_MaxIterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_MaxIterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MaxIterations(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_MaxIterations(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Minimum number of iterations required for a power flow solution.
    pub fn Get_MinIterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_MinIterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MinIterations(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_MinIterations(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set present solution mode
    pub fn Get_Mode(&self) -> Result<SolveModes, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Mode(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Mode(&self, value: SolveModes) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Mode(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// ID (text) of the present solution mode
    pub fn ModeID(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Solution_Get_ModeID(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Max number of iterations required to converge at any control iteration of the most recent solution.
    pub fn MostIterationsDone(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_MostIterationsDone(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of solutions to perform for Monte Carlo and time series simulations
    pub fn Get_Number(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Number(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Number(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Number(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Gets the time required to perform the latest solution (Read only)
    pub fn Process_Time(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Process_Time(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Randomization mode for random variables "Gaussian" or "Uniform"
    pub fn Get_Random(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Random(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Random(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Random(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Seconds from top of the hour.
    pub fn Get_Seconds(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Seconds(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Seconds(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Seconds(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time step size in sec
    pub fn Get_StepSize(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_StepSize(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_StepSize(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_StepSize(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Flag that indicates if elements of the System Y have been changed by recent activity.
    pub fn SystemYChanged(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Solution_Get_SystemYChanged(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the solution process time + sample time for time step
    pub fn Time_of_Step(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Time_of_Step(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Solution convergence tolerance.
    pub fn Get_Tolerance(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Tolerance(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Tolerance(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Tolerance(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Gets/sets the accumulated time of the simulation
    pub fn Get_Total_Time(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Total_Time(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Total_Time(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Total_Time(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Total iterations including control iterations for most recent solution.
    pub fn Totaliterations(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Totaliterations(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set year for planning studies
    pub fn Get_Year(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_Year(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Year(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_Year(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Hour as a double, including fractional part
    pub fn Get_dblHour(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_dblHour(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_dblHour(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_dblHour(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent default  annual load growth rate
    pub fn Get_pctGrowth(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Solution_Get_pctGrowth(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_pctGrowth(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_pctGrowth(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Set_StepsizeHr(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_StepsizeHr(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Set_StepsizeMin(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_Set_StepsizeMin(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn BusLevels(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Solution_Get_BusLevels_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    pub fn IncMatrix(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Solution_Get_IncMatrix_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    pub fn IncMatrixCols(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Solution_Get_IncMatrixCols(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn IncMatrixRows(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Solution_Get_IncMatrixRows(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    pub fn Laplacian(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Solution_Get_Laplacian_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }
    pub fn SolveAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Solution_SolveAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }
}

pub struct ILineGeometries<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILineGeometries <'a> {
}
impl<'a> ILineGeometries<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    pub fn Get_Units(&self) -> Result<Box::<[LineUnits]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Units_GR(self.ctx_ptr); }
        let int_result = self.ctx.GetInt32ArrayGR()?;
        Ok(unsafe { transmute(int_result) })
    }

    pub fn Set_Units(&self, value: &[LineUnits]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Units(self.ctx_ptr, value.as_ptr() as *const i32, value.len() as i32); }
        self.ctx.DSSError()
    }

    /// Array of strings with all LineGeometrie names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_LineGeometries_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of LineGeometrie objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first LineGeometrie active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active LineGeometrie by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_LineGeometries_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active LineGeometrie.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_LineGeometries_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next LineGeometrie active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active LineGeometrie; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active LineGeometrie by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Array of strings with names of all conductors in the active LineGeometry object
    pub fn Conductors(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_LineGeometries_Get_Conductors(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Emergency ampere rating
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Normal ampere rating
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_RhoEarth(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_RhoEarth(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RhoEarth(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_RhoEarth(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Reduce(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_LineGeometries_Get_Reduce(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Reduce(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Reduce(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Number of Phases
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Resistance matrix, ohms
    pub fn Rmatrix(&self, Frequency: f64, Length: f64, Units: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Rmatrix_GR(self.ctx_ptr, Frequency, Length, Units) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Reactance matrix, ohms
    pub fn Xmatrix(&self, Frequency: f64, Length: f64, Units: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Xmatrix_GR(self.ctx_ptr, Frequency, Length, Units) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex impedance matrix, ohms
    pub fn Zmatrix(&self, Frequency: f64, Length: f64, Units: i32) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Zmatrix_GR(self.ctx_ptr, Frequency, Length, Units) };
        self.ctx.GetComplexArrayGR()
    }

    /// Capacitance matrix, nF
    pub fn Cmatrix(&self, Frequency: f64, Length: f64, Units: i32) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Cmatrix_GR(self.ctx_ptr, Frequency, Length, Units) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Get/Set the X (horizontal) coordinates of the conductors
    pub fn Get_Xcoords(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Xcoords_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xcoords(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Xcoords(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Get/Set the Y (vertical/height) coordinates of the conductors
    pub fn Get_Ycoords(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Get_Ycoords_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Ycoords(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Ycoords(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Number of conductors in this geometry. Default is 3. Triggers memory allocations. Define first!
    pub fn Get_Nconds(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineGeometries_Get_Nconds(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Nconds(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineGeometries_Set_Nconds(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ILineSpacings<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILineSpacings <'a> {
}
impl<'a> ILineSpacings<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all LineSpacing names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_LineSpacings_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of LineSpacing objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first LineSpacing active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active LineSpacing by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_LineSpacings_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active LineSpacing.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_LineSpacings_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next LineSpacing active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active LineSpacing; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active LineSpacing by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Number of Phases
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Nconds(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_Nconds(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Nconds(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_Nconds(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Units(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_LineSpacings_Get_Units(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Units(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_Units(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Get/Set the X (horizontal) coordinates of the conductors
    pub fn Get_Xcoords(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Get_Xcoords_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xcoords(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_Xcoords(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Get/Set the Y (vertical/height) coordinates of the conductors
    pub fn Get_Ycoords(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Get_Ycoords_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Ycoords(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LineSpacings_Set_Ycoords(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }
}

pub struct ILoadShapes<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILoadShapes <'a> {
}
impl<'a> ILoadShapes<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all LoadShape names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_LoadShapes_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of LoadShape objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first LoadShape active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active LoadShape by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_LoadShapes_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active LoadShape.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_LoadShapes_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next LoadShape active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active LoadShape; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active LoadShape by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn New(&self, Name: String) -> Result<i32, DSSError> {
        let Name_c = CString::new(Name).unwrap();
        let result = unsafe { dss_capi::ctx_LoadShapes_New(self.ctx_ptr, Name_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Normalize(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Normalize(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Fixed interval time value, hours.
    pub fn Get_HrInterval(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_HrInterval(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_HrInterval(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_HrInterval(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Fixed Interval time value, in minutes
    pub fn Get_MinInterval(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_MinInterval(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MinInterval(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_MinInterval(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set Number of points in active Loadshape.
    pub fn Get_Npts(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_Npts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Npts(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_Npts(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_PBase(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_PBase(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PBase(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_PBase(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles for the P multiplier in the Loadshape.
    pub fn Get_Pmult(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Get_Pmult_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Pmult(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_Pmult(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Base for normalizing Q curve. If left at zero, the peak value is used.
    pub fn Get_QBase(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_Qbase(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_QBase(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_Qbase(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles containing the Q multipliers.
    pub fn Get_Qmult(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Get_Qmult_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Qmult(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_Qmult(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Time array in hours correscponding to P and Q multipliers when the Interval=0.
    pub fn Get_TimeArray(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Get_TimeArray_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_TimeArray(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_TimeArray(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Boolean flag to let Loads know to use the actual value in the curve rather than use the value as a multiplier.
    pub fn Get_UseActual(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_LoadShapes_Get_UseActual(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_UseActual(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_UseActual(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    pub fn Get_sInterval(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_LoadShapes_Get_SInterval(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_sInterval(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_Set_SInterval(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Converts the current LoadShape data to float32/single precision.
    /// If there is no data or the data is already represented using float32, nothing is done.
    ///
    /// (API Extension)
    pub fn UseFloat32(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_UseFloat32(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Converts the current LoadShape data to float64/double precision.
    /// If there is no data or the data is already represented using float64, nothing is done.
    ///
    /// (API Extension)
    pub fn UseFloat64(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_LoadShapes_UseFloat64(self.ctx_ptr) };
        self.ctx.DSSError()
    }
}

pub struct ILoads<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ILoads <'a> {
}
impl<'a> ILoads<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Load names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Loads_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Load objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Load active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Load by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Load.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Load active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Load; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Load by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Factor for allocating loads by connected xfkva
    pub fn Get_AllocationFactor(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_AllocationFactor(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllocationFactor(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_AllocationFactor(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of a loadshape with both Mult and Qmult, for CVR factors as a function of time.
    pub fn Get_CVRcurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_CVRcurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CVRcurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_CVRcurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Percent reduction in Q for percent reduction in V. Must be used with dssLoadModelCVR.
    pub fn Get_CVRvars(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_CVRvars(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CVRvars(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_CVRvars(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent reduction in P for percent reduction in V. Must be used with dssLoadModelCVR.
    pub fn Get_CVRwatts(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_CVRwatts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CVRwatts(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_CVRwatts(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Factor relates average to peak kw.  Used for allocation with kwh and kwhdays
    pub fn Get_Cfactor(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Cfactor(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Cfactor(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Cfactor(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Class(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Class_(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Class(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Class_(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the growthshape curve for yearly load growth factors.
    pub fn Get_Growth(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_Growth(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Growth(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_Growth(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Delta loads are connected line-to-line.
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Loads_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// The Load Model defines variation of P and Q with voltage.
    pub fn Get_Model(&self) -> Result<LoadModels, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Model(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Model(&self, value: LoadModels) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Model(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Number of customers in this load, defaults to one.
    pub fn Get_NumCust(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_NumCust(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NumCust(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_NumCust(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get or set Power Factor for Active Load. Specify leading PF as negative. Updates kvar based on present value of kW
    pub fn Get_PF(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_PF(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PF(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_PF(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Average percent of nominal load in Monte Carlo studies; only if no loadshape defined for this load.
    pub fn Get_PctMean(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_PctMean(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PctMean(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_PctMean(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent standard deviation for Monte Carlo load studies; if there is no loadshape assigned to this load.
    pub fn Get_PctStdDev(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_PctStdDev(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PctStdDev(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_PctStdDev(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Relative Weighting factor for the active LOAD
    pub fn Get_RelWeight(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_RelWeight(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RelWeight(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_RelWeight(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Neutral resistance for wye-connected loads.
    pub fn Get_Rneut(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Rneut(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rneut(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Rneut(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of harmonic current spectrrum shape.
    pub fn Get_Spectrum(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_Spectrum(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Spectrum(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_Spectrum(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Response to load multipliers: Fixed (growth only), Exempt (no LD curve), Variable (all).
    pub fn Get_Status(&self) -> Result<LoadStatus, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Status(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_Status(&self, value: LoadStatus) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Status(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Maximum per-unit voltage to use the load model. Above this, constant Z applies.
    pub fn Get_Vmaxpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Vmaxpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vmaxpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Vmaxpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Minimum voltage for unserved energy (UE) evaluation.
    pub fn Get_Vminemerg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Vminemerg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vminemerg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Vminemerg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Minimum voltage for energy exceeding normal (EEN) evaluations.
    pub fn Get_Vminnorm(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Vminnorm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vminnorm(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Vminnorm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Minimum voltage to apply the load model. Below this, constant Z is used.
    pub fn Get_Vminpu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Vminpu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Vminpu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Vminpu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Neutral reactance for wye-connected loads.
    pub fn Get_Xneut(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Xneut(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xneut(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Xneut(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of yearly duration loadshape
    pub fn Get_Yearly(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_Yearly(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Yearly(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_Yearly(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Array of 7 doubles with values for ZIPV property of the load object
    pub fn Get_ZIPV(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Loads_Get_ZIPV_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_ZIPV(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_ZIPV(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Name of the loadshape for a daily load profile.
    pub fn Get_daily(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_daily(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_daily(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_daily(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of the loadshape for a duty cycle simulation.
    pub fn Get_duty(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_duty(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_duty(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Loads_Set_duty(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Set kV rating for active Load. For 2 or more phases set Line-Line kV. Else actual kV across terminals.
    pub fn Get_kV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set kW for active Load. Updates kvar based on present PF.
    pub fn Get_kW(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kW(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kW(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kW(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Base load kva. Also defined kw and kvar or pf input, or load allocation by kwh or xfkva.
    pub fn Get_kva(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kva(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kva(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kva(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set kvar for active Load. If set, updates PF based on present kW.
    pub fn Get_kvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// kwh billed for this period. Can be used with Cfactor for load allocation.
    pub fn Get_kwh(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kwh(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kwh(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kwh(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Length of kwh billing period for average demand calculation. Default 30.
    pub fn Get_kwhdays(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_kwhdays(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kwhdays(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_kwhdays(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent of Load that is modeled as series R-L for harmonics studies
    pub fn Get_pctSeriesRL(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_pctSeriesRL(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_pctSeriesRL(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_pctSeriesRL(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Rated service transformer kVA for load allocation, using AllocationFactor. Affects kW, kvar, and pf.
    pub fn Get_xfkVA(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_xfkVA(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_xfkVA(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_xfkVA(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the sensor monitoring this load.
    pub fn Sensor(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Loads_Get_Sensor(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of phases
    ///
    /// (API Extension)
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Loads_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Loads_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IMeters<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IMeters <'a> {
}
impl<'a> IMeters<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    //TODO: check if this needs to be adjusted
    /// Returns the list of all PCE within the area covered by the energy meter
    pub fn ZonePCE(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Meters_Get_ZonePCE(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of strings with all Meter names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Meters_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Meter objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Meter active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Meter by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Meters_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Meter.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Meters_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Meter active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Meter; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Meter by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Close All Demand Interval Files. Users are required to close the DI files at the end of a run.
    pub fn CloseAllDIFiles(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_CloseAllDIFiles(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Calculate reliability indices
    pub fn DoReliabilityCalc(&self, AssumeRestoration: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_DoReliabilityCalc(self.ctx_ptr, bool_to_u16(AssumeRestoration)) };
        self.ctx.DSSError()
    }

    /// Open Demand Interval (DI) files
    pub fn OpenAllDIFiles(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_OpenAllDIFiles(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Resets registers of active meter.
    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Resets registers of all meter objects.
    pub fn ResetAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_ResetAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Forces active Meter to take a sample.
    pub fn Sample(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Sample(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Causes all EnergyMeter objects to take a sample at the present time.
    pub fn SampleAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_SampleAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Saves meter register values.
    pub fn Save(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Save(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Save All EnergyMeter objects
    pub fn SaveAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_SaveAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SetActiveSection(&self, SectIdx: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_SetActiveSection(self.ctx_ptr, SectIdx) };
        self.ctx.DSSError()
    }

    /// Wide string list of all branches in zone of the active EnergyMeter object.
    pub fn AllBranchesInZone(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Meters_Get_AllBranchesInZone(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of names of all zone end elements.
    pub fn AllEndElements(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Meters_Get_AllEndElements(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of doubles: set the phase allocation factors for the active meter.
    pub fn Get_AllocFactors(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Meters_Get_AllocFactors_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_AllocFactors(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_AllocFactors(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Average Repair time in this section of the meter zone
    pub fn AvgRepairTime(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_AvgRepairTime(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the magnitude of the real part of the Calculated Current (normally determined by solution) for the Meter to force some behavior on Load Allocation
    pub fn Get_CalcCurrent(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Meters_Get_CalcCurrent_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_CalcCurrent(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_CalcCurrent(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Number of branches in Active energymeter zone. (Same as sequencelist size)
    pub fn CountBranches(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_CountBranches(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of zone end elements in the active meter zone.
    pub fn CountEndElements(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_CountEndElements(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total customer interruptions for this Meter zone based on reliability calcs.
    pub fn CustInterrupts(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_CustInterrupts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Global Flag in the DSS to indicate if Demand Interval (DI) files have been properly opened.
    pub fn DIFilesAreOpen(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Meters_Get_DIFilesAreOpen(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sum of Fault Rate time Repair Hrs in this section of the meter zone
    pub fn FaultRateXRepairHrs(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_FaultRateXRepairHrs(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set Name of metered element
    pub fn Get_MeteredElement(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Meters_Get_MeteredElement(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MeteredElement(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Meters_Set_MeteredElement(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// set Number of Metered Terminal
    pub fn Get_MeteredTerminal(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_MeteredTerminal(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MeteredTerminal(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_MeteredTerminal(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of branches (lines) in this section
    pub fn NumSectionBranches(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_NumSectionBranches(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Customers in the active section.
    pub fn NumSectionCustomers(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_NumSectionCustomers(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of feeder sections in this meter's zone
    pub fn NumSections(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_NumSections(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Type of OCP device. 1=Fuse; 2=Recloser; 3=Relay
    pub fn OCPDeviceType(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_OCPDeviceType(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Array of doubles to set values of Peak Current property
    pub fn Get_Peakcurrent(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Meters_Get_Peakcurrent_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Peakcurrent(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_Peakcurrent(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Array of strings containing the names of the registers.
    pub fn RegisterNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Meters_Get_RegisterNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of all the values contained in the Meter registers for the active Meter.
    pub fn RegisterValues(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Meters_Get_RegisterValues_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// SAIDI for this meter's zone. Execute DoReliabilityCalc first.
    pub fn SAIDI(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SAIDI(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Returns SAIFI for this meter's Zone. Execute Reliability Calc method first.
    pub fn SAIFI(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SAIFI(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// SAIFI based on kW rather than number of customers. Get after reliability calcs.
    pub fn SAIFIKW(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SAIFIKW(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// SequenceIndex of the branch at the head of this section
    pub fn SectSeqIdx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SectSeqIdx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total Customers downline from this section
    pub fn SectTotalCust(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SectTotalCust(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Size of Sequence List
    pub fn SeqListSize(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SeqListSize(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/set Index into Meter's SequenceList that contains branch pointers in lexical order. Earlier index guaranteed to be upline from later index. Sets PDelement active.
    pub fn Get_SequenceIndex(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SequenceIndex(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SequenceIndex(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Meters_Set_SequenceIndex(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Sum of the branch fault rates in this section of the meter's zone
    pub fn SumBranchFltRates(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_SumBranchFltRates(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total Number of customers in this zone (downline from the EnergyMeter)
    pub fn TotalCustomers(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Meters_Get_TotalCustomers(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Totals of all registers of all meters
    pub fn Totals(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Meters_Get_Totals_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

}

pub struct IPDElements<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IPDElements <'a> {
}
impl<'a> IPDElements<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// accummulated failure rate for this branch on downline
    pub fn AccumulatedL(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_AccumulatedL(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of PD elements (including disabled elements)
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/Set Number of failures per year.
    /// For LINE elements: Number of failures per unit length per year.
    pub fn Get_FaultRate(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_FaultRate(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_FaultRate(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PDElements_Set_FaultRate(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set the first enabled PD element to be the active element.
    /// Returns 0 if none found.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of the terminal of active PD element that is on the "from"
    /// side. This is set after the meter zone is determined.
    pub fn FromTerminal(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_FromTerminal(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Boolean indicating of PD element should be treated as a shunt
    /// element rather than a series element. Applies to Capacitor and Reactor
    /// elements in particular.
    pub fn IsShunt(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_PDElements_Get_IsShunt(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Failure rate for this branch. Faults per year including length of line.
    pub fn Lambda(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_Lambda(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/Set name of active PD Element. Returns null string if active element
    /// is not PDElement type.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PDElements_Get_Name(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PDElements_Set_Name(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Advance to the next PD element in the circuit. Enabled elements
    /// only. Returns 0 when no more elements.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of customers, this branch
    pub fn Numcustomers(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_Numcustomers(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the parent PD element to be the active circuit element.
    /// Returns 0 if no more elements upline.
    pub fn ParentPDElement(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_ParentPDElement(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Average repair time for this element in hours
    pub fn Get_RepairTime(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_RepairTime(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RepairTime(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PDElements_Set_RepairTime(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Integer ID of the feeder section that this PDElement branch is part of
    pub fn SectionID(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_SectionID(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total miles of line from this element to the end of the zone. For recloser siting algorithm.
    pub fn TotalMiles(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_TotalMiles(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Total number of customers from this branch to the end of the zone
    pub fn Totalcustomers(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_Totalcustomers(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/Set percent of faults that are permanent (require repair). Otherwise, fault is assumed to be transient/temporary.
    pub fn Get_pctPermanent(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PDElements_Get_pctPermanent(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_pctPermanent(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PDElements_Set_pctPermanent(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of strings consisting of all PD element names.
    ///
    /// (API Extension)
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_PDElements_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }
    /// Array of doubles with the maximum current across the conductors, for each PD
    /// element.
    ///
    /// By default, only the *first terminal* is used for the maximum current, matching
    /// the behavior of the "export capacity" command. Pass `true` to
    /// force the analysis to all terminals.
    ///
    /// See also:
    /// https://sourceforge.net/p/electricdss/discussion/beginners/thread/da5b93ca/
    ///
    /// (API Extension)
    pub fn AllMaxCurrents(&self, AllNodes: bool) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllMaxCurrents_GR(self.ctx_ptr, bool_to_u16(AllNodes)) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of doubles with the maximum current across the conductors as a percentage
    /// of the Normal Ampere Rating, for each PD element.
    ///
    /// By default, only the *first terminal* is used for the maximum current, matching
    /// the behavior of the "export capacity" command. Pass `true` to
    /// force the analysis to all terminals.
    ///
    /// See also:
    /// https://sourceforge.net/p/electricdss/discussion/beginners/thread/da5b93ca/
    ///
    /// (API Extension)
    pub fn AllPctNorm(&self, AllNodes: bool) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllPctNorm_GR(self.ctx_ptr, bool_to_u16(AllNodes)) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Array of doubles with the maximum current across the conductors as a percentage
    /// of the Emergency Ampere Rating, for each PD element.
    ///
    /// By default, only the *first terminal* is used for the maximum current, matching
    /// the behavior of the "export capacity" command. Pass `true` to
    /// force the analysis to all terminals.
    ///
    /// See also:
    /// https://sourceforge.net/p/electricdss/discussion/beginners/thread/da5b93ca/
    ///
    /// (API Extension)
    pub fn AllPctEmerg(&self, AllNodes: bool) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllPctEmerg_GR(self.ctx_ptr, bool_to_u16(AllNodes)) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex array of currents for all conductors, all terminals, for each PD element.
    ///
    /// (API Extension)
    pub fn AllCurrents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllCurrents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array (magnitude and angle format) of currents for all conductors, all terminals, for each PD element.
    ///
    /// (API Extension)
    pub fn AllCurrentsMagAng(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllCurrentsMagAng_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex double array of Sequence Currents for all conductors of all terminals, for each PD elements.
    ///
    /// (API Extension)
    pub fn AllCplxSeqCurrents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllCplxSeqCurrents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Double array of the symmetrical component currents (magnitudes only) into each 3-phase terminal, for each PD element.
    ///
    /// (API Extension)
    pub fn AllSeqCurrents(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllSeqCurrents_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Complex array of powers into each conductor of each terminal, for each PD element.
    ///
    /// (API Extension)
    pub fn AllPowers(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllPowers_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array of sequence powers into each 3-phase teminal, for each PD element
    ///
    /// (API Extension)
    pub fn AllSeqPowers(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllSeqPowers_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Integer array listing the number of phases of all PD elements
    ///
    /// (API Extension)
    pub fn AllNumPhases(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllNumPhases_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// Integer array listing the number of conductors of all PD elements
    ///
    /// (API Extension)
    pub fn AllNumConductors(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllNumConductors_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// Integer array listing the number of terminals of all PD elements
    ///
    /// (API Extension)
    pub fn AllNumTerminals(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_PDElements_Get_AllNumTerminals_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }
}

pub struct IPVSystems<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IPVSystems <'a> {
}
impl<'a> IPVSystems<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all PVSystem names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_PVSystems_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of PVSystem objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first PVSystem active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active PVSystem by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active PVSystem.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next PVSystem active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active PVSystem; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active PVSystem by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Get/set the present value of the Irradiance property in kW/m²
    pub fn Get_Irradiance(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_Irradiance(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Irradiance(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_Irradiance(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set the power factor for the active PVSystem
    pub fn Get_PF(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_PF(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PF(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_PF(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of PVSYSTEM energy meter register names
    pub fn RegisterNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_PVSystems_Get_RegisterNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of doubles containing values in PVSystem registers.
    pub fn RegisterValues(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Get_RegisterValues_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Get/set Rated kVA of the PVSystem
    pub fn Get_kVArated(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_kVArated(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kVArated(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_kVArated(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get kW output
    pub fn kW(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_kW(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get/set kvar output value
    pub fn Get_kvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_kvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_kvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the dispatch shape to use for daily simulations. Must be previously
    /// defined as a Loadshape object of 24 hrs, typically. In the default dispatch
    /// mode, the PVSystem element uses this loadshape to trigger State changes.
    ///
    /// (API Extension)
    pub fn Get_daily(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_daily(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_daily(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_daily(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of the load shape to use for duty cycle dispatch simulations such as
    /// for solar ramp rate studies. Must be previously defined as a Loadshape
    /// object. Typically would have time intervals of 1-5 seconds.
    ///
    /// (API Extension)
    pub fn Get_duty(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_duty(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_duty(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_duty(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Dispatch shape to use for yearly simulations. Must be previously defined
    /// as a Loadshape object. If this is not specified, the Daily dispatch shape,
    /// if any, is repeated during Yearly solution modes. In the default dispatch
    /// mode, the PVSystem element uses this loadshape to trigger State changes.
    ///
    /// (API Extension)
    pub fn Get_yearly(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_yearly(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_yearly(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_yearly(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Temperature shape to use for daily simulations. Must be previously defined
    /// as a TShape object of 24 hrs, typically. The PVSystem element uses this
    /// TShape to determine the Pmpp from the Pmpp vs T curve. Units must agree
    /// with the Pmpp vs T curve.
    ///
    /// (API Extension)
    pub fn Get_Tdaily(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_Tdaily(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Tdaily(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_Tdaily(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Temperature shape to use for duty cycle dispatch simulations such as for
    /// solar ramp rate studies. Must be previously defined as a TShape object.
    /// Typically would have time intervals of 1-5 seconds. Designate the number
    /// of points to solve using the Set Number=xxxx command. If there are fewer
    /// points in the actual shape, the shape is assumed to repeat. The PVSystem
    /// model uses this TShape to determine the Pmpp from the Pmpp vs T curve.
    /// Units must agree with the Pmpp vs T curve.
    ///
    /// (API Extension)
    pub fn Get_Tduty(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_Tduty(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Tduty(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_Tduty(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Temperature shape to use for yearly simulations. Must be previously defined
    /// as a TShape object. If this is not specified, the Daily dispatch shape, if
    /// any, is repeated during Yearly solution modes. The PVSystem element uses
    /// this TShape to determine the Pmpp from the Pmpp vs T curve. Units must
    /// agree with the Pmpp vs T curve.
    ///
    /// (API Extension)
    pub fn Get_Tyearly(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_Tyearly(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Tyearly(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_PVSystems_Set_Tyearly(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Returns the current irradiance value for the active PVSystem. Use it to
    /// know what's the current irradiance value for the PV during a simulation.
    pub fn IrradianceNow(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_IrradianceNow(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets/sets the rated max power of the PV array for 1.0 kW/sq-m irradiance
    /// and a user-selected array temperature of the active PVSystem.
    pub fn Get_Pmpp(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_PVSystems_Get_Pmpp(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Pmpp(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_PVSystems_Set_Pmpp(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the sensor monitoring this element.
    pub fn Sensor(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_PVSystems_Get_Sensor(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct IReactors<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IReactors <'a> {
}
impl<'a> IReactors<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Reactor names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Reactors_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Reactor objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Reactor active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Reactor by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reactors_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Reactor.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reactors_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Reactor active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Reactor; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Reactor by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// How the reactor data was provided: 1=kvar, 2=R+jX, 3=R and X matrices, 4=sym components.
    /// Depending on this value, only some properties are filled or make sense in the context.
    pub fn SpecType(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_SpecType(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Delta connection or wye?
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Reactors_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Indicates whether Rmatrix and Xmatrix are to be considered in parallel.
    pub fn Get_Parallel(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Reactors_Get_Parallel(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Parallel(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Parallel(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Inductance, mH. Alternate way to define the reactance, X, property.
    pub fn Get_LmH(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_LmH(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LmH(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_LmH(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// For 2, 3-phase, kV phase-phase. Otherwise specify actual coil rating.
    pub fn Get_kV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_kV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_kV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Total kvar, all phases.  Evenly divided among phases. Only determines X. Specify R separately
    pub fn Get_kvar(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_kvar(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kvar(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_kvar(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of phases.
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of first bus.
    /// Bus2 property will default to this bus, node 0, unless previously specified.
    /// Only Bus1 need be specified for a Yg shunt reactor.
    pub fn Get_Bus1(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reactors_Get_Bus1(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Bus1(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reactors_Set_Bus1(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of 2nd bus. Defaults to all phases connected to first bus, node 0, (Shunt Wye Connection) except when Bus2 is specifically defined.
    /// Not necessary to specify for delta (LL) connection
    pub fn Get_Bus2(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reactors_Get_Bus2(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Bus2(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reactors_Set_Bus2(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of XYCurve object, previously defined, describing per-unit variation of phase inductance, L=X/w, vs. frequency. Applies to reactance specified by X, LmH, Z, or kvar property. L generally decreases somewhat with frequency above the base frequency, approaching a limit at a few kHz.
    pub fn Get_LCurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reactors_Get_LCurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LCurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reactors_Set_LCurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Name of XYCurve object, previously defined, describing per-unit variation of phase resistance, R, vs. frequency. Applies to resistance specified by R or Z property. If actual values are not known, R often increases by approximately the square root of frequency.
    pub fn Get_RCurve(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reactors_Get_RCurve(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RCurve(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reactors_Set_RCurve(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Resistance (in series with reactance), each phase, ohms. This property applies to REACTOR specified by either kvar or X. See also help on Z.
    pub fn Get_R(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_R(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_R(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reactance, each phase, ohms at base frequency. See also help on Z and LmH properties.
    pub fn Get_X(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_X(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_X(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_X(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Resistance in parallel with R and X (the entire branch). Assumed infinite if not specified.
    pub fn Get_Rp(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reactors_Get_Rp(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rp(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Rp(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Resistance matrix, ohms at base frequency. Order of the matrix is the number of phases. Mutually exclusive to specifying parameters by kvar or X.
    pub fn Get_Rmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Rmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Rmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Rmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Reactance matrix, ohms at base frequency. Order of the matrix is the number of phases. Mutually exclusive to specifying parameters by kvar or X.
    pub fn Get_Xmatrix(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Xmatrix_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xmatrix(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Xmatrix(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Alternative way of defining R and X properties. Enter a 2-element array representing R +jX in ohms.
    pub fn Get_Z(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Z_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    pub fn Set_Z(&self, value: Complex<f64>) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Z(self.ctx_ptr, &value.re, 2) };
        self.ctx.DSSError()
    }

    /// Positive-sequence impedance, ohms, as a 2-element array representing a complex number.
    ///
    /// If defined, Z1, Z2, and Z0 are used to define the impedance matrix of the REACTOR.
    ///
    /// Z1 MUST BE DEFINED TO USE THIS OPTION FOR DEFINING THE MATRIX.
    ///
    /// Side Effect: Sets Z2 and Z0 to same values unless they were previously defined.
    pub fn Get_Z1(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Z1_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    pub fn Set_Z1(&self, value: Complex<f64>) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Z1(self.ctx_ptr, &value.re, 2) };
        self.ctx.DSSError()
    }

    /// Negative-sequence impedance, ohms, as a 2-element array representing a complex number.
    ///
    /// Used to define the impedance matrix of the REACTOR if Z1 is also specified.
    ///
    /// Note: Z2 defaults to Z1 if it is not specifically defined. If Z2 is not equal to Z1, the impedance matrix is asymmetrical.
    pub fn Get_Z2(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Z2_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    pub fn Set_Z2(&self, value: Complex<f64>) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Z2(self.ctx_ptr, &value.re, 2) };
        self.ctx.DSSError()
    }

    /// Zero-sequence impedance, ohms, as a 2-element array representing a complex number.
    ///
    /// Used to define the impedance matrix of the REACTOR if Z1 is also specified.
    ///
    /// Note: Z0 defaults to Z1 if it is not specifically defined.
    pub fn Get_Z0(&self) -> Result<Complex<f64>, DSSError> {
        unsafe { dss_capi::ctx_Reactors_Get_Z0_GR(self.ctx_ptr) };
        self.ctx.GetComplexSimpleGR()
    }

    pub fn Set_Z0(&self, value: Complex<f64>) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reactors_Set_Z0(self.ctx_ptr, &value.re, 2) };
        self.ctx.DSSError()
    }
}

pub struct IReclosers<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IReclosers <'a> {
}
impl<'a> IReclosers<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Recloser names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Reclosers_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Recloser objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Recloser active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Recloser by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reclosers_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Recloser.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reclosers_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Recloser active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Recloser; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Recloser by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Open(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Open(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Ground (3I0) instantaneous trip setting - curve multipler or actual amps.
    pub fn Get_GroundInst(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_GroundInst(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GroundInst(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_GroundInst(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Ground (3I0) trip multiplier or actual amps
    pub fn Get_GroundTrip(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_GroundTrip(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GroundTrip(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_GroundTrip(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of object this Recloser to be monitored.
    pub fn Get_MonitoredObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reclosers_Get_MonitoredObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reclosers_Set_MonitoredObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number of Monitored object for the Recloser
    pub fn Get_MonitoredTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_MonitoredTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_MonitoredTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of fast shots
    pub fn Get_NumFast(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_NumFast(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NumFast(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_NumFast(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Phase instantaneous curve multipler or actual amps
    pub fn Get_PhaseInst(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_PhaseInst(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PhaseInst(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_PhaseInst(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Phase trip curve multiplier or actual amps
    pub fn Get_PhaseTrip(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_PhaseTrip(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PhaseTrip(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_PhaseTrip(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of Doubles: reclose intervals, s, between shots.
    pub fn RecloseIntervals(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Get_RecloseIntervals_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    /// Number of shots to lockout (fast + delayed)
    pub fn Get_Shots(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_Shots(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Shots(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_Shots(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of the circuit element that is being switched by the Recloser.
    pub fn Get_SwitchedObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Reclosers_Get_SwitchedObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Reclosers_Set_SwitchedObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number of the controlled device being switched by the Recloser
    pub fn Get_SwitchedTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_SwitchedTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_SwitchedTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reset recloser to normal state.
    /// If open, lock out the recloser.
    /// If closed, resets recloser to first operation.
    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Get/Set present state of recloser.
    /// If set to open (ActionCodes.Open=1), open recloser's controlled element and lock out the recloser.
    /// If set to close (ActionCodes.Close=2), close recloser's controlled element and resets recloser to first operation.
    pub fn Get_State(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_State(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_State(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_State(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set normal state (ActionCodes.Open=1, ActionCodes.Close=2) of the recloser.
    pub fn Get_NormalState(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Reclosers_Get_NormalState(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormalState(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Reclosers_Set_NormalState(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IRegControls<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IRegControls <'a> {
}
impl<'a> IRegControls<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all RegControl names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_RegControls_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of RegControl objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first RegControl active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active RegControl by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_RegControls_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active RegControl.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_RegControls_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next RegControl active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active RegControl; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active RegControl by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// CT primary ampere rating (secondary is 0.2 amperes)
    pub fn Get_CTPrimary(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_CTPrimary(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CTPrimary(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_CTPrimary(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay [s] after arming before the first tap change. Control may reset before actually changing taps.
    pub fn Get_Delay(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_Delay(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Delay(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_Delay(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Regulation bandwidth in forward direciton, centered on Vreg
    pub fn Get_ForwardBand(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ForwardBand(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ForwardBand(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ForwardBand(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// LDC R setting in Volts
    pub fn Get_ForwardR(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ForwardR(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ForwardR(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ForwardR(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Target voltage in the forward direction, on PT secondary base.
    pub fn Get_ForwardVreg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ForwardVreg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ForwardVreg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ForwardVreg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// LDC X setting in Volts
    pub fn Get_ForwardX(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ForwardX(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ForwardX(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ForwardX(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay is inversely adjsuted, proportinal to the amount of voltage outside the regulating band.
    pub fn Get_IsInverseTime(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_RegControls_Get_IsInverseTime(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsInverseTime(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_IsInverseTime(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Regulator can use different settings in the reverse direction.  Usually not applicable to substation transformers.
    pub fn Get_IsReversible(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_RegControls_Get_IsReversible(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsReversible(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_IsReversible(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Maximum tap change per iteration in STATIC solution mode. 1 is more realistic, 16 is the default for a faster soluiton.
    pub fn Get_MaxTapChange(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_MaxTapChange(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MaxTapChange(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_MaxTapChange(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of a remote regulated bus, in lieu of LDC settings
    pub fn Get_MonitoredBus(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_RegControls_Get_MonitoredBus(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredBus(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_RegControls_Set_MonitoredBus(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// PT ratio for voltage control settings
    pub fn Get_PTratio(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_PTratio(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PTratio(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_PTratio(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Bandwidth in reverse direction, centered on reverse Vreg.
    pub fn Get_ReverseBand(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ReverseBand(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ReverseBand(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ReverseBand(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reverse LDC R setting in Volts.
    pub fn Get_ReverseR(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ReverseR(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ReverseR(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ReverseR(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Target voltage in the revese direction, on PT secondary base.
    pub fn Get_ReverseVreg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ReverseVreg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ReverseVreg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ReverseVreg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Reverse LDC X setting in volts.
    pub fn Get_ReverseX(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_ReverseX(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ReverseX(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_ReverseX(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay [s] for subsequent tap changes in a set. Control may reset before actually changing taps.
    pub fn Get_TapDelay(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_TapDelay(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TapDelay(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_TapDelay(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Integer number of the tap that the controlled transformer winding is currentliy on.
    pub fn Get_TapNumber(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_TapNumber(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TapNumber(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_TapNumber(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Tapped winding number
    pub fn Get_TapWinding(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_TapWinding(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TapWinding(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_TapWinding(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of the transformer this regulator controls
    pub fn Get_Transformer(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_RegControls_Get_Transformer(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Transformer(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_RegControls_Set_Transformer(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// First house voltage limit on PT secondary base.  Setting to 0 disables this function.
    pub fn Get_VoltageLimit(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_VoltageLimit(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_VoltageLimit(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_VoltageLimit(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Winding number for PT and CT connections
    pub fn Get_Winding(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_RegControls_Get_Winding(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Winding(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_RegControls_Set_Winding(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IRelays<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IRelays <'a> {
}
impl<'a> IRelays<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Relay names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Relays_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Relay objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Relay active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Relay by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Relays_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Relay.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Relays_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Relay active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Relay; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Relay by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Full name of object this Relay is monitoring.
    pub fn Get_MonitoredObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Relays_Get_MonitoredObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Relays_Set_MonitoredObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Number of terminal of monitored element that this Relay is monitoring.
    pub fn Get_MonitoredTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_MonitoredTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MonitoredTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Set_MonitoredTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of element that will be switched when relay trips.
    pub fn Get_SwitchedObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Relays_Get_SwitchedObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Relays_Set_SwitchedObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number of the switched object that will be opened when the relay trips.
    pub fn Get_SwitchedTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_SwitchedTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Set_SwitchedTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Open relay's controlled element and lock out the relay.
    pub fn Open(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Open(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Close the switched object controlled by the relay. Resets relay to first operation.
    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Reset relay to normal state.
    /// If open, lock out the relay.
    /// If closed, resets relay to first operation.
    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Get/Set present state of relay.
    /// If set to open, open relay's controlled element and lock out the relay.
    /// If set to close, close relay's controlled element and resets relay to first operation.
    pub fn Get_State(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_State(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_State(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Set_State(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Normal state of relay.
    pub fn Get_NormalState(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Relays_Get_NormalState(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormalState(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Relays_Set_NormalState(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ISensors<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ISensors <'a> {
}
impl<'a> ISensors<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Sensor names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Sensors_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Sensor objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Sensor active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Sensor by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Sensors_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Sensor.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Sensors_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Sensor active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Sensor; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Sensor by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn ResetAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_ResetAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Array of doubles for the line current measurements; don't use with kWS and kVARS.
    pub fn Get_Currents(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Sensors_Get_Currents_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Currents(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_Currents(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// True if measured voltages are line-line. Currents are always line currents.
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Sensors_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Full Name of the measured element
    pub fn Get_MeteredElement(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Sensors_Get_MeteredElement(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MeteredElement(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Sensors_Set_MeteredElement(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Number of the measured terminal in the measured element.
    pub fn Get_MeteredTerminal(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_MeteredTerminal(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MeteredTerminal(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_MeteredTerminal(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Assumed percent error in the Sensor measurement. Default is 1.
    pub fn Get_PctError(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_PctError(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_PctError(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_PctError(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// True if voltage measurements are 1-3, 3-2, 2-1.
    pub fn Get_ReverseDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Sensors_Get_ReverseDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ReverseDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_ReverseDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Weighting factor for this Sensor measurement with respect to other Sensors. Default is 1.
    pub fn Get_Weight(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_Weight(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Weight(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_Weight(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles for Q measurements. Overwrites Currents with a new estimate using kWS.
    pub fn Get_kVARS(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Sensors_Get_kVARS_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_kVARS(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_kVARS(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Array of doubles for the LL or LN (depending on Delta connection) voltage measurements.
    pub fn Get_kVS(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Sensors_Get_kVS_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_kVS(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_kVS(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Voltage base for the sensor measurements. LL for 2 and 3-phase sensors, LN for 1-phase sensors.
    pub fn Get_kVbase(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Sensors_Get_kVbase(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kVbase(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_kVbase(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of doubles for P measurements. Overwrites Currents with a new estimate using kVARS.
    pub fn Get_kWS(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Sensors_Get_kWS_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_kWS(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Sensors_Set_kWS(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Array of doubles for the allocation factors for each phase.
    pub fn AllocationFactor(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Sensors_Get_AllocationFactor_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }
}

pub struct ISwtControls<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ISwtControls <'a> {
}
impl<'a> ISwtControls<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all SwtControl names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_SwtControls_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of SwtControl objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first SwtControl active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active SwtControl by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_SwtControls_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active SwtControl.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_SwtControls_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next SwtControl active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active SwtControl; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active SwtControl by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Open or Close the switch. No effect if switch is locked.  However, Reset removes any lock and then closes the switch (shelf state).
    pub fn Get_Action(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_Action(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Action(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_Action(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Time delay [s] betwen arming and opening or closing the switch.  Control may reset before actually operating the switch.
    pub fn Get_Delay(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_Delay(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Delay(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_Delay(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// The lock prevents both manual and automatic switch operation.
    pub fn Get_IsLocked(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_SwtControls_Get_IsLocked(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsLocked(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_IsLocked(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Get/set Normal state of switch (see actioncodes) dssActionOpen or dssActionClose
    pub fn Get_NormalState(&self) -> Result<ActionCodes, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_NormalState(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_NormalState(&self, value: ActionCodes) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_NormalState(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// Set it to force the switch to a specified state, otherwise read its present state.
    pub fn Get_State(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_State(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_State(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_State(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Full name of the switched element.
    pub fn Get_SwitchedObj(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_SwtControls_Get_SwitchedObj(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedObj(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_SwtControls_Set_SwitchedObj(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Terminal number where the switch is located on the SwitchedObj
    pub fn Get_SwitchedTerm(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_SwtControls_Get_SwitchedTerm(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SwitchedTerm(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_SwtControls_Set_SwitchedTerm(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct ITSData<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ITSData <'a> {
}
impl<'a> ITSData<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all TSData names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_TSData_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of TSData objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first TSData active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active TSData by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_TSData_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active TSData.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_TSData_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next TSData active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active TSData; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active TSData by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Emergency ampere rating
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Normal Ampere rating
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rdc(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Rdc(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rdc(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_Rdc(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Rac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_Rac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_GMRac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GMRac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_GMRac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRUnits(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_GMRUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GMRUnits(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_GMRUnits(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Radius(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Radius(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Radius(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_Radius(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_RadiusUnits(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_RadiusUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RadiusUnits(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_RadiusUnits(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_ResistanceUnits(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_ResistanceUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ResistanceUnits(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_ResistanceUnits(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Diameter(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_Diameter(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Diameter(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_Diameter(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_EpsR(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_EpsR(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EpsR(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_EpsR(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_InsLayer(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_InsLayer(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_InsLayer(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_InsLayer(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaIns(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_DiaIns(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaIns(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_DiaIns(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaCable(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_DiaCable(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaCable(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_DiaCable(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_DiaShield(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_DiaShield(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DiaShield(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_DiaShield(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_TapeLayer(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_TapeLayer(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TapeLayer(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_TapeLayer(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_TapeLap(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_TSData_Get_TapeLap(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_TapeLap(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_TSData_Set_TapeLap(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IText<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IText <'a> {
}
impl<'a> IText<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    /// Runs a list of strings as commands directly in the DSS engine.
    /// Intermediate results are ignored.
    ///
    /// (API Extension)
    pub fn Commands(&self, value: &[String]) -> Result<(), DSSError> {
        let (_value_cstrs, value_c) = self.ctx.PrepareStringArray(value);
        unsafe { dss_capi::ctx_Text_CommandArray(self.ctx_ptr, value_c.as_ptr() as *mut *const c_char, value.len() as i32); }
        self.ctx.DSSError()
    }

    /// Runs a large string as commands directly in the DSS engine.
    /// Intermediate results are ignored.
    ///
    /// (API Extension)
    pub fn CommandBlock(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Text_CommandBlock(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Input command string for the DSS.
    pub fn Get_Command(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Text_Get_Command(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Command(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Text_Set_Command(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Result string for the last command.
    pub fn Result(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Text_Get_Result(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct ITopology<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ITopology <'a> {
}
impl<'a> ITopology<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Returns index of the active branch
    pub fn ActiveBranch(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_ActiveBranch(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Topological depth of the active branch
    pub fn ActiveLevel(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_ActiveLevel(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Array of all isolated branch names.
    pub fn AllIsolatedBranches(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Topology_Get_AllIsolatedBranches(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of all isolated load names.
    pub fn AllIsolatedLoads(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Topology_Get_AllIsolatedLoads(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of all looped element names, by pairs.
    pub fn AllLoopedPairs(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Topology_Get_AllLoopedPairs(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Move back toward the source, return index of new active branch, or 0 if no more.
    pub fn BackwardBranch(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_BackwardBranch(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Name of the active branch.
    pub fn Get_BranchName(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Topology_Get_BranchName(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_BranchName(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Topology_Set_BranchName(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Set the active branch to one containing this bus, return index or 0 if not found
    pub fn Get_BusName(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Topology_Get_BusName(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_BusName(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Topology_Set_BusName(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Sets the first branch active, returns 0 if none.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// First load at the active branch, return index or 0 if none.
    pub fn FirstLoad(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_FirstLoad(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Move forward in the tree, return index of new active branch or 0 if no more
    pub fn ForwardBranch(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_ForwardBranch(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Move to looped branch, return index or 0 if none.
    pub fn LoopedBranch(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_LoopedBranch(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the next branch active, returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Next load at the active branch, return index or 0 if no more.
    pub fn NextLoad(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_NextLoad(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of isolated branches (PD elements and capacitors).
    pub fn NumIsolatedBranches(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_NumIsolatedBranches(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of isolated loads
    pub fn NumIsolatedLoads(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_NumIsolatedLoads(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of loops
    pub fn NumLoops(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_NumLoops(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Move to directly parallel branch, return index or 0 if none.
    pub fn ParallelBranch(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Topology_Get_ParallelBranch(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct ITransformers<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for ITransformers <'a> {
}
impl<'a> ITransformers<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Transformer names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Transformers_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Transformer objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Transformer active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Transformer by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Transformers_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Transformer.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Transformers_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Transformer active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Transformer; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Transformer by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Active Winding delta or wye connection?
    pub fn Get_IsDelta(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_Transformers_Get_IsDelta(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_IsDelta(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_IsDelta(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Active Winding maximum tap in per-unit.
    pub fn Get_MaxTap(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_MaxTap(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MaxTap(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_MaxTap(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding minimum tap in per-unit.
    pub fn Get_MinTap(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_MinTap(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_MinTap(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_MinTap(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding number of tap steps betwein MinTap and MaxTap.
    pub fn Get_NumTaps(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_NumTaps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NumTaps(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_NumTaps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of windings on this transformer. Allocates memory; set or change this property first.
    pub fn Get_NumWindings(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_NumWindings(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NumWindings(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_NumWindings(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding resistance in %
    pub fn Get_R(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_R(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_R(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_R(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding neutral resistance [ohms] for wye connections. Set less than zero for ungrounded wye.
    pub fn Get_Rneut(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Rneut(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rneut(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Rneut(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding tap in per-unit.
    pub fn Get_Tap(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Tap(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Tap(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Tap(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding Number from 1..NumWindings. Update this before reading or setting a sequence of winding properties (R, Tap, kV, kVA, etc.)
    pub fn Get_Wdg(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Wdg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Wdg(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Wdg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Name of an XfrmCode that supplies electircal parameters for this Transformer.
    pub fn Get_XfmrCode(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Transformers_Get_XfmrCode(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_XfmrCode(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Transformers_Set_XfmrCode(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Percent reactance between windings 1 and 2, on winding 1 kVA base. Use for 2-winding or 3-winding transformers.
    pub fn Get_Xhl(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Xhl(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xhl(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Xhl(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent reactance between windigns 1 and 3, on winding 1 kVA base.  Use for 3-winding transformers only.
    pub fn Get_Xht(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Xht(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xht(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Xht(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Percent reactance between windings 2 and 3, on winding 1 kVA base. Use for 3-winding transformers only.
    pub fn Get_Xlt(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Xlt(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xlt(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Xlt(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding neutral reactance [ohms] for wye connections.
    pub fn Get_Xneut(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_Xneut(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xneut(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_Xneut(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding kV rating.  Phase-phase for 2 or 3 phases, actual winding kV for 1 phase transformer.
    pub fn Get_kV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_kV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_kV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Active Winding kVA rating. On winding 1, this also determines normal and emergency current ratings for all windings.
    pub fn Get_kVA(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_kVA(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_kVA(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_kVA(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Complex array of voltages for active winding
    ///
    /// WARNING: If the transformer has open terminal(s), results may be wrong, i.e. avoid using this
    /// in those situations. For more information, see https://github.com/dss-extensions/dss-extensions/issues/24
    pub fn WdgVoltages(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Transformers_Get_WdgVoltages_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// All Winding currents (ph1, wdg1, wdg2,... ph2, wdg1, wdg2 ...)
    ///
    /// WARNING: If the transformer has open terminal(s), results may be wrong, i.e. avoid using this
    /// in those situations. For more information, see https://github.com/dss-extensions/dss-extensions/issues/24
    pub fn WdgCurrents(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Transformers_Get_WdgCurrents_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// All winding currents in CSV string form like the WdgCurrents property
    ///
    /// WARNING: If the transformer has open terminal(s), results may be wrong, i.e. avoid using this
    /// in those situations. For more information, see https://github.com/dss-extensions/dss-extensions/issues/24
    pub fn strWdgCurrents(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Transformers_Get_strWdgCurrents(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Transformer Core Type: 0=Shell; 1=1ph; 3-3leg; 4=4-Leg; 5=5-leg; 9=Core-1-phase
    pub fn Get_CoreType(&self) -> Result<CoreType, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_CoreType(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_CoreType(&self, value: CoreType) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_CoreType(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    /// dc Resistance of active winding in ohms for GIC analysis
    pub fn Get_RdcOhms(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Transformers_Get_RdcOhms(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RdcOhms(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Transformers_Set_RdcOhms(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Complex array with the losses by type (total losses, load losses, no-load losses), in VA
    ///
    /// (API Extension)
    pub fn LossesByType(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Transformers_Get_LossesByType_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }

    /// Complex array with the losses by type (total losses, load losses, no-load losses), in VA, concatenated for ALL transformers
    ///
    /// (API Extension)
    pub fn AllLossesByType(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        unsafe { dss_capi::ctx_Transformers_Get_AllLossesByType_GR(self.ctx_ptr) };
        self.ctx.GetComplexArrayGR()
    }
}

pub struct IVsources<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IVsources <'a> {
}
impl<'a> IVsources<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Vsource names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Vsources_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Vsource objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Vsource active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Vsource by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Vsources_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Vsource.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Vsources_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Vsource active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Vsource; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Vsource by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Phase angle of first phase in degrees
    pub fn Get_AngleDeg(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_AngleDeg(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AngleDeg(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_AngleDeg(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Source voltage in kV
    pub fn Get_BasekV(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_BasekV(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_BasekV(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_BasekV(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Source frequency in Hz
    pub fn Get_Frequency(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_Frequency(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Frequency(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_Frequency(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Number of phases
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Per-unit value of source voltage
    pub fn Get_pu(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Vsources_Get_pu(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_pu(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Vsources_Set_pu(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IWireData<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IWireData <'a> {
}
impl<'a> IWireData<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all WireData names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_WireData_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of WireData objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first WireData active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active WireData by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_WireData_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active WireData.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_WireData_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next WireData active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active WireData; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active WireData by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Emergency ampere rating
    pub fn Get_EmergAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_EmergAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EmergAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_EmergAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Normal Ampere rating
    pub fn Get_NormAmps(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_NormAmps(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_NormAmps(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_NormAmps(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rdc(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Rdc(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rdc(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_Rdc(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_Rac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Rac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Rac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_Rac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRac(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_GMRac(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_GMRac(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_GMRac(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_GMRUnits(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_GMRUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_GMRUnits(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_GMRUnits(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_Radius(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Radius(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Radius(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_Radius(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_RadiusUnits(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_RadiusUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_RadiusUnits(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_RadiusUnits(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn Get_ResistanceUnits(&self) -> Result<LineUnits, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_ResistanceUnits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(unsafe { transmute(result) })
    }

    pub fn Set_ResistanceUnits(&self, value: LineUnits) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_ResistanceUnits(self.ctx_ptr, (value) as i32) };
        self.ctx.DSSError()
    }

    pub fn Get_Diameter(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_Diameter(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Diameter(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_Diameter(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Equivalent conductor radius for capacitance calcs. Specify this for bundled conductors. Defaults to same value as radius.
    pub fn Get_CapRadius(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_WireData_Get_CapRadius(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CapRadius(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_WireData_Set_CapRadius(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IXYCurves<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IXYCurves <'a> {
}
impl<'a> IXYCurves<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all XYCurve names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_XYCurves_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of XYCurve objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first XYCurve active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active XYCurve by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_XYCurves_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active XYCurve.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_XYCurves_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next XYCurve active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active XYCurve; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active XYCurve by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Get/Set Number of points in X-Y curve
    pub fn Get_Npts(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Npts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Npts(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Npts(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set X values as a Array of doubles. Set Npts to max number expected if setting
    pub fn Get_Xarray(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Get_Xarray_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Xarray(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Xarray(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Factor to scale X values from original curve
    pub fn Get_Xscale(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Xscale(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xscale(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Xscale(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Amount to shift X value from original curve
    pub fn Get_Xshift(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Xshift(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Xshift(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Xshift(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/Set Y values in curve; Set Npts to max number expected if setting
    pub fn Get_Yarray(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Get_Yarray_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }

    pub fn Set_Yarray(&self, value: &[f64]) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Yarray(self.ctx_ptr, value.as_ptr(), value.len() as i32) };
        self.ctx.DSSError()
    }

    /// Factor to scale Y values from original curve
    pub fn Get_Yscale(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Yscale(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Yscale(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Yscale(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Amount to shift Y value from original curve
    pub fn Get_Yshift(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_Yshift(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Yshift(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_Yshift(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set X value or get interpolated value after setting Y
    pub fn Get_x(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_x(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_x(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_x(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Set Y value or get interpolated Y value after setting X
    pub fn Get_y(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_XYCurves_Get_y(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_y(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_XYCurves_Set_y(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IYMatrix<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IYMatrix <'a> {
}
impl<'a> IYMatrix<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    pub fn ZeroInjCurr(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_ZeroInjCurr(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn GetSourceInjCurrents(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_GetSourceInjCurrents(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn GetPCInjCurr(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_GetPCInjCurr(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn BuildYMatrixD(&self, BuildOps: i32, AllocateVI: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_BuildYMatrixD(self.ctx_ptr, BuildOps, AllocateVI) };
        self.ctx.DSSError()
    }

    pub fn AddInAuxCurrents(&self, SType: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_AddInAuxCurrents(self.ctx_ptr, SType) };
        self.ctx.DSSError()
    }

    pub fn Get_SystemYChanged(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_YMatrix_Get_SystemYChanged(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SystemYChanged(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_SystemYChanged(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    pub fn Get_UseAuxCurrents(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_YMatrix_Get_UseAuxCurrents(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_UseAuxCurrents(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_UseAuxCurrents(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Sparse solver options. See the enumeration SparseSolverOptions
    pub fn Get_SolverOptions(&self) -> Result<u64, DSSError> {
        let result = unsafe { dss_capi::ctx_YMatrix_Get_SolverOptions(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SolverOptions(&self, value: u64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_SolverOptions(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    pub fn CheckConvergence(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_YMatrix_CheckConvergence(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn SetGeneratordQdV(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_SetGeneratordQdV(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Get_LoadsNeedUpdating(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_YMatrix_Get_LoadsNeedUpdating(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LoadsNeedUpdating(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_LoadsNeedUpdating(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    pub fn Get_SolutionInitialized(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_YMatrix_Get_SolutionInitialized(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_SolutionInitialized(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_SolutionInitialized(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    pub fn Get_Iteration(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_YMatrix_Get_Iteration(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Iteration(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_YMatrix_Set_Iteration(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IZIP<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IZIP <'a> {
}
impl<'a> IZIP<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }
    
    /// Extracts the contents of the file "FileName" from the current (open) ZIP file.
    /// Returns a byte-String.
    ///
    /// (API Extension)
    pub fn Extract(&self, fileName: String) -> Result<Box::<[i8]>, DSSError> {
        let fileName_c = CString::new(fileName).unwrap();
        unsafe { dss_capi::ctx_ZIP_Extract_GR(self.ctx_ptr, fileName_c.as_ptr()); }
        self.ctx.GetInt8ArrayGR()
    }

    /// List of strings consisting of all names match the regular expression provided in regexp.
    /// If no expression is provided (empty String), all names in the current open ZIP are returned.
    ///
    /// See https://regex.sorokin.engineer/en/latest/regular_expressions.html for information on
    /// the expression syntax and options.
    ///
    /// (API Extension)
    pub fn List(&self, regexp: String) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        let regexp_c = CString::new(regexp).unwrap();
        unsafe { dss_capi::ctx_ZIP_List(self.ctx_ptr, &mut data, &mut cnt[0], regexp_c.as_ptr()); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Opens and prepares a ZIP file to be used by the DSS text parser.
    /// Currently, the ZIP format support is limited by what is provided in the Free Pascal distribution.
    /// Besides that, the full filenames inside the ZIP must be shorter than 256 characters.
    /// The limitations should be removed in a future revision.
    ///
    /// (API Extension)
    pub fn Open(&self, FileName: String) -> Result<(), DSSError> {
        let FileName_c = CString::new(FileName).unwrap();
        unsafe { dss_capi::ctx_ZIP_Open(self.ctx_ptr, FileName_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Closes the current open ZIP file
    ///
    /// (API Extension)
    pub fn Close(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_ZIP_Close(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Runs a "Redirect" command inside the current (open) ZIP file.
    /// In the current implementation, all files required by the script must
    /// be present inside the ZIP, using relative paths. The only exceptions are
    /// memory-mapped files.
    ///
    /// (API Extension)
    pub fn Redirect(&self, FileInZip: String) -> Result<(), DSSError> {
        let FileInZip_c = CString::new(FileInZip).unwrap();
        unsafe { dss_capi::ctx_ZIP_Redirect(self.ctx_ptr, FileInZip_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Check if the given path name is present in the current ZIP file.
    ///
    /// (API Extension)
    pub fn Contains(&self, Name: String) -> Result<bool, DSSError> {
        let Name_c = CString::new(Name).unwrap();
        let result = unsafe { (dss_capi::ctx_ZIP_Contains(self.ctx_ptr, Name_c.as_ptr()) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

}

pub struct IGICSources<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IGICSources <'a> {
}
impl<'a> IGICSources<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all GICSource names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_GICSources_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of GICSource objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first GICSource active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active GICSource by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_GICSources_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active GICSource.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_GICSources_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next GICSource active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active GICSource; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active GICSource by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// First bus name of GICSource (Created name)
    pub fn Bus1(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_GICSources_Get_Bus1(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Second bus name
    pub fn Bus2(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_GICSources_Get_Bus2(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Phases, this GICSource element.
    pub fn Get_Phases(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Phases(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Phases(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Phases(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Northward E Field V/km
    pub fn Get_EN(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_EN(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EN(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_EN(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Eastward E Field, V/km
    pub fn Get_EE(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_EE(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_EE(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_EE(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Latitude of Bus1 (degrees)
    pub fn Get_Lat1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Lat1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Lat1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Lat1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Latitude of Bus2 (degrees)
    pub fn Get_Lat2(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Lat2(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Lat2(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Lat2(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Longitude of Bus1 (Degrees)
    pub fn Get_Lon1(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Lon1(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Lon1(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Lon1(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Longitude of Bus2 (Degrees)
    pub fn Get_Lon2(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Lon2(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Lon2(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Lon2(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Specify dc voltage directly
    pub fn Get_Volts(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_GICSources_Get_Volts(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_Volts(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_GICSources_Set_Volts(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}

pub struct IParallel<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IParallel <'a> {
}
impl<'a> IParallel<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    pub fn CreateActor(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_CreateActor(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn Wait(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_Wait(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// Gets/sets the ID of the Active Actor
    pub fn Get_ActiveActor(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_ActiveActor(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ActiveActor(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_Set_ActiveActor(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// (read) Sets ON/OFF (1/0) Parallel features of the Engine
    /// (write) Delivers if the Parallel features of the Engine are Active
    pub fn Get_ActiveParallel(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_ActiveParallel(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ActiveParallel(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_Set_ActiveParallel(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Gets/sets the CPU of the Active Actor
    pub fn Get_ActorCPU(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_ActorCPU(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ActorCPU(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_Set_ActorCPU(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Gets the progress of all existing actors in pct
    pub fn ActorProgress(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Parallel_Get_ActorProgress_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// Gets the status of each actor
    pub fn ActorStatus(&self) -> Result<Box::<[i32]>, DSSError> {
        unsafe { dss_capi::ctx_Parallel_Get_ActorStatus_GR(self.ctx_ptr) };
        self.ctx.GetInt32ArrayGR()
    }

    /// (read) Reads the values of the ConcatenateReports option (1=enabled, 0=disabled)
    /// (write) Enable/Disable (1/0) the ConcatenateReports option for extracting monitors data
    pub fn Get_ConcatenateReports(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_ConcatenateReports(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_ConcatenateReports(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Parallel_Set_ConcatenateReports(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Delivers the number of CPUs on the current PC
    pub fn NumCPUs(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_NumCPUs(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Delivers the number of Cores of the local PC
    pub fn NumCores(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_NumCores(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the number of Actors created
    pub fn NumOfActors(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Parallel_Get_NumOfActors(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }
}

pub struct IStorages<'a> {
    ctx_ptr: *const c_void,
    ctx: &'a DSSContext,
}

unsafe impl<'a> Send for IStorages <'a> {
}
impl<'a> IStorages<'a> {
    pub fn new(ctx: &'a DSSContext) -> Self {
        Self {
            ctx: ctx,
            ctx_ptr: ctx.ctx_ptr,
        }
    }

    /// Array of strings with all Storage names in the circuit.
    pub fn AllNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Storages_Get_AllNames(self.ctx_ptr, &mut data, &mut cnt[0]); }
        self.ctx.GetStringArray(data, cnt)
    }

    /// Number of Storage objects in active circuit.
    pub fn Count(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_Count(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the first Storage active. Returns 0 if no more.
    pub fn First(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_First(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Sets the active Storage by Name.
    pub fn Get_Name(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_Storages_Get_Name(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Gets the name of the active Storage.
    pub fn Set_Name(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Storages_Set_Name(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }

    /// Sets the next Storage active. Returns 0 if no more.
    pub fn Next(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_Next(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Get the index of the active Storage; index is 1-based: 1..count
    pub fn Get_idx(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_idx(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Set the active Storage by index; index is 1-based: 1..count
    pub fn Set_idx(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Storages_Set_idx(self.ctx_ptr, value); }
        self.ctx.DSSError()
    }

    /// Per unit state of charge
    pub fn Get_puSOC(&self) -> Result<f64, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_puSOC(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_puSOC(&self, value: f64) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Storages_Set_puSOC(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Get/set state: 0=Idling; 1=Discharging; -1=Charging;
    ///
    /// Related enumeration: StorageStates
    pub fn Get_State(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_Storages_Get_State(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_State(&self, value: i32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_Storages_Set_State(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }

    /// Array of Names of all Storage energy meter registers
    pub fn RegisterNames(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_Storages_Get_RegisterNames(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Array of values in Storage registers.
    pub fn RegisterValues(&self) -> Result<Box::<[f64]>, DSSError> {
        unsafe { dss_capi::ctx_Storages_Get_RegisterValues_GR(self.ctx_ptr) };
        self.ctx.GetFloat64ArrayGR()
    }
}

pub struct IDSS<'a> {
    ctx_ptr: *const c_void,
    pub ctx: &'a DSSContext,
    pub ActiveCircuit: ICircuit<'a>,
    pub Circuits: ICircuit<'a>,
    pub Error: IError<'a>,
    pub Text: IText<'a>,
    pub DSSProgress: IDSSProgress<'a>,
    pub ActiveClass: IActiveClass<'a>,
    pub Executive: IDSS_Executive<'a>,
    // pub Events: IDSSEvents<'a>,
    pub Parser: IParser<'a>,
    // pub DSSim_Coms: IDSSimComs<'a>,
    pub YMatrix: IYMatrix<'a>,
    pub ZIP: IZIP<'a>,
}

unsafe impl<'a> Send for IDSS <'a> {
}
impl<'a> IDSS<'a> {

    // Initialize all structures of the classic DSS API.
    ///
    /// For creating new independent DSS instances, use the function NewContext.
    pub fn new(ctx: &'a DSSContext) -> Self{
        Self {
            ctx_ptr: ctx.ctx_ptr,
            ctx: ctx,
            ActiveCircuit: ICircuit::new(&ctx),
            Circuits: ICircuit::new(&ctx),
            Error: IError::new(&ctx),
            Text: IText::new(&ctx),
            DSSProgress: IDSSProgress::new(&ctx),
            ActiveClass: IActiveClass::new(&ctx),
            Executive: IDSS_Executive::new(&ctx),
            // Events: IDSSEvents::new(&ctx),
            Parser: IParser::new(&ctx),
            // DSSim_Coms: IDSSimComs::new(&ctx),
            YMatrix: IYMatrix::new(&ctx),
            ZIP: IZIP::new(&ctx),
        }
    }
    
    /// Runs a large string as commands directly in the DSS engine.
    /// Intermediate results are ignored.
    ///
    /// This is a shortcut to `dss.Text.CommandBlock()`, which itself is 
    /// an alternative to `dss.Text.Command()` that accepts multi-line
    /// strings.
    ///
    /// (API Extension)
    pub fn Command(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_Text_CommandBlock(self.ctx_ptr, value_c.as_ptr()); }
        self.ctx.DSSError()
    }
     
    /// Creates a wrapper for a new DSS engine context.
    /// A DSS Context encapsulates most of the global state of the original OpenDSS engine,
    /// allowing the user to create multiple instances in the same process. By creating contexts
    /// manually, the management of threads and potential issues should be handled by the user.
    ///
    /// The Rust implementation wraps several pointers and utility functions. 
    /// Use the result to initialize another IDSS struct via `IDSS::new`
    ///
    /// (API Extension)
    pub fn NewContext(&self) -> Result<DSSContext, DSSError> {
        let newCtxPtr = unsafe { dss_capi::ctx_New() };
        if newCtxPtr.is_null() {
            return Err(DSSError {
                number: 0,
                message: "Could not create a new DSS Context".to_string()
            });
        }
        let ctx = DSSContext::new(newCtxPtr);
        Ok(ctx)    
    }

    pub fn NewCircuit(&self, name: String) -> Result<&ICircuit, DSSError> {
        let name_c = CString::new(name).unwrap();
        unsafe { dss_capi::ctx_DSS_NewCircuit(self.ctx_ptr, name_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(&self.ActiveCircuit)
    }
                       
    /// Get version string for the DSS engine.
    pub fn Version(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Get_Version(self.ctx_ptr)) }.to_string_lossy().into_owned();
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn ClearAll(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_ClearAll(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    /// This is a no-op function, does nothing. Left for compatibility.
    pub fn Reset(&self) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Reset(self.ctx_ptr) };
        self.ctx.DSSError()
    }

    pub fn SetActiveClass(&self, ClassName: String) -> Result<i32, DSSError> {
        let ClassName_c = CString::new(ClassName).unwrap();
        let result = unsafe { dss_capi::ctx_DSS_SetActiveClass(self.ctx_ptr, ClassName_c.as_ptr()) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// This is a no-op function, does nothing. Left for compatibility.
    ///
    /// Calling `Start` in AltDSS/DSS-Extensions is required but that is already
    /// handled automatically, so the users do not need to call it manually.
    ///
    /// On the official OpenDSS, `Start` also does nothing at all in the current
    /// versions.
    pub fn Start(&self, code: i32) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Start(self.ctx_ptr, code) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// List of DSS intrinsic classes (names of the classes)
    pub fn Classes(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_DSS_Get_Classes(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// DSS Data File Path.  Default path for reports, etc. from DSS
    pub fn Get_DataPath(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Get_DataPath(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_DataPath(&self, value: String) -> Result<(), DSSError> {
        let value_c = CString::new(value).unwrap();
        unsafe { dss_capi::ctx_DSS_Set_DataPath(self.ctx_ptr, value_c.as_ptr()) };
        self.ctx.DSSError()
    }

    /// Returns the path name for the default text editor.
    pub fn DefaultEditor(&self) -> Result<String, DSSError> {
        let result = unsafe { CStr::from_ptr(dss_capi::ctx_DSS_Get_DefaultEditor(self.ctx_ptr)).to_string_lossy().into_owned() };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of Circuits currently defined
    pub fn NumCircuits(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Get_NumCircuits(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of DSS intrinsic classes
    pub fn NumClasses(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Get_NumClasses(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// Number of user-defined classes
    pub fn NumUserClasses(&self) -> Result<i32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Get_NumUserClasses(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    /// List of user-defined classes
    pub fn UserClasses(&self) -> Result<Box::<[String]>, DSSError> {
        let mut cnt: [i32; 4] = [0, 0, 0, 0];
        let mut data: *mut *mut c_char= 0 as *mut *mut c_char;
        unsafe { dss_capi::ctx_DSS_Get_UserClasses(self.ctx_ptr, &mut data, &mut cnt[0]) };
        self.ctx.GetStringArray(data, cnt)
    }

    /// Gets/sets whether text output is allowed
    pub fn Get_AllowForms(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_AllowForms(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllowForms(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_AllowForms(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Gets/sets whether running the external editor for "Show" is allowed
    ///
    /// AllowEditor controls whether the external editor is used in commands like "Show".
    /// If you set to 0 (false), the editor is not executed. Note that other side effects,
    /// such as the creation of files, are not affected.
    ///
    /// (API Extension)
    pub fn Get_AllowEditor(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_AllowEditor(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllowEditor(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_AllowEditor(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// LegacyModels was a flag used to toggle legacy (pre-2019) models for PVSystem, InvControl, Storage and
    /// StorageControl.
    /// In the official OpenDSS version 9.0, the old models were removed. They were temporarily present here
    /// but were also removed in DSS C-API v0.13.0.
    ///
    /// **NOTE**: this property will be removed for v1.0. It is left to avoid breaking the current API too soon.
    ///
    /// (API Extension)
    pub fn Get_LegacyModels(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_LegacyModels(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_LegacyModels(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_LegacyModels(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// If disabled, the engine will not change the active working directory during execution. E.g. a "compile"
    /// command will not "chdir" to the file path.
    ///
    /// If you have issues with long paths, enabling this might help in some scenarios.
    ///
    /// Defaults to True (allow changes, backwards compatible) in the 0.10.x versions of DSS C-API.
    /// This might change to False in future versions.
    ///
    /// This can also be set through the environment variable DSS_CAPI_ALLOW_CHANGE_DIR. Set it to 0 to
    /// disallow changing the active working directory.
    ///
    /// (API Extension)
    pub fn Get_AllowChangeDir(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_AllowChangeDir(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllowChangeDir(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_AllowChangeDir(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// If enabled, the `DOScmd` command is allowed. Otherwise, an error is reported if the user tries to use it.
    ///
    /// Defaults to False/0 (disabled state). Users should consider DOScmd deprecated on DSS-Extensions.
    ///
    /// This can also be set through the environment variable DSS_CAPI_ALLOW_DOSCMD. Setting it to 1 enables
    /// the command.
    ///
    /// (API Extension)
    pub fn Get_AllowDOScmd(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_AllowDOScmd(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_AllowDOScmd(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_AllowDOScmd(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// If enabled, in case of errors or empty arrays, the API returns arrays with values compatible with the
    /// official OpenDSS COM interface.
    ///
    /// For example, consider the function `Loads_Get_ZIPV`. If there is no active circuit or active load element:
    /// - In the disabled state (COMErrorResults=False), the function will return "[]", an array with 0 elements.
    /// - In the enabled state (COMErrorResults=True), the function will return "[0.0]" instead. This should
    /// be compatible with the return value of the official COM interface.
    ///
    /// Defaults to True/1 (enabled state) in the v0.12.x series. This will change to false in future series.
    ///
    /// This can also be set through the environment variable DSS_CAPI_COM_DEFAULTS. Setting it to 0 disables
    /// the legacy/COM behavior. The value can be toggled through the API at any time.
    ///
    /// (API Extension)
    pub fn Get_COMErrorResults(&self) -> Result<bool, DSSError> {
        let result = unsafe { (dss_capi::ctx_DSS_Get_COMErrorResults(self.ctx_ptr) != 0) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_COMErrorResults(&self, value: bool) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_COMErrorResults(self.ctx_ptr, bool_to_u16(value)) };
        self.ctx.DSSError()
    }

    /// Controls some compatibility flags introduced to toggle some behavior from the official OpenDSS.
    ///
    /// **THESE FLAGS ARE GLOBAL, affecting all DSS engines in the process.**
    ///
    /// The current bit flags are:
    ///
    /// - 0x1 (bit 0): If enabled, don't check for NaNs in the inner solution loop. This can lead to various errors.
    ///     This flag is useful for legacy applications that don't handle OpenDSS API errors properly. Through the
    ///     development of DSS-Extensions, we noticed this is actually a quite common issue.
    /// - 0x2 (bit 1): Toggle worse precision for certain aspects of the engine. For example, the sequence-to-phase
    ///     (`As2p`) and sequence-to-phase (`Ap2s`) transform matrices. On DSS C-API, we fill the matrix explicitly
    ///     using higher precision, while numerical inversion of an initially worse precision matrix is used in the
    ///     official OpenDSS. We will introduce better precision for other aspects of the engine in the future,
    ///     so this flag can be used to toggle the old/bad values where feasible.
    /// - 0x4 (bit 2): Toggle some InvControl behavior introduced in OpenDSS 9.6.1.1. It could be a regression
    ///     but needs further investigation, so we added this flag in the time being.
    ///
    /// These flags may change for each version of DSS C-API, but the same value will not be reused. That is,
    /// when we remove a compatibility flag, it will have no effect but will also not affect anything else
    /// besides raising an error if the user tries to toggle a flag that was available in a previous version.
    ///
    /// We expect to keep a very limited number of flags. Since the flags are more transient than the other
    /// options/flags, it was preferred to add this generic function instead of a separate function per
    /// flag.
    ///
    /// Related enumeration: DSSCompatFlags
    ///
    /// (API Extension)
    pub fn Get_CompatFlags(&self) -> Result<u32, DSSError> {
        let result = unsafe { dss_capi::ctx_DSS_Get_CompatFlags(self.ctx_ptr) };
        self.ctx.DSSError()?;
        Ok(result)
    }

    pub fn Set_CompatFlags(&self, value: u32) -> Result<(), DSSError> {
        unsafe { dss_capi::ctx_DSS_Set_CompatFlags(self.ctx_ptr, value) };
        self.ctx.DSSError()
    }
}
