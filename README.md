# fixed-point

This library defines a fixed point numeric type `FixedPoint<R>(R)` and its operations.

The type is intended for embedded software. As a tuple-struct, `FixedPoint` is serialised cleanly and compactly. The library is `no-std` with dependencies on `serde` and, optionally `defmt`.  

The maximum size of any fixed point representation is deliberately limited to 32 bits (signed) or 31 bits (unsigned) and floating point conversions use `f32`. This simplifies the code and ensures it runs well on a microcontroller. These limits can be raised by changing `Float` and `Fixed` type aliases.

The parameter `R` is the type of the representation on the wire and in memory. A trait `Spec` implemented for `R` gives its scaling and precision. Several types implementing Spec are provided in module `unit`.  

For example, `struct Volt(i32)` defines an i32 representation of voltage. Then `impl Spec for Volt` gives the precision of this representation as one decimal place.

The traits defined on FixedPoint<R> provide all representations with:

- Conversions to and from Float.
- Operations add and substract among `FixedPoint` of the same type and scaling by Float (ie the algebra of a linear space).
- Equality and ordering.
- Debug, Display and defmt::Format.
- Parsing from strings.
- Serde.

## Contribution policy

Contributions via GitHub pull requests are gladly accepted from their original author. Along with any pull requests, please state that the contribution is your original work and that you license the work to the project under the project's open source license. Whether or not you state this explicitly, by submitting any copyrighted material via pull request, email, or other means you agree to license the material under the project's open source license and warrant that you have the legal authority to do so.

## License

This code is open source software licensed under the [Apache-2.0 license](./LICENSE).

© Copyright [Cuprous P/L](https://www.cuprous.com.au/), 2023
