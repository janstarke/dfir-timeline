use flow_record_common::{Error, FieldType, ToMsgPackValue};
use strum::Display;


/// Values for the mode field -- which identifies the file type and permissions.
#[derive(Debug, Display)]
pub enum FileType {
    Unknown,
    Regular,
    Directory,
    NamedPipe,
    CharacterDevice,
    BlockDevice,
    SymbolicLink,
    Shadow,
    Socket,
    Whiteout,

    /// "Virtual File" created by TSK for file system areas
    VirtualFile,

    /// "Virtual Directory" created by TSK to hold data like orphan files
    VirtualDirectory,
}

impl TryFrom<&str> for FileType {
    type Error = flow_record_common::Error;

    /// <https://github.com/sleuthkit/sleuthkit/blob/develop/tsk/fs/fs_inode.c#L25>
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let type_s = if value.len() == 9 {
            return Ok(Self::Unknown)
        } else if value.len() == 10 {
            value.chars().next().unwrap()
        } else if value.len() == 12 {
            let mut iter = value.chars();
            let s1 = iter.next().unwrap();
            let s2 = iter.next().unwrap();
            let s3 = iter.next().unwrap();

            if s1 != s3 {
                return Err(Error::InvalidModeString(value.into()));
            }
            if s2 != '/' {
                return Err(Error::InvalidModeString(value.into()));
            }
            s1
        } else {
            return Err(Error::InvalidModeString(value.into()));            
        };

        match type_s {
            'r' => Ok(Self::Regular),
            'd' => Ok(Self::Directory),
            'p' => Ok(Self::NamedPipe),
            'c' => Ok(Self::CharacterDevice),
            'b' => Ok(Self::BlockDevice),
            'l' => Ok(Self::SymbolicLink),
            's' => Ok(Self::Shadow),
            'h' => Ok(Self::Socket),
            'w' => Ok(Self::Whiteout),
            'v' => Ok(Self::VirtualFile),
            'V' => Ok(Self::VirtualDirectory),
            _ => Err(Error::InvalidModeString(value.into()))
        }
    }
}

impl ToMsgPackValue for FileType {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.to_string().into())
    }

    fn field_type() -> flow_record_common::FieldType {
        FieldType::String
    }
}