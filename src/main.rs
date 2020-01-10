use hex_field::*;
use plotters::prelude::*;

trait ToLine {
    fn to_line(&self) -> Vec<(f32, f32)>;
    fn link_to(&self, other: &Self) -> Vec<(f32, f32)>;
}

impl ToLine for Hex {
    fn to_line(&self) -> Vec<(f32, f32)> {
        vec![
            self.top_left(),
            self.top_right(),
            self.right(),
            self.bot_right(),
            self.bot_left(),
            self.left(),
            self.top_left(),
        ]
    }

    fn link_to(&self, other: &Self) -> Vec<(f32, f32)> {
        vec![self.center(), other.center()]
    }
}

fn draw_node(
    chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
    node: &algo::rand_tree::Node<Hex>,
) -> Result<(), Box<dyn std::error::Error>> {
    chart.draw_series(LineSeries::new(node.data.to_line(), &BLUE))?;
    for n in node.iter() {
        chart.draw_series(LineSeries::new(node.data.link_to(&n.data), &RED))?;
        draw_node(chart, n)?;
    }

    Ok(())
}

fn draw_line(
    chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
    p1: (f32, f32),
    p2: (f32, f32),
) -> Result<(), Box<dyn std::error::Error>> {
    chart.draw_series(LineSeries::new(vec![p1, p2], &BLUE))?;
    Ok(())
}

fn draw_lines_to_vertices(
    chart: &mut ChartContext<BitMapBackend, RangedCoord<RangedCoordf32, RangedCoordf32>>,
    p: (f32, f32),
    hex: &Hex,
) -> Result<(), Box<dyn std::error::Error>> {
    for v in &hex.points() {
        draw_line(chart, *v, p)?;
    }
    Ok(())
}

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

    let conf = Config {
        width: 400f32,
        height: 200f32,
        offset_x: ax / 2f32,
        offset_y: ax / 2f32,
    };
    let hf = HexField::new(conf);
    let tree = algo::rand_tree::generate(&hf, 5, (ax / 2f32, ax / 2f32), 0.6f32, 1);

    draw_node(&mut chart, &tree)?;

    let c = tree.root().data.clone();
    let (t, tr) = (c.top_hex(), c.top_right_hex());
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
