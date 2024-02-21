# AltDSS-Rust
A crate with Rust bindings (currently based on bindgen) to AltDSS/DSS C-API, the alternative OpenDSS implementation from DSS-Extensions.

Initial tracking is done at https://github.com/dss-extensions/dss-extensions/issues/34  
The issue tracker here will be used after the initial issue is closed.

The current milestones are:

1. **(DONE)** Provide access to the classic API, organized to mimic the original OpenDSS COM classes (note that COM objects are **not** used on DSS-Extensions; COM is a Windows-only technology).
    - Expose all of the classic DSS C-API functions.

2. Try the upcoming wrapper for the official OpenDSSDirect.DLL. The DLL was rewritten recently (but it's still Windows-only right now).

3. Provide access to the new API, which exposes all DSS classes and functions more conveniently.

This project is then expected to allow using both the official OpenDSS, within its limitations, and the DSS-Extensions implementation (aka AltDSS/DSS C-API).

----

# Current status

- Initial testing done on x64 Linux, based on DSS C-API v**0.14.1**.
- Exposes nearly all of the classic OpenDSS API and most of the classic API extensions in AltDSS/DSS C-API.
- Organized in two main high-level structs: a `common::DSSContext` and `classic::IDSS`. `IDSS` mimics the COM organization, per [DSS-Python](https://dss-extensions.org/dss_python/dss/#module-dss.IDSS) (plus [DSS Sharp](https://dss-extensions.org/dss_sharp/html/6ec40528-724b-089f-8ac5-ce043f8f981f.htm) and DSS MATLAB) and the official implementation per https://opendss.epri.com/COMInterface.html
- Future interfaces, exposed in other modules, will reuse the `DSSContext` struct.
- Nearly all methods return a `Result<sometype, DSSError>` since DSS errors could be produced by nearly all DSS C-API functions. Future Rust versions could make this more comfortable.
- Multi-threading confirmed to work fine on x64 Linux. Tests pending for other platforms.

Pending tasks and decisions:

- Tests and docs; for the general API functions, the best is to document the DSS C-API header and automate porting those to all other projects (this is partially done right now; could be more integrated).
- ~~Adjust licensing (move to Apache 2)~~
- ~~Merge relevant code from the OpenEnergySolutions repositories~~
- Mirror in Rust the default behavior from https://github.com/dss-extensions/AltDSS-Go/issues/2
- ~~Wait for DSS C-API 0.14.0 to be released~~
- ~~Identifiers: decide if/what/how to adapt the naming style, original vs. Rust's snake case (for most things).~~
    - The closer the names are to the other bindings (the other DSS-Extensions, and the official OpenDSS COM), the easier it would be to port code and ease the transition from other programming languages.
    - OpenDSS uses units as names in lot of places. Capitalization is important for those. Although `kWh` is acceptable as `kwh`, many other quantities would be ambiguous in lowercase: `kV` vs `Kv` (kilovolts vs some K constant), `MV` vs `mV` (megavolts vs millivolts). In other words, although there are some bad function names in OpenDSS and DSS C-API, the correct capitalization of units does provide context that could be enough to avoid reading for the documentation in many situations.
        - The warning from the compiler are not useful in most cases, so we would need to check each function name and document the changes. From one of the examples, there is a variable named `losses_kWh` and the compiler suggestion is `losses_k_wh`.
        - This will only get worse (more manual work) when we expose all 50+ DSS object types. For a motivating example, see https://github.com/dss-extensions/dss_python/blob/0.15.0b1/tests/test_obj.py#L500 -- in Python, which already exposes all objects, the names are intentionally kept as close as possible to the DSS property names (and follows the renaming we're doing for the upcoming DSS C-API release), in fact it allows easily transforming a .DSS script to a Python script without too much hassle. If we were to adapt the names for each programming language, it would be challenging to both maintain and use.
    - Rust is a typed language and IDE integration is already quite good; that's a good argument against worrying about the naming convention.
    - Would there be any overhead if we decide to duplicate everything (as wrappers) to rename the methods for those who really value this kind of convention?


# Getting started

Currently, this is not published to `crates.io` and there is no special handling for the DSS C-API binaries.  

This could change shortly, so remember to check later.

Some direct instructions to get up and running (assuming Rust and tools are already installed):

(NOTE: this needs to be updated, but the general idea still applies)

```shell
mkdir altdss-tests
cd altdss-tests
wget -qO- https://github.com/dss-extensions/dss_capi/releases/download/0.14.1/dss_capi_0.14.1_linux_x64.tar.gz | tar zxv
git clone --depth=1 https://github.com/dss-extensions/electricdss-tst
git clone https://github.com/dss-extensions/altdss-rust
export LD_LIBRARY_PATH=`pwd`/dss_capi/lib/linux_x64
cd altdss-rust
cargo build
cargo run --example ieee13
cargo run --example list_props
cargo run --example parallel
```

# Examples

Check the [`examples`](https://github.com/dss-extensions/AltDSS-Rust/tree/main/examples) folder.