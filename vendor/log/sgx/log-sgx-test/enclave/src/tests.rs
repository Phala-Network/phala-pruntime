//#[cfg(test)]
pub mod tests {
    use std::prelude::v1::*;
    use log::{Level, LevelFilter, ParseLevelError};
    use std::string::ToString;

    //#[test]
    pub fn test_levelfilter_from_str() {
        let tests = [
            ("off", Ok(LevelFilter::Off)),
            ("error", Ok(LevelFilter::Error)),
            ("warn", Ok(LevelFilter::Warn)),
            ("info", Ok(LevelFilter::Info)),
            ("debug", Ok(LevelFilter::Debug)),
            ("trace", Ok(LevelFilter::Trace)),
            ("OFF", Ok(LevelFilter::Off)),
            ("ERROR", Ok(LevelFilter::Error)),
            ("WARN", Ok(LevelFilter::Warn)),
            ("INFO", Ok(LevelFilter::Info)),
            ("DEBUG", Ok(LevelFilter::Debug)),
            ("TRACE", Ok(LevelFilter::Trace)),
            ("asdf", Err(ParseLevelError::default())),
        ];
        for &(s, ref expected) in &tests {
            assert_eq!(expected, &s.parse());
        }
    }

    //#[test]
    pub fn test_level_from_str() {
        let tests = [
            ("OFF", Err(ParseLevelError::default())),
            ("error", Ok(Level::Error)),
            ("warn", Ok(Level::Warn)),
            ("info", Ok(Level::Info)),
            ("debug", Ok(Level::Debug)),
            ("trace", Ok(Level::Trace)),
            ("ERROR", Ok(Level::Error)),
            ("WARN", Ok(Level::Warn)),
            ("INFO", Ok(Level::Info)),
            ("DEBUG", Ok(Level::Debug)),
            ("TRACE", Ok(Level::Trace)),
            ("asdf", Err(ParseLevelError::default())),
        ];
        for &(s, ref expected) in &tests {
            assert_eq!(expected, &s.parse());
        }
    }

    //#[test]
    pub fn test_level_show() {
        assert_eq!("INFO", Level::Info.to_string());
        assert_eq!("ERROR", Level::Error.to_string());
    }

    //#[test]
    pub fn test_levelfilter_show() {
        assert_eq!("OFF", LevelFilter::Off.to_string());
        assert_eq!("ERROR", LevelFilter::Error.to_string());
    }

    //#[test]
    pub fn test_cross_cmp() {
        assert!(Level::Debug > LevelFilter::Error);
        assert!(LevelFilter::Warn < Level::Trace);
        assert!(LevelFilter::Off < Level::Error);
    }

    //#[test]
    pub fn test_cross_eq() {
        assert!(Level::Error == LevelFilter::Error);
        assert!(LevelFilter::Off != Level::Error);
        assert!(Level::Trace == LevelFilter::Trace);
    }

    //#[test]
    pub fn test_to_level() {
        assert_eq!(Some(Level::Error), LevelFilter::Error.to_level());
        assert_eq!(None, LevelFilter::Off.to_level());
        assert_eq!(Some(Level::Debug), LevelFilter::Debug.to_level());
    }

    //#[test]
    pub fn test_to_level_filter() {
        assert_eq!(LevelFilter::Error, Level::Error.to_level_filter());
        assert_eq!(LevelFilter::Trace, Level::Trace.to_level_filter());
    }

    //#[test]
    //#[cfg(feature = "std")]
    #[allow(deprecated)]
    pub fn test_error_trait() {
        use log::SetLoggerError;
        use std::error::Error;
        let e = SetLoggerError::default();
        assert_eq!(
            e.description(),
            "attempted to set a logger after the logging system \
             was already initialized"
        );
    }

    //#[test]
    pub fn test_metadata_builder() {
        use log::MetadataBuilder;
        let target = "myApp";
        let metadata_test = MetadataBuilder::new()
            .level(Level::Debug)
            .target(target)
            .build();
        assert_eq!(metadata_test.level(), Level::Debug);
        assert_eq!(metadata_test.target(), "myApp");
    }

    //#[test]
    pub fn test_metadata_convenience_builder() {
        use log::Metadata;
        let target = "myApp";
        let metadata_test = Metadata::builder()
            .level(Level::Debug)
            .target(target)
            .build();
        assert_eq!(metadata_test.level(), Level::Debug);
        assert_eq!(metadata_test.target(), "myApp");
    }

    //#[test]
    pub fn test_record_builder() {
        use log::{MetadataBuilder, RecordBuilder};
        let target = "myApp";
        let metadata = MetadataBuilder::new().target(target).build();
        let fmt_args = format_args!("hello");
        let record_test = RecordBuilder::new()
            .args(fmt_args)
            .metadata(metadata)
            .module_path(Some("foo"))
            .file(Some("bar"))
            .line(Some(30))
            .build();
        assert_eq!(record_test.metadata().target(), "myApp");
        assert_eq!(record_test.module_path(), Some("foo"));
        assert_eq!(record_test.file(), Some("bar"));
        assert_eq!(record_test.line(), Some(30));
    }

    //#[test]
    pub fn test_record_convenience_builder() {
        use log::{Metadata, Record};
        let target = "myApp";
        let metadata = Metadata::builder().target(target).build();
        let fmt_args = format_args!("hello");
        let record_test = Record::builder()
            .args(fmt_args)
            .metadata(metadata)
            .module_path(Some("foo"))
            .file(Some("bar"))
            .line(Some(30))
            .build();
        assert_eq!(record_test.target(), "myApp");
        assert_eq!(record_test.module_path(), Some("foo"));
        assert_eq!(record_test.file(), Some("bar"));
        assert_eq!(record_test.line(), Some(30));
    }

    //#[test]
    pub fn test_record_complete_builder() {
        use log::Record;
        let target = "myApp";
        let record_test = Record::builder()
            .module_path(Some("foo"))
            .file(Some("bar"))
            .line(Some(30))
            .target(target)
            .level(Level::Error)
            .build();
        assert_eq!(record_test.target(), "myApp");
        assert_eq!(record_test.level(), Level::Error);
        assert_eq!(record_test.module_path(), Some("foo"));
        assert_eq!(record_test.file(), Some("bar"));
        assert_eq!(record_test.line(), Some(30));
    }
}
