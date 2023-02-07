use plotters::prelude::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut x: Vec<f64> = Vec::new();
    for _ in 0..100 {
        x.push(rng.gen_range(-100.0..100.0));
    }
    let mut y: Vec<f64> = Vec::new();
    for _ in 0..100 {
        y.push(rng.gen_range(-100.0..100.0));
    }
    let data: Vec<(f64, f64)> = x.iter().cloned().zip(y.iter().cloned()).collect();

    let root_area = BitMapBackend::new("plot.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("Scatter Plot", ("sans-serif", 40.0))
        .build_cartesian_2d(-100.0..100.0, -100.0..100.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 4.0_f64, BLUE)))
        .unwrap();
    root_area.present().unwrap();
    println!("Plot finished");
}
