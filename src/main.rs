
extern crate glob;
extern crate plotters;
use plotters::prelude::*;

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
    let root = SVGBackend::new("plot.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).expect("Root Fill");

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
        ).expect("ChartBuilder");

    chart
        .configure_mesh()
        .x_desc("n")
        .y_desc("Time (ms)")
        .draw().expect("Chart.draw");

    for (idx, &series) in ["CHN", "USA", "RUS", "JPN", "DEU", "IND", "OWID_WRL"]
        .iter()
        .enumerate()
    {
        let color = Palette99::pick(idx).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                vec![(1,2),(3,4)],
                color.stroke_width(3),
            )).expect("Char.draw_series")
            .label(series)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart.configure_series_labels().border_style(BLACK).draw().expect("Char.configure_series_labels");

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to plot.svg");
}
