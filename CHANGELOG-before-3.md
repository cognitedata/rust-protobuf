# Changelog

Alpha versions of branch 3 are published. They are not guaranteed to have backwards compatibility.
Stable versions are 2.xx still supported.

## [3] - Unreleased

## [3.0.0] - 2020-05-01

* New stable version released.

## [3.0.0-alpha.14] - 2022-05-01

* JSON support is moved into a separate crate `protobuf-json-mapping`
* Generated code for well known types is more similar to regular protobuf now
  (well known type modules are now placed as is in `well_known_types` module instead of being flattened).
* `file_descriptor_proto()` function is private now in generated code.
  Descriptor can be obtained by calling `file_descriptor().proto()`.
* `file_descriptor()` now returns `&'static FileDescriptor`
* Hide `UnknownValues` from public API (keep `UnknownFields`)

## [3.0.0-alpha.13] - 2022-05-01

* More efficient reflective operations
* Fewer internals exposed from public API (e.g. `ProtobufType` is no longer public)

## [3.0.0-alpha.12] - 2022-04-30

* keywords are now [escaped with `_` suffix](https://github.com/stepancheg/rust-protobuf/issues/618)
* generate trivial `is_initialized` functions when possible

## [3.0.0-alpha.11] - 2022-04-29

* all fields are public now in generated messages
* `protobuf-support` crate added. This crate contains utilities shared by other protobuf crates.
  This crate has no public API.

## [3.0.0-alpha.10] - 2022-04-11

* remove `expose_oneof` codegen option (it is on by default for a long time)
* remove unused serde-related options from `rustproto.proto`
* in generated code, enum variant names are converted to camel case
* generate `SpecialFields` field in messages instead of a pair (`unknown_fields`, `cached_size`)
* reflection operations like `nested_messages` now return iterator instead of `Vec`
* replace `fn Enum::values()` with `const Enum::VALUES`
* remove `EnumFull::enum_descriptor()`, does not provide much value over `enum_descriptor_static`
* rename `EnumFull::enum_descriptor_static` -> `enum_descriptor`
* fix `EnumFull::descriptor` for enums with `allow_alias`
* rename `MessageDescriptor::enums` to `nested_enums`
* rename `MessageDescriptor::descriptor_static` to `descriptor`
* add `EnumDescriptor::enclosing_message()`
* `Message` now requires `PartialEq`

## [3.0.0-alpha.9] - 2022-04-04

- Unimplement `DerefMut` for `MessageField`: `DerefMut` which modifies the object state is too dangerous.
  `Deref` is kept though. `Deref` returns an empty instance if field is not set.
- Added `Message::clear`, removed trait `Clear`
- `Lazy` (which is used in generated code) is now implemented with `once_cell` crate.
- protobuf 3 experimental `optional` fields are implemented

## [3.0.0-alpha.8] - 2022-02-21

- Lite runtime generation is restored. When lite runtime requested, code is generated without reflection support.
- `lite` option can be specified when using `protoc-gen-rust` plugin similarly to how
  [C++ or Java do](https://github.com/protocolbuffers/protobuf/issues/6489).

## [3.0.0-alpha.7] - 2022-02-20

- Getters are now generated without `get_` prefix. `get_` prefix also removed from public API functions.
- Reflection API now supports `ServiceDescriptor`. Which can be used to generate code for gRPC for example.
- Message size computation uses u64 now. It is explicit error now on attempt to serialize a message larger than 2GiB.
- Binary message parsing now switches by tag, not by field name. It is faster.

## [3.0.0-alpha.6] - 2022-02-08

- [Fixed invalid aliasing and uninitialized memory access](https://github.com/stepancheg/rust-protobuf/pull/592)
- `MessageField` now implements `Deref` and `DerefMut`
- Slightly more compact generated code
- serde is no longer supported natively. See explanations [in the issue](https://github.com/stepancheg/rust-protobuf/issues/519).
- Rename `ProtobufError` to `Error` and make it opaque type. Rename `ProtobufResult` to `Result`.
- Rename `ProtobufEnum` to `Enum` and `ProtobufEnumOrUnknown` to `EnumOrUnknown`
- `gen_mod_rs` option is `true` by default now: code generator now generates `mod.rs` with modules
- `carllerche` options renamed to `tokio` (since `bytes` crate now lives in `tokio` org)
- `protobuf-parse` API cleanup (API used to parse `.proto` files, not protocol buffers data files)
- Remove `protoc` crate, most of it is incorprotated into `protobuf-parse` crate, and the rest is not very useful
- Generated enums for `oneof` are marked `#[non_exhaustive]`

## [3.0.0-alpha.5] - 2022-02-02

- Dynamic messages work (but not tested enough)

## [3.0.0-alpha.4] - 2022-02-02

- `Display` for message now outputs text format, and `Debug` for message does standard rust `#[derive(Debug)]`.
- Smaller generated code (common code snippets extracted into the library runtime)
- Improvements in dynamic messages

## [3.0.0-alpha.3] - 2022-02-01

- Remove `protobuf-codegen-pure` and `protoc-rust` crates. Now all codegen
  (pure or with `protoc` is done using `protobuf-codegen` crate).
- `LazyV2` (internal utility for rust-protobuf) now implements `Drop`.
- Default `.proto` parser in `protobuf-codegen` is now pure-rust (not using `protoc` command)
- Dynamic messages mostly work

## [3.0.0-alpha.2] - 2021-11-01

- Use `always_output_default_values` option
  [to output empty array for repeated fields when serializing to JSON](https://github.com/stepancheg/rust-protobuf/pull/550)
- Switch error handling to `thiserror` and `anyhow` crates
- `.proto` file parsing now lives in a separate crate `protobuf-parse`

## [3.0.0-alpha.1] - 2021-10-24

### Backward compatibility

Version 3.0 is backward incompatible with 2.0 version. Changes are listed here:

- Enum fields are now
  [generated as `ProtobufEnumOrUnknown<E>`](https://github.com/stepancheg/rust-protobuf/issues/233)
- Nested messages are now
  [generated in nested mods](https://github.com/stepancheg/rust-protobuf/commit/da2e25dc6c20efcea3893c78e587b43b89da9528)
- Getters are no longer generated for public fields
- Field accessors (getters and setters) are not generated by default for public fields
- [Remove global `parse_length_delimited*`
  functions](https://github.com/stepancheg/rust-protobuf/commit/91c0875e909cdc0648256f0c45cd4a9ade1e4fa0)
- `PartialEq` with large number of fields
  [now panics](https://github.com/stepancheg/rust-protobuf/commit/4f1ca564a00e85b6e3821e91aace71ccb6592bf5).
  Previosly it could cause stack overflow in the Rust compiler.
- [Change `message_down_cast*` functions
  signatures](https://github.com/stepancheg/rust-protobuf/commit/a05a4216fc3305c67b7a2d19011be3bd503d5166)
- [Remove `descriptorx` from `protobuf`
  crate](https://github.com/stepancheg/rust-protobuf/commit/4e8896645c3e017ac91f529cb69ce76b002f6fc1)

### New features

- [Option to store repeated message fields in `Vec` instead of `RepeatedField`](
  https://github.com/stepancheg/rust-protobuf/issues/280). This option may be turned on by default later.
- Similarly, [option to store singular field on `Option` instead of `SingularPtrField`](
  https://github.com/stepancheg/rust-protobuf/issues/300), which also may be turned on by default later.
- `generate_getter` option to disable generation of getters functions.
- [Flush `CodedOutputStream` on `drop`](https://github.com/stepancheg/rust-protobuf/commit/0e9cc5964c2731a771725bcf70125d3eb1c273b3)

## [2.27] - Unreleased

- `protoc-bin-vendored` now lives in a [separate repository](https://github.com/stepancheg/rust-protoc-bin-vendored)

## [2.27.1] - 2022-02-05

- Min rust version bumped back to 1.52.1

## [2.27.0] - 2022-02-03

- Bump min rust version to 1.55.0
- [Fixed invalid aliasing and uninitialized memory access](https://github.com/stepancheg/rust-protobuf/pull/592)

## [2.26.1] - 2022-02-01

- Documentation

## [2.26.0] - 2020-01-31

- Min supported Rust version is 1.52.1.
- [Fix `SingularField::unwrap_or_default`](https://github.com/stepancheg/rust-protobuf/issues/572)
- [`serde_rename_all` codegen option](https://github.com/stepancheg/rust-protobuf/pull/586)


## [2.25.2] - 2021-10-24

- [don't panic on short buffer](https://github.com/stepancheg/rust-protobuf/pull/571)

## [2.25.1] - 2021-08-21

- [Allow leading zeros in float literals exponent](https://github.com/stepancheg/rust-protobuf/pull/565)

## [2.25.0] - 2021-08-08

- [Implement `Extend` and more `PartialEq` for `RepeatedField`](https://github.com/stepancheg/rust-protobuf/issues/561)
- [Make code generation deterministic](https://github.com/stepancheg/rust-protobuf/issues/562)

## [2.24.2] - 2021-08-08

- [Fix reflective enum access](https://github.com/stepancheg/rust-protobuf/issues/564)

## [2.24.1] - 2021-06-11

- [Stopgap support for Apple M1 for protoc-bin-vendored](https://github.com/stepancheg/rust-protobuf/pull/556)

## [2.24.0] - 2021-06-11

- Accidentally published version roughtly equivalent to 2.23.0

## [2.23.0] - 2021-04-24

- Update bundled `protoc` version to 3.15.8

## [2.22.1] - 2021-03-18

- Work around [some breaking changes in Rust nightly](https://github.com/stepancheg/rust-protobuf/issues/551)

## [2.22.0] - 2021-02-06

- Slightly better prefix stripping algorithm in pure rust codegen: "." is now considered to be a prefix for "foo/bar.proto".
  (Probably Rust stdlib should do that out of the box)
- Update bundled version of `protoc` to version 3.14.0
- `protoc-bin-vendored` now includes `google/**.proto` files, so `import "google/protobuf/Timestamp.proto"`
  should work with this crate without relying on external files

## [2.21.0] - 2021-02-06

- [Use fully qualified crate name in serde derives](https://github.com/stepancheg/rust-protobuf/pull/545)

## [2.20.0] - 2021-01-06

- update `bytes` crate dependency version from 0.6 to 1.0

## [2.19.0] - 2021-01-05

- Add `UnknownFields::remove`
- Fix several inconsistencies between `protoc` parser and pure rust parser
- `protobuf::parse_from*` functions are deprecated, use `protobuf::Message::parse_from*` instead
- impl `IntoIterator` for `&mut RepeatedField<T>` and for `RepeatedField<T>`
- update `bytes` crate dependency version from 0.5 to 0.6

## [2.18.2] - 2021-03-28

- Backport `rust::skip` workaround from 2.22.1

## [2.18.1] - 2020-11-22

- [map field referencing nested messages](https://github.com/stepancheg/rust-protobuf/issues/531)

## [2.18.0] - 2020-10-04

- `gen_mod_rs` codegen option can be used to generate `.rs` files friendlier to generating files in `$OUT_DIR`

## [2.17.0] - 2020-08-12

- `protoc` crate now depends on `which` crate for `protoc` binary lookup
- pure rust codegen output adjusted to be closer to `protoc`-command based output
- `RepeatedField::retain` is implemented

## [2.16.2] - 2020-07-06

- Fix compilation when feature `with-bytes` is enabled

## [2.16.1] - 2020-07-06

- Mute self-deprecation warning when compiling rust-protobuf

## [2.16.0] - 2020-07-06

- Generated repeated and message fields for proto2 are
  [public now](https://github.com/stepancheg/rust-protobuf/commit/f391e9ae0968ae08f4a68798c3b3f25852590150)
  (for proto3 all fields are already public).
- Generated code no longer contains `unsafe`
- Minor changes in generated files

## [2.15.1] - 2020-06-26

- [Use full name of Box in generated code](https://github.com/stepancheg/rust-protobuf/pull/492)

## [2.15.0] - 2020-06-21

- Min supported rust version if 1.44.1 now
- Replace deprecated `#![cfg_attr(rustfmt, rustfmt_skip)]` with `#![rustfmt::skip]`

## [2.14.0] - 2020-04-12

- Rename `protoc_rust::Args` to `protoc_rust::Codegen`
- Rename `protobuf_codegen_pure::Args` to `protobuf_codegen_pure::Codegen`
- [`protoc-bin-vendored` crate](https://docs.rs/protoc-bin-vendored) introduced

## [2.13.0] - 2020-04-09

- [Implement Any::pack,is,unpack
  operations](https://github.com/stepancheg/rust-protobuf/commit/e91bf7eb20abe68a7b29264b864e2cecbbb3f769)
- Rename `protoc::Args` to `protoc::ProtocLangOut`

## [2.12.0] - 2020-03-26

- Generated code for reflection now references messages by Protobuf name, not by Rust name.

## [2.11.0] - 2020-03-23

### Backward compatibility

- [Rename](https://github.com/stepancheg/rust-protobuf/commit/65667cb6e75e91027d595e8be1bce25cc29d7c88)
  `ProtobufValueRef` to `ReflectValueRef`. Old name is kept for a while.

### Other changes

- Generated code [now uses](https://github.com/stepancheg/rust-protobuf/commit/f362d93c3a0f2405115f92a7f6bb08ad058fbf02)
  associated constant `Lazy::INIT` for `Lazy` intialization instead of initializing fields directly.

## [2.10.3] - 2020-03-23

- Oneof names are escaped now in generated code

## [2.10.2] - 2020-03-01

- Added `dyn` to a list of rust keyword needed escaping

## [2.10.1] - 2020-01-14

- Remove accidentally slipped into 2.10 [version check](https://github.com/stepancheg/rust-protobuf/issues/466)

## [2.10.0] - 2020-01-01

### Backward compatibility

- Minimum supported Rust version is 1.40.0 now
- `bytes` crate [upgraded to 0.5](https://github.com/stepancheg/rust-protobuf/issues/465)

## Other changes

- [`ProtobufError` now provides error message in `Display` instead of
  `Error::description`](https://github.com/stepancheg/rust-protobuf/commit/24c20a0503c0946836d3044dad524757bac2cc8a)
- [`Debug` is now implemented for `EnumValueDescriptor` and
  `ReflectValueRef`](https://github.com/stepancheg/rust-protobuf/commit/0e6a2f4c50f07d2c6f8007abd469daa08bc09b9c)

## [2.9] - Unreleased

Changes in 2.9 branch are not included in 2.10.

## [2.9.0] - (2019-09-30) yanked

### Backward compatibility

- Minimum supported Rust version is 1.34.2 now
- Generated code by protobuf compiler is now compatible
  only with exactly the same version of protobuf library.
  Note you can use
  [pure rust protobuf compiler](https://docs.rs/protobuf-codegen-pure)
  to avoid dependency on `protoc` binary.
- `UnknownFields::fields` field is
  [no longer public](https://github.com/stepancheg/rust-protobuf/commit/8ad35ecaa0accaa251f9f29708e4ed3b96f2351b)

### Big changes

- Text format and JSON printing and parsing is now implemented
- Mutation reflection is implemented
- All fields are public now except optional or repeated fields when `syntax = "proto2"`,
  but message fields are public even when `syntax = "proto2"`

### Other changes

- [`Box<dyn Message>` now implements
  `Clone`](https://github.com/stepancheg/rust-protobuf/commit/08aedca14f6a4cf8bb85c3e82d2dae05cddf57b8)
- Generated code is slightly cleaner now (does not use `use` statements)
- Generated code no longer uses unsafe (protobuf library still does)
- Add a couple functions to reflection

## [2.8] - Unreleased

## [2.8.2] - 2019-12-31

- [Add `async` and `await` to rust keywords](https://github.com/stepancheg/rust-protobuf/pull/461)

## [2.8.1] - 2019-09-06

- [hidden lifetime parameters in types are deprecated](https://github.com/stepancheg/rust-protobuf/issues/435)

## [2.8.0] - 2019-07-22

- [Fix `dyn Trait` warning (again)](https://github.com/stepancheg/rust-protobuf/issues/414)

## [2.7.0] - 2019-07-02
- Minimum supported Rust version
  [is 1.27](https://github.com/stepancheg/rust-protobuf/commit/f51811e8376f7c46b433479903a1bd8670246aa0).
  This version stabilizes `dyn Trait` syntax.
- `inside_protobuf` option is added which slightly modifies generated code inside protobuf.
  Should not affect users.
- [Generated files are now compatible only with the same version of
  runtime](https://github.com/stepancheg/rust-protobuf/commit/2ab4d50c27c4dd7803b64ce1a43e2c134532c7a6)
- [Fixed codegen for mutually recursive messages with
  oneofs](https://github.com/stepancheg/rust-protobuf/pull/420)
- [Clippy annotations are now generated as `#[allow(clippy::all)]` instead of
  `#[allow(clippy)]`](https://github.com/stepancheg/rust-protobuf/pull/332)

## [2.6.2] - 2019-06-03
- Fix [OOM on malformed input for fields of type
  `bytes::Bytes`](https://github.com/stepancheg/rust-protobuf/issues/411)

## [2.6.1] - 2019-05-27

- [Fix `Hash` of
  `UnknownFields`](https://github.com/stepancheg/rust-protobuf/commit/7f285cc42990e34bd8a489519aaae216a93584cf)
- Improve rustdoc a little

## [2.6.0] - 2019-05-19

- [lite_runtime rust-protobuf option](https://github.com/stepancheg/rust-protobuf/pull/399)
- Fix [OOM on malformed input](https://github.com/stepancheg/rust-protobuf/issues/411)
- Minimum supported Rust version is [1.26](https://github.com/stepancheg/rust-protobuf/commit/71f09ae92e86be2ce439e71452c3ca1749a4bda7)
- [Implement `Hash` for
  `UnknownFields`](https://github.com/stepancheg/rust-protobuf/commit/113babc8c56deb7e2453f0d11c2bfc21134d540f)

## [2.5.0] - 2019-04-15

- `generate_accessors` options to disable generation of accessor functions (except getters).
- [`Default` is now implented for all `&MyMessage`
  types](https://github.com/stepancheg/rust-protobuf/commit/c026777976c895898fb50bc7c52802967bd33af5#diff-405e0ba76bb0afaaa4e11e89bc4bb943R4)
- [`Debug` is now implemented for oneof enums](https://github.com/stepancheg/rust-protobuf/issues/397)
  
## [2.4.2] - 2019-03-29

- [Fix well-known types codegen](https://github.com/stepancheg/grpc-rust/issues/129)
- [More extension types are supported now](https://github.com/stepancheg/rust-protobuf/issues/392)

## [2.3.1] - 2019-03-05

- [Fix codegen when `syntax = "proto2"` and `with-bytes` option and string field with default
  value](https://github.com/stepancheg/rust-protobuf/issues/395)

## [2.3.0] - 2019-01-30

- [`Default` is implemented for enums even in proto2](
  https://github.com/stepancheg/rust-protobuf/commit/166966627ebc1e5ce650acd1593489e52757178e)
- [Fix codegen where map value is an enum](https://github.com/stepancheg/rust-protobuf/issues/376)

## [2.2.5] - 2019-01-20

- [Escape extension field names](https://github.com/stepancheg/rust-protobuf/issues/371)

## [2.2.4] - 2019-01-13

- [Replace tempdir dependency with tempfile](https://github.com/stepancheg/rust-protobuf/pull/374)
- [Fix serialization of signed int map keys or values](https://github.com/stepancheg/rust-protobuf/pull/372)

## [2.2.2] - 2018-12-29

- [Fix codegen on Windows](https://github.com/stepancheg/rust-protobuf/issues/356)

## [2.2.1] - 2018-12-25

- [Fix panic with oneof and bytes codegen](https://github.com/stepancheg/rust-protobuf/issues/362)

## [2.2.0] - 2018-11-17

- [Implement](https://github.com/stepancheg/rust-protobuf/commit/c0856a0b7b9a74224d535ecb691c46669c86a878)
  `From<Option<T>> for SingularPtrField<T>`
  
## [2.1.5] - 2019-01-13

- [Fix serialization of signed int map keys or values](https://github.com/stepancheg/rust-protobuf/pull/372)

## [2.1.4] - 2018-11-01

- Revert clippy annotations

## [2.1.3] - 2018-10-31

- [Replace old clippy annotations with recommended new
  annotations](https://github.com/stepancheg/rust-protobuf/pull/332)

## [2.1.2] - 2018-10-28

- [`cached_size` field is public now](https://github.com/stepancheg/rust-protobuf/issues/348)

## [2.1.1] - 2018-10-09

- [Make `Customize::_future_options` public](https://github.com/stepancheg/rust-protobuf/pull/346)

## [2.1.0] - 2018-10-06

- Support of `protoc` command from Google protobuf before 3.0 is dropped
  (it might work, but not tested by CI); this does not affect `syntax = "proto2"` which is supported
- [When using `protoc` codegen options can now be passed with `--rust_opt`
  flag](https://github.com/stepancheg/rust-protobuf/commit/7ebf32b47cb18160752a943dccb9d0d7ecdf91ed)
- [Serde is now supported](https://github.com/stepancheg/rust-protobuf/issues/266)
- [`unknown_fields` field is public now](https://github.com/stepancheg/rust-protobuf/commit/24e6479e869d61455bfcf50dde102e6278648516)

## [2.0.6] - 2019-01-13

- [Fix serialization of signed int map keys or values](https://github.com/stepancheg/rust-protobuf/pull/372)

## [2.0.5] - 2018-09-21

- [Global `parse_length_delimited*` functions are
  deprecated](https://github.com/stepancheg/rust-protobuf/commit/efdfd5cacfa4f87b2a6e3ffc124d77692db142d9)
- [Fixed a bug with quotes in string literal parsing in pure
  codegen](https://github.com/stepancheg/rust-protobuf/issues/337)

## [2.0.4] - 2018-07-19

- Minimum bytes version is 0.4 now (since protobuf doesn't work with 0.3 anyway)

## [2.0.3] - 2018-07-11

- [Fix panic on singular string field appeared more than
  once](https://github.com/stepancheg/rust-protobuf/commit/28adf07a0b0027ddc8ff57f04ffeb69f35f65620)
- [Properly handle map fields with key or value skipped in binary proto](
  https://github.com/stepancheg/rust-protobuf/issues/318)

## [2.0.2] - 2018-05-29

- Make rust-protobuf compatible with rust 1.23.0

## [2.0.1] - 2018-05-27

- Fix codegen with enum with
  [default value a reserved rust keyword](https://github.com/stepancheg/rust-protobuf/issues/295)

## [2.0.0] - 2018-05-17

- Rebublished branch 1.6 because of
  [backward compatibility issues in 1.6 branch](https://github.com/stepancheg/rust-protobuf/issues/289)

## [1.7.5] - 2018-05-20
- Fix [OOM on malformed input](https://github.com/stepancheg/rust-protobuf/issues/411)

## [1.7.4] - 2018-07-11

- [Fix panic on singular string field appeared more than
  once](https://github.com/stepancheg/rust-protobuf/commit/28adf07a0b0027ddc8ff57f04ffeb69f35f65620)
- [Properly handle map fields with key or value skipped in binary proto](
  https://github.com/stepancheg/rust-protobuf/issues/318)

## [1.7.3] - 2018-05-29

- Make rust-protobuf compatible with rust 1.23.0

## [1.7.2] - 2018-05-27

- Fix codegen with enum with
  [default value a reserved rust keyword](https://github.com/stepancheg/rust-protobuf/issues/295)

## [1.7.1] - 2018-05-17

- Rebublished branch 1.5 because of
  [backward compatibility issues in 1.6 branch](https://github.com/stepancheg/rust-protobuf/issues/289)

## [1.6.0] - 2018-05-11

Republished as 2.0.0

### New features

- Pure rust codegen
- Generated code can now be customized not only with `rustproto.proto`
  but also when invoked programmatically with
  [`protoc-rust`](https://github.com/stepancheg/rust-protobuf/blob/b8573bd53cf5a9611598abbf02b71c49e59a8891/protobuf-codegen/src/customize.rs#L9)
- [Oneof are now public by
  default](https://github.com/stepancheg/rust-protobuf/commit/8bd911e2ea0d4461580105209ae11d9d3ec21fd0)
- [Option to specify recursion limit](https://github.com/stepancheg/rust-protobuf/pull/248)
- [Implement conversions for `Repeated*`](https://github.com/stepancheg/rust-protobuf/pull/236)
- [Proto files with suffixes others than `.proto`
  are now supported](https://github.com/stepancheg/rust-protobuf/pull/265)
- [Generated code now uses closures instead of private functions
  for reflection](https://github.com/stepancheg/rust-protobuf/pull/267)

### Backward compatibility issues

- [Drop `MessageStatic` trait](https://github.com/stepancheg/rust-protobuf/issues/214)
- [Protobuf no longer exposes internal `hex`
  module](https://github.com/stepancheg/rust-protobuf/commit/8ad9687529a565c5ef2db93732cc20c8d8d22f00)
- [`protobuf-codegen` is a separate crate](https://github.com/stepancheg/rust-protobuf/pull/261)
- [Drop old reflection
  accessors](https://github.com/stepancheg/rust-protobuf/commit/7a03aee4e67bdd25ae6c403f37386707a0ab5eb9).
  Now code may need to be regenerated when protobuf version changed.
- [Implement `std::io` traits by `CodedInputStream` and
  `CodedOutputStream`](https://github.com/stepancheg/rust-protobuf/pull/232)
- `*descriptor_static()` function signatures no longer include `Option` param
  ([1](https://github.com/stepancheg/rust-protobuf/commit/8723fca5fb29e279b3ab7d2a28c8fab79189c9c2),
  [2](https://github.com/stepancheg/rust-protobuf/commit/c5446983be3b9d8d49ee39b443ed4fabd8f35440))

## [1.5.1] - 2018-04-02
- [Fix serialization or large repeated packed fields](https://github.com/stepancheg/rust-protobuf/issues/281)

## [1.5.0] - 2018-03-25
- [Unknown enum values are now stored in unknown fields](https://github.com/stepancheg/rust-protobuf/pull/276)

## [1.4.5] - 2018-04-02
- [Fix serialization or large repeated packed fields](https://github.com/stepancheg/rust-protobuf/issues/281)

## [1.4.4] - 2018-03-05
- [Escape macro keyword](https://github.com/stepancheg/rust-protobuf/pull/269)

## [1.4.3] - 2017-12-03
- [Allow enum variants to be named `Self`](https://github.com/stepancheg/rust-protobuf/pull/259)

## [1.4.2] - 2017-10-14
- [Properly read messages from blocking streams](https://github.com/stepancheg/rust-protobuf/issues/157)

## [1.4.1] - 2017-06-24
- [Convert `String` to `Chars`](https://github.com/stepancheg/rust-protobuf/pull/225)

## [1.4] - 2017-06-24
- Start of changelog

## [0.0.0] - 2013-07-28
- First commit added to the repository
