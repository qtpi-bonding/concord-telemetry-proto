use concord_telemetry_proto::v1;
use concord_telemetry_proto::TraceCarrier;
use prost::Message;

#[test]
fn proto_carrier_converts_both_ways() {
    let plain = TraceCarrier {
        traceparent: Some("00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".into()),
        tracestate: None,
        baggage: Some("a=b".into()),
    };
    let wire: v1::TraceCarrier = (&plain).into();
    let decoded = v1::TraceCarrier::decode(wire.encode_to_vec().as_slice()).expect("decode");
    assert_eq!(TraceCarrier::from(&decoded), plain);
}

#[test]
fn field_is_absent_for_an_empty_carrier() {
    assert_eq!(v1::field(&TraceCarrier::default()), None);
    let plain = TraceCarrier {
        baggage: Some("a=b".into()),
        ..Default::default()
    };
    let wire = v1::field(&plain).expect("set carrier");
    assert_eq!(wire.baggage.as_deref(), Some("a=b"));
    assert_eq!(TraceCarrier::from(&wire), plain);
}
