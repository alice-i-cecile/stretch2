pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(200f32), ..Default::default() },
                max_size: taffy::geometry::Size {
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
