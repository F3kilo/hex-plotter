use glam::Vec2;
use hex_field::algo::rand_tree::{RandHexTree, MappedTree};
use hex_field::*;
use plotters::prelude::*;
use rand::prelude::*;

trait ToLine {
    fn to_line(&self) -> Vec<(f32, f32)>;
    fn link_to(&self, other: &Self) -> Vec<(f32, f32)>;
}

impl ToLine for Hex {
    fn to_line(&self) -> Vec<(f32, f32)> {
        let mut v: Vec<(f32, f32)> = self.vertices().map(|v| v.into()).collect();
        v.push(self.left_top_vert().into());
        v
    }

    fn link_to(&self, other: &Self) -> Vec<(f32, f32)> {
        vec![self.center().into(), other.center().into()]
    }
}

fn draw_node(
    chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
    tree: &MappedTree<Hex>,
    node: &Hex,
) -> Result<(), Box<dyn std::error::Error>> {
    chart.draw_series(LineSeries::new(node.to_line(), &BLUE))?;
    let children = tree.children(node);
    for child in children {
        draw_node(chart, tree, child)?;
        chart.draw_series(LineSeries::new(node.link_to(child), &RED))?;
    }

    Ok(())
}

// fn draw_line(
//     chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
//     p1: (f32, f32),
//     p2: (f32, f32),
// ) -> Result<(), Box<dyn std::error::Error>> {
//     chart.draw_series(LineSeries::new(vec![p1, p2], &BLUE))?;
//     Ok(())
// }
//
// fn draw_lines_to_vertices(
//     chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
//     p: (f32, f32),
//     hex: &Hex,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     for v in &hex.points() {
//         draw_line(chart, *v, p)?;
//     }
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("twoscale.png", (1920, 1440)).into_drawing_area();
    root.fill(&WHITE)?;

    let ax = 1000f32;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(20)
        .caption("Rand hex tree", ("sans-serif", 50.0).into_font())
        .build_ranged(0f32..ax, 0f32..ax)?;

    chart.configure_mesh().draw()?;


    let center = Vec2::new(ax / 2f32, ax / 2f32);
    let conf = Config {
        hex_size: Vec2::new(320f32, 160f32),
        offset: center,
    };
    let hf = HexField::new(conf);
    let hex = Hex::new(
        hf.hex_center_by_containing_point(center),
        hf.hex_size(),
    );

    let mut tree = algo::rand_tree::RandHexTree::new(hex, 0.5f64, StdRng::seed_from_u64(0));
    tree.add_hexes(5);
    println!("Calculated!!!");
    draw_node(&mut chart, &tree.tree(), &tree.tree().root().unwrap())?;

    // let c = tree.root().data.clone();
    // let (t, tr) = (c.top_hex(), c.top_right_hex());
    // chart.draw_series(LineSeries::new(
    //     (0..=200).map(|t| {
    //         let angle = (t as f32) / 200f32 * 2f32 * std::f32::consts::PI;
    //         let (x, y) = c.center();
    //         (x + angle.cos() * c.size().0 / 16f32 * 7f32, y + angle.sin() * c.size().1 / 2f32)
    //     }),
    //     &BLACK,
    // ))?;

    Ok(())
}
