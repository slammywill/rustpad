#[allow(dead_code)]
#[repr(u8)]
pub enum DeviceMode {
    Session = 0x0,
    Note = 0x1,
    Custom1 = 0x4,
    Custom2 = 0x5,
    Custom3 = 0x6,
    Custom4 = 0x7,
    DawFaders = 0xD,
    Programmer = 0x7F,
}
