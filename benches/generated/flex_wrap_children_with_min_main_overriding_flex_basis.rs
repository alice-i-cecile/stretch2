pub fn compute() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
