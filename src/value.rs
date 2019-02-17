use addressing_mode::{AddressingMode, Condition, DataSize};
use registers::{ConditionCode, Registers};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Byte(u8),
    Word(u16),
    LongWord(u32),
}

impl Value {
    pub fn from_raw(size: DataSize, value: u32) -> Value {
        match size {
            DataSize::Byte => Value::Byte(value as u8),
            DataSize::Word => Value::Word(value as u16),
            DataSize::LongWord => Value::LongWord(value as u32),
        }
    }

    pub fn eor_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s ^ v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();

                let r = s ^ v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s ^ v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
        }
    }

    pub fn or_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();

                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
        }
    }

    pub fn add_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s as u16 + v as u16;
                let max = 0xff;
                let neg = 0x80;
                if r > 0xff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();
                let r = s as u32 + v as u32;

                if r > 0xffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Word(r as u16), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s as u64 + v as u64;

                if r > 0xffff_ffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::LongWord(r as u32), cc)
            }
        }
    }

    pub fn sub_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s as i16 - v as i16;
                let max = 0xff;
                let neg = 0x80;
                if r > 0xff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();
                let r = s as i32 - v as i32;

                if r > 0xffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Word(r as u16), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s as i64 - v as i64;

                if r > 0xffff_ffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::LongWord(r as u32), cc)
            }
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Value::Byte(v) => v as i8 as i32,
            Value::Word(v) => v as i16 as i32,
            Value::LongWord(v) => v as i32,
        }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::Byte(v) => v as u32,
            Value::Word(v) => v as u32,
            Value::LongWord(v) => v as u32,
        }
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::Byte(v) => v as u8,
            Value::Word(v) => v as u8,
            Value::LongWord(v) => v as u8,
        }
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::Byte(v) => v as u16,
            Value::Word(v) => v as u16,
            Value::LongWord(v) => v as u16,
        }
    }
}

impl Into<Value> for Option<u32> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::LongWord(val),
            _ => panic!("Not valid"),
        }
    }
}

impl Into<Value> for Option<u16> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::Word(val),
            _ => panic!("Not valid"),
        }
    }
}

impl Into<Value> for Option<u8> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::Byte(val),
            _ => panic!("Not valid"),
        }
    }
}
