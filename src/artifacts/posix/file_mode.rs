use bitflags::bitflags;
use flow_record_common::{FieldType, ToMsgPackValue};
use lazy_regex::regex_captures;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct FileMode: u16 {
        const UNSPECIFIED = 0o000000;

        const ISUID = 0o004000;
        const ISGID = 0o002000;
        const ISVTX = 0o001000;

        const IRUSR = 0o000400;
        const IWUSR = 0o000200;
        const IXUSR = 0o000100;

        const IRGRP = 0o000040;
        const IWGRP = 0o000020;
        const IXGRP = 0o000010;

        const IROTH = 0o000004;
        const IWOTH = 0o000002;
        const IXOTH = 0o000001;
    }
}

impl TryFrom<&str> for FileMode {
    type Error = flow_record_common::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((_, rusr, wusr, xusr, rgrp, wgrp, xgrp, roth, woth, xoth)) = regex_captures!(
            r#"([-r])([-w])([-xsS])([-r])([-w])([-xsS])([-r])([-w])([-xtT])"#,
            value
        ) {
            let mut mode = Self::UNSPECIFIED;
            if rusr == "r" {
                mode |= Self::IRUSR
            }
            if wusr == "w" {
                mode |= Self::IWUSR
            }

            match xusr {
                "x" => mode |= Self::IXUSR,
                "s" => mode |= Self::IXUSR | Self::ISUID,
                "S" => mode |= Self::ISUID,
                _ => (),
            }

            if rgrp == "r" {
                mode |= Self::IRGRP
            }
            if wgrp == "w" {
                mode |= Self::IWGRP
            }

            match xgrp {
                "x" => mode |= Self::IXGRP,
                "s" => mode |= Self::IXGRP | Self::ISGID,
                "S" => mode |= Self::ISGID,
                _ => (),
            }

            if roth == "r" {
                mode |= Self::IROTH
            }
            if woth == "w" {
                mode |= Self::IWOTH
            }

            match xoth {
                "x" => mode |= Self::IXOTH,
                "t" => mode |= Self::IXOTH | Self::ISVTX,
                "T" => mode |= Self::ISVTX,
                _ => (),
            }
            Ok(mode)
        } else {
            Err(flow_record_common::Error::InvalidModeString(value.into()))
        }
    }
}

impl ToMsgPackValue for FileMode {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Integer(self.bits().into())
    }

    fn field_type() -> flow_record_common::FieldType {
        FieldType::UnixFileMode
    }
}

#[cfg(test)]
mod tests {
    use crate::artifacts::posix::FileMode;

    #[test]
    fn test_single_flags() {
        assert_eq!(FileMode::try_from("r--------").unwrap(), FileMode::IRUSR);
        assert_eq!(FileMode::try_from("-w-------").unwrap(), FileMode::IWUSR);
        assert_eq!(FileMode::try_from("--x------").unwrap(), FileMode::IXUSR);
        assert_eq!(FileMode::try_from("---r-----").unwrap(), FileMode::IRGRP);
        assert_eq!(FileMode::try_from("----w----").unwrap(), FileMode::IWGRP);
        assert_eq!(FileMode::try_from("-----x---").unwrap(), FileMode::IXGRP);
        assert_eq!(FileMode::try_from("------r--").unwrap(), FileMode::IROTH);
        assert_eq!(FileMode::try_from("-------w-").unwrap(), FileMode::IWOTH);
        assert_eq!(FileMode::try_from("--------x").unwrap(), FileMode::IXOTH);

        assert_eq!(FileMode::try_from("--S------").unwrap(), FileMode::ISUID);
        assert_eq!(FileMode::try_from("-----S---").unwrap(), FileMode::ISGID);
        assert_eq!(FileMode::try_from("--------T").unwrap(), FileMode::ISVTX);

        assert_eq!(
            FileMode::try_from("--s------").unwrap(),
            FileMode::IXUSR | FileMode::ISUID
        );
        assert_eq!(
            FileMode::try_from("-----s---").unwrap(),
            FileMode::IXGRP | FileMode::ISGID
        );
        assert_eq!(
            FileMode::try_from("--------t").unwrap(),
            FileMode::IXOTH | FileMode::ISVTX
        );
    }

    #[test]
    fn test_simple_modes() {
        assert_eq!(
            FileMode::try_from("rwxrwxrwx").unwrap(),
            FileMode::from_bits(0o777).unwrap()
        );
        assert_eq!(
            FileMode::try_from("rwxr-xr-x").unwrap(),
            FileMode::from_bits(0o755).unwrap()
        );
        assert_eq!(
            FileMode::try_from("rw-r--r--").unwrap(),
            FileMode::from_bits(0o644).unwrap()
        );
        assert_eq!(
            FileMode::try_from("r--------").unwrap(),
            FileMode::from_bits(0o400).unwrap()
        );
    }
}
