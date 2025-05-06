use crate::interpret::prototype::Prototype;
struct Instance {
    prototype: Prototype,
}
impl Instance {
    fn new(prototype: Prototype) -> Self {
        Self { prototype }
    }
    pub(crate) fn to_string(&self) -> String {
         format!("instance {:?}", self.prototype.struct_name)
    }
}
