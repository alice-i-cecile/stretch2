pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_grow: 1f32,
                margin: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(10f32),
                    end: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
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
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
