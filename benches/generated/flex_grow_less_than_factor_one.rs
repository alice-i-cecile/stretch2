pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_grow: 0.2f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_node(taffy::style::Style { flex_grow: 0.2f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node2 = taffy
        .new_node(taffy::style::Style { flex_grow: 0.4f32, flex_shrink: 0f32, ..Default::default() }, &[])
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
