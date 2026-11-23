use concord_telemetry_proto::v1::Cause;
use prost::Message;

#[test]
fn cause_round_trips_through_prost_and_json() {
    let c = Cause::new("action", "A1");
    assert_eq!(Cause::decode(&c.encode_to_vec()[..]).unwrap(), c);
    assert_eq!(
        serde_json::to_value(Cause::new("action", "A1")).unwrap(),
        serde_json::json!({"kind": "action", "id": "A1"})
    );
    assert_eq!(
        serde_json::from_value::<Cause>(serde_json::json!({"kind": "action", "id": "A1"})).unwrap(),
        c
    );
    assert_eq!(
        serde_json::from_value::<Cause>(serde_json::json!({})).unwrap(),
        Cause::default()
    );
}
