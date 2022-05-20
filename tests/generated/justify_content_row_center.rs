#[test]
fn justify_content_row_center() {
    let mut stretch = sprawl::Stretch::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
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
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
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
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 35f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 10f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 45f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 10f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 55f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 0f32);
}
