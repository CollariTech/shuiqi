use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub enum ShuiqiEvent {
    Redraw,
    PerformResize(PhysicalSize<u32>),
    Window(winit::event::WindowEvent)
}