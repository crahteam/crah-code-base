use wtux::utils::printer::{pres, ppos, pneg};
use wtux::WTux;
use eframe::egui;
fn main() {
   ppos!("Welcome to the WTux Engine");

   pollster::block_on(WTux::run());

}
