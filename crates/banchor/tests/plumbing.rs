#[test]
fn banchor_routing_key_is_the_cross_binary_entry_point() {
    assert_eq!(bsuite_core::RoutingKey::banchor().stable_name(), "banchor");
}
