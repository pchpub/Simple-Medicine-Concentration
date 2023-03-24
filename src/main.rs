use medicine_concentration::mods::{medicine::spawn::get_time2concentration, types::Config};
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/output.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rdr = std::fs::File::open("config.json")?;
    let config: Config = serde_json::from_reader(rdr)?;

    let mut last_day_centrations = Vec::new();
    let mut y_spec_max: f32 = 0.0;
    for medicine in config.medicines.iter() {
        let time2concentration = get_time2concentration(medicine,config.time_step);
        let last_day_centration = time2concentration.get_day_concentration(11.0).iter().map(|x|(x.0 as f32,x.1 as f32)).collect::<Vec<(f32,f32)>>();
        let last_day_centration_max = last_day_centration
            .iter()
            .map(|x| x.1)
            .fold(0.0_f32, |a, b| a.max(b));
        if last_day_centration_max > y_spec_max {
            y_spec_max = last_day_centration_max;
        }
        last_day_centrations.push((&medicine.name,last_day_centration));
    }
    let y_spec_max = y_spec_max * 1.1;

    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..24f32, 0f32..y_spec_max)?;

    chart.configure_mesh().draw()?;
    for (index, last_day_centration) in last_day_centrations.iter().enumerate() {
        let color = Palette99::pick(index).mix(0.9);
        chart
            .draw_series(LineSeries::new(last_day_centration.1.clone().into_iter(), color.stroke_width(3)))?
            .label(last_day_centration.0)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(3))
            });
    }
    chart
        .configure_series_labels()
        .label_font(("sans-serif", 30).into_font())
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
