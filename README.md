# concord-telemetry-proto

The `concord_telemetry.v1` wire types and the pieces of concord-telemetry that
must stay free of OpenTelemetry.

- `v1::{Principal, PrincipalKind, Verification}`: hand-written prost types, with serde
  (JSON names match pbjson output).
- `v1::TraceCarrier`: the prost message; `From` impls convert to and from the plain
  `TraceCarrier` at the crate root, and `v1::field` yields the optional message field
  (absent when the carrier is empty).
- `v1::Cause`: what caused a record — a registered cause kind and the id in its
  owner's own format.
- `proto/concord_telemetry/v1/{principal,trace_carrier}.proto` and `proto_include_dir()`,
  for a consumer's `build.rs`.
- `TRACEPARENT_ENV`, `BAGGAGE_ENV`, `inject_env`.

## Importing the protos

Add this crate under `[build-dependencies]`, pass `proto_include_dir()` as a protoc
include path, and point both prost and pbjson at the hand-written types:

```rust
prost_build::Config::new()
    .extern_path(concord_telemetry_proto::PROTO_PACKAGE_PATH,
                 concord_telemetry_proto::RUST_EXTERN_PATH)
```

Add the crate as a normal dependency too, since generated code names
`::concord_telemetry_proto::v1::*`. `proto-consumer-test` is a working example.

`concord-telemetry` re-exports these types; depend on this crate directly from proto and
type crates that must not pull the OpenTelemetry stack.

## Check

`scripts/check.sh` runs fmt, clippy (`-D warnings`) and the tests.
