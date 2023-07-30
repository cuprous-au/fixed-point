# fixed-point

This library defines a fixed point numeric type `FixedPoint<S>(S)` and its operations.

The type is intended for embedded software.  As a tuple-struct, `FixedPoint` is serialized cleanly and compactly.  The library is `no-std` with dependencies on `serde` and, optionally `defmt`.  

The maximum size of any fixed point representation is deliberately limited to 32 bits (signed) or 31 bits (unsigned) and floating point conversions use `f32`.  This simplifies the code and ensures it runs well on a microcontroller. These limits can be raised by changing `Float` and `Fixed` type aliases.

The parameter `S` is the type of the representation on the wire and in memory.  A trait `Spec` implemented for `S` gives its scaling and precision. Several types implementing Spec are provided in module `unit`.  

For example, `struct Volt(i32)` defines an i32 representation of voltage. Then `impl Spec for Volt` gives the precision of this representation as one decimal place.

The traits defined on FixedPoint<S> provide all representations with:

- Conversions to and from Float.
- Operations add and substract among `FixedPoint` of the same type and scaling by Float (ie the algebra of a linear space).
- Equality and ordering.
- Debug, Display and defmt::Format.
- Parsing from strings.
- Serde.

