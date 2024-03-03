
extern crate glob;

fn bench_file(tgt: &str) {
   println!("bench {}", tgt);
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
