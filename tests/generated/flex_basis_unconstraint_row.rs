#[test]
fn flex_basis_unconstraint_row() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_basis: taffy::style::Dimension::Points(50f32),
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
}
