use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub enum ShuiqiEvent {
    PerformResize(PhysicalSize<u32>),
    Window(winit::event::WindowEvent)
}