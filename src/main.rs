
extern crate glob;
extern crate plotters;
use plotters::prelude::*;
use std::fs::File;
use std::io::BufReader;


fn bench_file(tgt: &str) {
   println!("bench {}", tgt);
   plot();
}

fn main() {
   let mut mode = "help";
   let mut targets = Vec::new();
   for arg in std::env::args().skip(1) {
      if arg=="bench" {
         mode = "bench";
      } else {
         targets.push(arg);
      }
   }
   if mode=="bench" {
      for tgt in targets {
         for bench in glob::glob(&format!("{}/*.bench",tgt)).expect("glob failed") {
            bench_file(&bench.expect("glob failed").display().to_string());
         }
         for bench in glob::glob(&format!("{}/*/*.bench",tgt)).expect("glob failed") {
            bench_file(&bench.expect("glob failed").display().to_string());
         }
         for bench in glob::glob(&format!("{}/*/*/*.bench",tgt)).expect("glob failed") {
            bench_file(&bench.expect("glob failed").display().to_string());
         }
      }
   } else if mode=="help" {
      println!("doby");
      println!("doby bench [targets]");
   }
}

fn plot() {
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let OUT_FILE_NAME: &str = "plot.svg";
    let root = SVGBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(750);

    let mut chart = ChartBuilder::on(&upper)
        .caption("Runtime", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (20u32..5000_0000u32)
                .log_scale()
                .with_key_points(vec![50, 100, 1000, 10000, 100000, 1000000, 10000000]),
            (0u32..50_0000u32)
                .log_scale()
                .with_key_points(vec![10, 50, 100, 1000, 10000, 100000, 200000]),
        )?;

    chart
        .configure_mesh()
        .x_desc("Time (ms)")
        .y_desc("n")
        .draw()?;

    for (idx, &series) in ["CHN", "USA", "RUS", "JPN", "DEU", "IND", "OWID_WRL"]
        .iter()
        .enumerate()
    {
        let color = Palette99::pick(idx).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                data[series].data.iter().map(
                    |&DailyData {
                         new_cases,
                         total_cases,
                         ..
                     }| (total_cases as u32, new_cases as u32),
                ),
                color.stroke_width(3),
            ))?
            .label(series)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
