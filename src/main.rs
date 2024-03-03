
extern crate glob;
extern crate plotters;
use plotters::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
struct BenchmarkConfig {
  basename: String,
  cmdchain: Vec<String>,
}
impl BenchmarkConfig {
   pub fn new(tgt: String) -> BenchmarkConfig {
      BenchmarkConfig {
         basename: tgt,
         cmdchain: Vec::new()
      }
   }
}

fn push_vec<S>(vec: Vec<S>, val: S) -> Vec<S> where S: Clone {
   let mut vec = vec.clone();
   vec.push(val);
   vec
}

fn bench_mark(cfg: BenchmarkConfig) {
   for cmd in cfg.cmdchain {
      let cmd = cmd.replace("$basename", &cfg.basename);
      println!("TODO run benchmark {}", cmd);
   }
}

fn bench_file(tgt: &str) {
   let file = File::open(tgt).expect("Open file");
   let basename = tgt.strip_suffix(".bench").expect("strip suffix .bench");
   let reader = BufReader::new(file);
   let mut config = None;
   for line in reader.lines() {
      let line = line.expect("Line in File").trim().to_string();
      if line.len()==0 && config.is_some() {
         bench_mark(config.clone().unwrap());
         config = None;
      } else if line.len() > 0 {
         if config.is_none() {
            config = Some(BenchmarkConfig::new(basename.to_string()));
         }
         if let Some(bcfg) = config.clone() {
            if line.starts_with("run: ") {
               config = Some(BenchmarkConfig {
                  cmdchain: push_vec(bcfg.cmdchain, line.strip_prefix("run: ").unwrap().to_string()),
                  ..bcfg
               })
            }
         }
      }
   }
   if config.is_some() {
      bench_mark(config.clone().unwrap())
   }
   plot(tgt.strip_suffix(".bench").expect("strip_suffix .bench"));
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

fn plot(base_name: &str) {
    let file_name = format!("{}.svg", base_name);
    let root = SVGBackend::new(&file_name, (1024, 768)).into_drawing_area();
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
    println!("Result has been saved to {}.svg", base_name);
}
