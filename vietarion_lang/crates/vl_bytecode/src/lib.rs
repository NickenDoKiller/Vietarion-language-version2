use serde::{Serialize, Deserialize};

/// OpCode: Tập lệnh của máy ảo Vietarion
/// Đây là những gì máy tính sẽ thực sự hiểu sau khi compile
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0x00,        // Dừng chương trình
    LoadConst = 0x01,   // Nạp hằng số vào stack (index trong pool)
    LoadVar = 0x02,     // Nạp biến
    StoreVar = 0x03,    // Lưu biến
    
    // Toán học
    Add = 0x10,
    Sub = 0x11,
    Mul = 0x12,
    Div = 0x13,
    
    // So sánh & Nhảy
    Eq = 0x20,
    Gt = 0x21,
    Lt = 0x22,
    Jmp = 0x30,         // Nhảy không điều kiện
    JmpIfFalse = 0x31,  // Nhảy nếu sai (dùng cho 'neu')

    // System
    Print = 0xF0,       // In ra màn hình
}

/// Cấu trúc file .vlbc
#[derive(Debug, Serialize, Deserialize)]
pub struct BytecodeChunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>, // Pool hằng số
    // Debug info: map từ instruction index -> line:col
    pub lines: Vec<(u32, u32)>, 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Nguyen(i64),
    Thuc(f64),
    Logic(bool),
    Chuoi(String),
}
