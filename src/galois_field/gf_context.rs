use super::Element;
use super::gf_elm::GFElm;

/// implemantation for Field Generator
/// usage :
/// ```
/// let gf5 = FieldGenerator::new(5);
/// println!("{:?}",gf5);
/// println!("{:?}",gf5.elm(3));
/// ```
/// 
#[derive(Debug)]
pub struct GFContext{
    modulo: Element
}

impl GFContext{
    pub fn new<T>(x:T) -> Self
    where Element: From<T>,
    {
        let modulo = Element::from(x);
        Self {modulo: modulo}
    }
    pub fn elm<T>(&self,x:T) -> GFElm
    where Element: From<T>,
    {
        GFElm::new(Element::from(x), self.modulo) 
    }
}