
pub mod can_message_ids {

    pub type MessageID = u32;
    pub const SPEED_ID: MessageID = 0x280;
    pub const G_LAT_ID: MessageID = 0x292;
    pub const G_LONG_ID: MessageID = 0x292;
    pub const YAW_ID: MessageID = 0x292;
    pub const STEERING_ID: MessageID = 0x002;
    pub const BRAKE_ID: MessageID = 0x182;
}