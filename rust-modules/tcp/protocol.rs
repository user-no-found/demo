//!TCP 消息协议模块
//!
//!定义统一的消息类型和协议格式。
//!协议格式：[类型:1字节][长度:8字节][数据:N字节]

//========================================
//消息类型定义
//========================================

///消息类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    ///字符串消息
    String = 1,
    ///原始字节数据
    Bytes = 2,
    ///文件传输
    File = 3,
    ///图片传输
    Image = 4,
    ///视频流
    VideoStream = 5,
}

impl MessageType {
    ///从 u8 转换为 MessageType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::String),
            2 => Some(Self::Bytes),
            3 => Some(Self::File),
            4 => Some(Self::Image),
            5 => Some(Self::VideoStream),
            _ => None,
        }
    }

    ///转换为 u8
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

//========================================
//消息头结构
//========================================

///消息头大小（字节）
pub const HEADER_SIZE: usize = 9;

///消息头结构
#[derive(Debug, Clone)]
pub struct MessageHeader {
    ///消息类型
    pub msg_type: MessageType,
    ///数据长度
    pub data_len: u64,
}

impl MessageHeader {
    ///创建新的消息头
    pub fn new(msg_type: MessageType, data_len: u64) -> Self {
        Self { msg_type, data_len }
    }

    ///序列化为字节数组
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut bytes = [0u8; HEADER_SIZE];
        bytes[0] = self.msg_type.to_u8();
        bytes[1..9].copy_from_slice(&self.data_len.to_be_bytes());
        bytes
    }

    ///从字节数组反序列化
    pub fn from_bytes(bytes: &[u8; HEADER_SIZE]) -> Option<Self> {
        let msg_type = MessageType::from_u8(bytes[0])?;
        let data_len = u64::from_be_bytes(bytes[1..9].try_into().ok()?);
        Some(Self { msg_type, data_len })
    }
}

//========================================
//文件元信息
//========================================

///文件元信息头大小（文件名长度字段）
pub const FILE_META_SIZE: usize = 2;

///文件元信息
#[derive(Debug, Clone)]
pub struct FileMeta {
    ///文件名
    pub filename: String,
}

impl FileMeta {
    ///创建新的文件元信息
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    ///序列化为字节（长度+文件名）
    pub fn to_bytes(&self) -> Vec<u8> {
        let name_bytes = self.filename.as_bytes();
        let len = name_bytes.len() as u16;
        let mut bytes = Vec::with_capacity(FILE_META_SIZE + name_bytes.len());
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.extend_from_slice(name_bytes);
        bytes
    }

    ///从字节反序列化
    pub fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.len() < FILE_META_SIZE {
            return None;
        }
        let len = u16::from_be_bytes(bytes[0..2].try_into().ok()?) as usize;
        if bytes.len() < FILE_META_SIZE + len {
            return None;
        }
        let filename = std::string::String::from_utf8(bytes[2..2 + len].to_vec()).ok()?;
        Some((Self { filename }, FILE_META_SIZE + len))
    }
}

//========================================
//完整消息结构
//========================================

///完整消息
#[derive(Debug, Clone)]
pub struct Message {
    ///消息头
    pub header: MessageHeader,
    ///消息数据
    pub data: Vec<u8>,
}

impl Message {
    ///创建字符串消息
    pub fn string(content: &str) -> Self {
        let data = content.as_bytes().to_vec();
        Self {
            header: MessageHeader::new(MessageType::String, data.len() as u64),
            data,
        }
    }

    ///创建字节消息
    pub fn bytes(data: Vec<u8>) -> Self {
        Self {
            header: MessageHeader::new(MessageType::Bytes, data.len() as u64),
            data,
        }
    }

    ///创建文件消息（包含文件名和内容）
    pub fn file(filename: &str, content: Vec<u8>) -> Self {
        let meta = FileMeta::new(filename);
        let meta_bytes = meta.to_bytes();
        let mut data = Vec::with_capacity(meta_bytes.len() + content.len());
        data.extend_from_slice(&meta_bytes);
        data.extend_from_slice(&content);
        Self {
            header: MessageHeader::new(MessageType::File, data.len() as u64),
            data,
        }
    }

    ///创建图片消息（包含文件名和内容）
    pub fn image(filename: &str, content: Vec<u8>) -> Self {
        let meta = FileMeta::new(filename);
        let meta_bytes = meta.to_bytes();
        let mut data = Vec::with_capacity(meta_bytes.len() + content.len());
        data.extend_from_slice(&meta_bytes);
        data.extend_from_slice(&content);
        Self {
            header: MessageHeader::new(MessageType::Image, data.len() as u64),
            data,
        }
    }

    ///创建视频流消息（单帧数据）
    pub fn video_frame(frame_data: Vec<u8>) -> Self {
        Self {
            header: MessageHeader::new(MessageType::VideoStream, frame_data.len() as u64),
            data: frame_data,
        }
    }

    ///序列化完整消息
    pub fn to_bytes(&self) -> Vec<u8> {
        let header_bytes = self.header.to_bytes();
        let mut bytes = Vec::with_capacity(HEADER_SIZE + self.data.len());
        bytes.extend_from_slice(&header_bytes);
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

//========================================
//解析后的消息内容
//========================================

///解析后的消息内容
#[derive(Debug)]
pub enum ParsedContent {
    ///字符串消息
    String(std::string::String),
    ///原始字节
    Bytes(Vec<u8>),
    ///文件（文件名 + 数据）
    File { filename: std::string::String, data: Vec<u8> },
    ///图片（文件名 + 数据）
    Image { filename: std::string::String, data: Vec<u8> },
    ///视频帧
    VideoFrame(Vec<u8>),
}

///解析接收到的消息内容
pub fn parse_message_content(msg: &Message) -> ParsedContent {
    match msg.header.msg_type {
        MessageType::String => {
            let content = std::string::String::from_utf8_lossy(&msg.data).to_string();
            ParsedContent::String(content)
        }
        MessageType::Bytes => {
            ParsedContent::Bytes(msg.data.clone())
        }
        MessageType::File | MessageType::Image => {
            if let Some((meta, offset)) = FileMeta::from_bytes(&msg.data) {
                let content = msg.data[offset..].to_vec();
                if msg.header.msg_type == MessageType::File {
                    ParsedContent::File { filename: meta.filename, data: content }
                } else {
                    ParsedContent::Image { filename: meta.filename, data: content }
                }
            } else {
                ParsedContent::Bytes(msg.data.clone())
            }
        }
        MessageType::VideoStream => {
            ParsedContent::VideoFrame(msg.data.clone())
        }
    }
}
