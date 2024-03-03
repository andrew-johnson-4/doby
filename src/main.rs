
extern crate glob;
extern crate plotters;
use plotters::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct BenchmarkConfig {
  label: String,
  basename: String,
  argchain: Vec<String>,
  cmdchain: Vec<String>,
}
impl BenchmarkConfig {
   pub fn new(tgt: String) -> BenchmarkConfig {
      BenchmarkConfig {
         label: "".to_string(),
         basename: tgt,
         argchain: Vec::new(),
         cmdchain: Vec::new(),
      }
   }
}

fn push_vec<S>(vec: Vec<S>, val: S) -> Vec<S> where S: Clone {
   let mut vec = vec.clone();
   vec.push(val);
   vec
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn bench_mark(cfg: BenchmarkConfig) -> (String,Vec<(u128,u128)>) {
   let mut name = "sh".to_string();
   let mut ms = Vec::new();
   for arg in cfg.argchain.clone().iter() {
   let mut cmd_i = 0;
   for cmd in cfg.cmdchain.clone().iter() {
      cmd_i += 1;
      let cmd = cmd.replace("$basename", &cfg.basename);
      let cmds = cmd.split(" ").collect::<Vec<&str>>();
      let mut prc = Command::new(cmds[0]);
      for parg in cmds.iter().skip(1) {
         prc.arg(parg);
      }
      if cmd_i == cfg.cmdchain.len() {
         prc.arg(arg);
      }
      let before = get_epoch_ms();
      let output = prc.spawn().expect("Failed to execute command")
                      .wait().expect("Failed to wait for command");
      let after = get_epoch_ms();
      if cmd_i == cfg.cmdchain.len() {
         ms.push((
            u128::from_str_radix(&arg, 10).expect("parse benchmark arg"),
            after - before
         ));
      }
      if !output.success() {
         println!("benchmark failed: {}", cmd);
      }
      name = cmd.to_string();
   }}
   if cfg.label.len()>0 {
      name = cfg.label.clone();
   }
   (name, ms)
}

fn bench_file(tgt: &str) {
   let file = File::open(tgt).expect("Open file");
   let basename = tgt.strip_suffix(".bench").expect("strip suffix .bench");
   let reader = BufReader::new(file);
   let mut config = None;
   let mut results = Vec::new();
   for line in reader.lines() {
      let line = line.expect("Line in File").trim().to_string();
      if line.len()==0 && config.is_some() {
         results.push( bench_mark(config.clone().unwrap()) );
         config = None;
      } else if line.len() > 0 {
         if config.is_none() {
            config = Some(BenchmarkConfig::new(basename.to_string()));
         }
         if let Some(bcfg) = config.clone() {
            if line.starts_with("label: ") {
               config = Some(BenchmarkConfig {
                  label: line.strip_prefix("label: ").unwrap().to_string(),
                  ..bcfg.clone()
               })
            }
            if line.starts_with("run: ") {
               config = Some(BenchmarkConfig {
                  cmdchain: push_vec(bcfg.cmdchain.clone(), line.strip_prefix("run: ").unwrap().to_string()),
                  ..bcfg.clone()
               })
            }
            if line.starts_with("arg: ") {
               config = Some(BenchmarkConfig {
                  argchain: push_vec(bcfg.argchain.clone(), line.strip_prefix("arg: ").unwrap().to_string()),
                  ..bcfg.clone()
               })
            }
         }
      }
   }
   if config.is_some() {
      results.push( bench_mark(config.clone().unwrap()) );
   }
   plot(tgt.strip_suffix(".bench").expect("strip_suffix .bench"), results);
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

fn plot(base_name: &str, results: Vec<(String,Vec<(u128,u128)>)>) {
    let file_name = format!("{}.svg", base_name);
    let root = SVGBackend::new(&file_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).expect("Root Fill");

    let (upper, _lower) = root.split_vertically(750);

    let mut chart = ChartBuilder::on(&upper)
        .caption(&format!("{} runtime", base_name), ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (0u32..50u32)
                .with_key_points(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]),
            (0u32..50_0000u32)
                .log_scale()
                .with_key_points(vec![10, 50, 100, 1000, 10000, 100000, 200000]),
        ).expect("ChartBuilder");

    chart
        .configure_mesh()
        .x_desc("n")
        .y_desc("Time (ms)")
        .draw().expect("Chart.draw");

    for (idx,(cmd,cmdts)) in results.iter().enumerate() {
       let points = cmdts.iter().map(|(x,y)| (*x as u32, *y as u32) ).collect::<Vec<(u32,u32)>>();
       let color = Palette99::pick(idx).mix(0.9);
       chart
            .draw_series(LineSeries::new(
                points,
                color.stroke_width(3),
            )).expect("Char.draw_series")
            .label(cmd)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    chart.configure_series_labels().border_style(BLACK).draw().expect("Char.configure_series_labels");

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}.svg", base_name);
}
