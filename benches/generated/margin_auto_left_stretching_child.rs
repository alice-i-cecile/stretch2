pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0f32),
                margin: taffy::geometry::Rect { start: taffy::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
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
                align_items: taffy::style::AlignItems::Center,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
