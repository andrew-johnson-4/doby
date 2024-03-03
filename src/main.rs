
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
         println!("TODO bench {}", tgt);
      }
   } else if mode=="help" {
      println!("doby");
      println!("doby bench [targets]");
   }
}
