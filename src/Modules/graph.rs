use plotters::prelude::*;
pub fn graph_template() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area for the plot
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();

    // Fill the background with white
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Sample Plot Title", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, -1.5..1.5)?;

    // Configure mesh (grid)
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("X-axis Label")
        .y_desc("Y-axis Label")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    // Generate sample data for sine wave
    let data: Vec<(f64, f64)> = (0..100)
        .map(|x| {
            let x = x as f64 / 10.0;
            (x, x.sin())
        })
        .collect();

    // Draw the line seri
    // // Draw the line series
    chart
        .draw_series(LineSeries::new(data, &BLUE))?
        .label("sin(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Optional: Add text annotation
    /*
    chart.draw_series(std::iter::once(Text::new(
        "Important point",
        (5.0, 0.0),
        ("sans-serif", 15).into_font(),
    )))?;
    */

    // To avoid the svg file being truncated
    root.present()?;

    println!("Plot has been saved to plot.png");

    Ok(())
}
