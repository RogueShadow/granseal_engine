
use granseal_engine::run;

fn main() {
    pollster::block_on(run("WGPU Practice.", 1920, 1080));
}