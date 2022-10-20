#[test]
fn test_single_and_gate() {
    use crate::logic::{AndGate, Lamp, Switch};

    // 1 x 0
    let switch_a = Box::new(Switch::create(true));
    let switch_b = Box::new(Switch::create(false));
    let and_gate = Box::new(AndGate::create_from(switch_a, switch_b));
    let lamp = Lamp::create(and_gate);
    assert_eq!(lamp.is_glowing(), false);

    // 1 x 1
    let switch_a = Box::new(Switch::create(true));
    let switch_b = Box::new(Switch::create(true));
    let and_gate = Box::new(AndGate::create_from(switch_a, switch_b));
    let lamp = Lamp::create(and_gate);
    assert_eq!(lamp.is_glowing(), true);

    // 0 x 0
    let switch_a = Box::new(Switch::create(false));
    let switch_b = Box::new(Switch::create(false));
    let and_gate = Box::new(AndGate::create_from(switch_a, switch_b));
    let lamp = Lamp::create(and_gate);
    assert_eq!(lamp.is_glowing(), false);
}
