#[test]
fn border_flex_child() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                border: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(10f32),
                    end: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 80f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 10f32);
}
