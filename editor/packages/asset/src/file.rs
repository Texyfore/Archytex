macro_rules! encdec {
    ($name:ident) => {
        use super::$name;

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

encdec!(Prop);
encdec!(Gizmo);
