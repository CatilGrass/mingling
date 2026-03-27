use crate::RenderResult;

pub trait Renderer {
    type Previous;
    fn render(p: Self::Previous, r: &mut RenderResult);
}
