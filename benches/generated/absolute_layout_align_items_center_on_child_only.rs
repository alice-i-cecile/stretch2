pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                align_self: taffy::style::AlignSelf::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(60f32),
                    height: taffy::style::Dimension::Points(40f32),
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
                    width: taffy::style::Dimension::Points(110f32),
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
