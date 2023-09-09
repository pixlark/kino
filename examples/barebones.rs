use kino::prelude::{Canvas, Context, Vec2, Vec4};
use std::rc::Rc;

fn main() {
    let context = Context::new();
    let mut canvas = Canvas::new(Rc::clone(&context));

    while !context.borrow().exit_requested() {
        context.borrow_mut().pump_events();

        canvas.clear();
        canvas.rect_fill(
            Vec2::new(50.0, 50.0),
            Vec2::new(50.0, 50.0),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
        );

        context.borrow_mut().finish_frame();
    }
}
