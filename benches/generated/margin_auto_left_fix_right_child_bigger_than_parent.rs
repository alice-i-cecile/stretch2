pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(72f32),
                    height: taffy::style::Dimension::Points(72f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Auto,
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
                justify_content: taffy::style::JustifyContent::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(52f32),
                    height: taffy::style::Dimension::Points(52f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
