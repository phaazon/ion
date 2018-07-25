#[macro_use]
extern crate spectra;

mod app_demo;
mod shadertoy;

use spectra::sys::res::{Store, StoreOpt};
use spectra::sys::time::Time;

use shadertoy::AnimatedParam;

struct D {
}

impl D {
  fn new(store: &mut Store<Context>) -> Self {
    D {}
  }
}

impl app_demo::Demo for D {
  fn render_frame(&mut self, t: Time)  {
  }
}

struct Context {

}

fn main() {
  let mut store: Store<Context> = Store::new(StoreOpt::default().set_root("data")).expect("store creation");
  let demo = D::new(&mut store);

  app_demo::run(demo);
}
