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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("twoscale.png", (2560, 2048)).into_drawing_area();
    root.fill(&WHITE)?;

    let ax = 800f32;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(20)
        .caption("Rand hex tree", ("sans-serif", 50.0).into_font())
        .build_ranged(0f32..ax, 0f32..ax)?;

    chart.configure_mesh().draw()?;

    let conf = Config {
        width: 14f32,
        height: 14f32,
        offset_x: ax / 2f32,
        offset_y: ax / 2f32,
    };
    let hf = HexField::new(conf);
    let tree = algo::rand_tree::generate(&hf, 1024, (ax / 2f32, ax / 2f32), 0.6f32, 1);

    draw_node(&mut chart, &tree)?;

    Ok(())
}
