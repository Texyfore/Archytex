macro_rules! encdec {
    ($name:path) => {
        impl $name {
            pub fn encode(&self) -> Option<Vec<u8>> {
                bincode::serialize(&self).ok()
            }

            pub fn decode(buf: &[u8]) -> Option<Self> {
                bincode::deserialize::<Self>(buf).ok()
            }
        }
    };
}

encdec!(crate::Prop);
encdec!(crate::Gizmo);
encdec!(crate::scene::Scene);
