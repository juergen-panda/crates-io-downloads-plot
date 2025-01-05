use plotters::prelude::*;
use postgres::NoTls;
use core::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = postgres::Client::connect(
        "host=localhost user=user password=password dbname=db_dump",
        NoTls,
    )?;

    let data: Vec<(i64,i64)> = client
        .query(
            "select cast(cd.downloads as varchar(1)) as leading_digit, count(*) as count
             from crate_downloads cd
             group by leading_digit
             order by leading_digit",
            &[],
        )?
        .iter()
        .map(|row| {
            let leading_digit: String = row.get("leading_digit");
            (leading_digit.parse::<i64>().unwrap(), row.get("count"))
        })
        .collect();

    println!("Total crates investigated {}.\n", data.iter().map(|(_,y)| *y).sum::<i64>());

    if let Some((_,y)) = data.iter().find(|(x,_)| *x == 0) {
        println!("Skipping {y} crates with 0 downloads in further analysis.");
    }

    let total_count = data.iter().filter(|(x,_)| *x != 0).map(|(_, y)| y).sum::<i64>();
    println!("Leading Digit | Observed frequency | Expected frequency");
    for (leading_digit, count) in data.iter().filter(|(x,_)| *x != 0) {
        println!(
            "{: <13} | {: <18.4} | {:.4}",
            leading_digit,
            (*count as f64) / (total_count as f64),
            (1.0 + 1.0 / (*leading_digit as f64)).log10()
        );
    }
    println!();
    
    let output_file = "plot.png";

    let root_area = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let max_y = data.iter().map(|(_, y)| *y).max().unwrap() as f64;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Count of Leading Digits", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (1..9i64).into_segmented(),
            0..((data.iter().map(|(_, count)| *count).max().unwrap_or(0) as f64 * 1.1) as i64),
        )?
        .set_secondary_coord(1.0..10.0, 0.0..max_y);

    chart.configure_mesh().draw()?;

    let observed_data = data.iter().map(|(x, y)| {
        Rectangle::new(
            [(*x as f64 + 0.1, *y as f64), (*x as f64 + 0.48, 0.0f64)],
            Into::<ShapeStyle>::into(RED).filled(),
        )
    });

    chart
        .draw_secondary_series(observed_data)?
        .label("Counts of leading digits as found in data")
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 20, y + 10)], RED.filled()));

    let expected_data = (1u32..10).map(|x| {
        Rectangle::new(
            [
                (
                    x as f64 + 0.52,
                    (1.0 + 1.0 / (x as f64)).log10() * total_count as f64,
                ),
                (x as f64 + 0.9, 0.0f64),
            ],
            Into::<ShapeStyle>::into(BLUE).filled(),
        )
    });

    chart
        .draw_secondary_series(expected_data)?
        .label("Counts of leading digits as expected by Benford's law")
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 20, y + 10)], BLUE.filled()));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    println!("Saved plot to file {output_file}");

    Ok(())
}
