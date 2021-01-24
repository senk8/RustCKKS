use super::gf_elm::GFElm;
use super::Element;

/// implemantation for Field Generator
/// usage :
/// ```
/// let gf5 = FieldGenerator::new(5);
/// println!("{:?}",gf5);
/// println!("{:?}",gf5.elm(3));
/// ```
///
#[derive(Debug)]
pub struct GFContext {
    modulo: Element,
}

impl GFContext {
    pub fn new(x: u64) -> Self
    {
        Self { modulo: x }
    }
    pub fn elm(&self, x: u64) -> GFElm
    {
        GFElm::new(x, self.modulo)
    }
}
