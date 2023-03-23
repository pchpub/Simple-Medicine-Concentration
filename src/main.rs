use medicine_concentration::mods::{medicine::spawn::get_time2concentration, types::Config};
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/output.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rdr = std::fs::File::open("config.json")?;
    let config: Config = serde_json::from_reader(rdr)?;

    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..24f32, 0f32..200f32)?;

    chart.configure_mesh().draw()?;
    // let medicine_num = config.len();
    for (index, medicine) in config.medicines.iter().enumerate() {
        // let index = index + 1;
        // let color = RGBColor(
        //     {
        //         if (index as f32) < (medicine_num as f32 / 3.0) {
        //             index as f32 / (medicine_num as f32 / 3.0) * 255.0
        //         } else {
        //             0.0
        //         }
        //     } as u8,
        //     {
        //         if (index as f32) < (medicine_num as f32 / 3.0) {
        //             0.0
        //         } else if (index as f32) < medicine_num as f32 / 3.0 * 2.0 {
        //             (index as f32 - medicine_num as f32 / 3.0) / (medicine_num as f32 / 3.0) * 255.0
        //         } else {
        //             0.0
        //         }
        //     } as u8,
        //     {
        //         if (index as f32) < medicine_num as f32 / 3.0 * 2.0 {
        //             0.0
        //         } else {
        //             (index as f32 - medicine_num as f32 / 3.0 * 2.0) / (medicine_num as f32 / 3.0)
        //                 * 255.0
        //         }
        //     } as u8,
        // );
        let color = Palette99::pick(index).mix(0.9);
        let time2concentration = get_time2concentration(medicine,config.time_step);
        let last_day_centration = time2concentration.get_day_concentration(11.0);
        let time_and_centration = last_day_centration
            .iter()
            .map(|x| (x.0 as f32, x.1 as f32));
        chart
            .draw_series(LineSeries::new(time_and_centration, color.stroke_width(3)))?
            .label(&medicine.name)
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
