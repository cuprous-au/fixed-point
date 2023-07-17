# fixed-point

This library defined a fixed point numeric type `FixedPoint<S>(S)` and its operations.

The type is specialised for embedded software.  As a tuple-struct, `FixedPoint` is serialized cleanly and compactly.  The library is `no-std` with dependencies on `serde` and, optionally `defmt`.  

The maximum size of the fixed point representation is deliberately limited to `i32` and floating point conversions use `f32`.This simplifies the code and ensures it runs well on a microcontroller. (These limits can be changed by changing `Float` and `Repr` type aliases.)

Type parameter `S` effectively represents the units of the number, e.g. `FixedPoint<Volt>` is a fixed point number representing a voltage.  A trait `Spec` implemented for `S` gives the scaling and precision of the fixed point representation. Several types implementing Spec are provided in module `unit`.

