use std::backtrace::Backtrace;

/// Main error type.
#[derive(Debug)]
pub struct KError {
    pub desc: Option<String>,
    pub backtrace: Backtrace,
}

impl PartialEq for KError {
    fn eq(&self, other: &Self) -> bool {
        self.desc.eq(&other.desc)
    }
}

/// Main result type. Alias for [`std::result::Result`]`<T, `[`KError`]`<'static>>`.
pub type KResult<T> = std::result::Result<T, KError>;

#[macro_export]
macro_rules! kerror {
    ($desc:expr) => {
        $crate::error::KError {
            desc: Some($desc.to_string()),
            backtrace: std::backtrace::Backtrace::capture(),
        }
    };
    () => {
        $crate::error::KError {
            desc: None,
            backtrace: std::backtrace::Backtrace::capture(),
        }
    };
}

#[macro_export]
macro_rules! kbail {
    ($desc:expr) => {
        return Err($crate::kerror!($desc))
    };
    () => {
        return Err($crate::kerror!())
    };
}

#[macro_export]
macro_rules! kensure {
    ($cond:expr, $desc:expr) => {
        if !$cond {
            $crate::kbail!($desc)
        }
    };

    ($cond:expr) => {
        if !$cond {
            $crate::kbail!()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kerror() {
        let e = kerror!();
        assert_eq!(e.desc, None);
        let e = kerror!("Uh oh!");
        assert_eq!(e.desc, Some("Uh oh!".to_string()));
    }

    #[test]
    fn test_kbail() {
        fn success() -> KResult<()> {
            Ok(())
        }

        fn fail() -> KResult<()> {
            kbail!("Uh oh!")
        }

        assert_eq!(success(), Ok(()));
        if let Err(e) = fail() {
            assert_eq!(e.desc, Some("Uh oh!".to_string()))
        } else {
            panic!()
        }
    }

    #[test]
    fn test_kensure() {
        fn success() -> KResult<()> {
            kensure!(1 + 1 == 2);
            Ok(())
        }

        fn fail() -> KResult<()> {
            kensure!(1 + 1 == 0, "Uh oh!");
            Ok(())
        }

        assert_eq!(success(), Ok(()));
        if let Err(e) = fail() {
            assert_eq!(e.desc, Some("Uh oh!".to_string()))
        } else {
            panic!()
        }
    }
}
