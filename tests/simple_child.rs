use taffy::{geometry::Point, style::Dimension};

#[test]
fn simple_child() {
    let mut taffy = taffy::Taffy::new();
    let node0_0 = taffy
        .new_node(
            taffy::style::Style {
                align_self: taffy::prelude::AlignSelf::Center,
                size: taffy::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[node0_0],
        )
        .unwrap();
    let node1_0 = taffy
        .new_node(
            taffy::style::Style {
                align_self: taffy::prelude::AlignSelf::Center,
                size: taffy::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1_1 = taffy
        .new_node(
            taffy::style::Style {
                align_self: taffy::prelude::AlignSelf::Center,
                size: taffy::geometry::Size { width: Dimension::Points(10f32), height: Dimension::Points(10f32) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: Dimension::Undefined, height: Dimension::Undefined },
                ..Default::default()
            },
            &[node1_0, node1_1],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: Dimension::Percent(100.0), height: Dimension::Percent(100.0) },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy
        .compute_layout(
            node,
            taffy::geometry::Size {
                width: taffy::prelude::Number::Defined(100f32),
                height: taffy::prelude::Number::Defined(100f32),
            },
        )
        .unwrap();
    assert_eq!(taffy.layout(node).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(taffy.layout(node0).unwrap().location, Point { x: 0.0, y: 0.0 });
    assert_eq!(taffy.layout(node1).unwrap().location, Point { x: 10.0, y: 0.0 });
    assert_eq!(taffy.layout(node0_0).unwrap().location, Point { x: 0.0, y: 0.0 });
    // Layout is relative so node1_0 location starts at (0,0) and is not ofset by it's parent location
    assert_eq!(taffy.layout(node1_0).unwrap().location, Point { x: 00.0, y: 0.0 });
    assert_eq!(taffy.layout(node1_1).unwrap().location, Point { x: 10.0, y: 0.0 });
}
