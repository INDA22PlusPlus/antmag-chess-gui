


extern crate piston_window;
use piston_window::*;
use INDA22PlusPlus_antmag_hw2::*;

fn main() {
    println!("Hello, world!");

    let mut window : PistonWindow = WindowSettings::new("My_Window", [640, 480])
    .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next(){
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], context.transform, graphics);
        });
    }

}
