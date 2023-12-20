# AltDSS-Rust
A crate with Rust bindings (based on bindgen) to AltDSS/DSS C-API, the alternative OpenDSS implementation from DSS-Extensions

Initial tracking is done at https://github.com/dss-extensions/dss-extensions/issues/34  
The issue tracker here will be used after the initial issue is closed.

The current milestones are:

1. Provide access to the classic API, organized to mimic the original OpenDSS COM classes (note that COM objects are **not** used on DSS-Extensions; COM is a Windows-only technology).

    - Expose all of the classic DSS C-API functions.
    - Try the upcoming wrapper for the official OpenDSSDirect.DLL. The DLL was rewritten recently (but it's still Windows-only right now).

3. Provide access to the new API, which exposes all DSS classes and functions more conveniently.

This project is then expected to allow using both the official OpenDSS, which its limitations, and DSS-Extensions implementation (aka AltDSS).
