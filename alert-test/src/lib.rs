use rust_alert::alert;

#[derive(Debug, Clone)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct AnotherError(String);

impl std::fmt::Display for AnotherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[alert(errors = [MyError, AnotherError])]
pub struct Alert {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_my_error() {
        let error = MyError("test error".to_string());
        let alert: Alert = error.into();
        assert_eq!(alert.message, "MyError: test error");
    }

    #[test]
    fn test_from_another_error() {
        let error = AnotherError("another test".to_string());
        let alert: Alert = error.into();
        assert_eq!(alert.message, "AnotherError: another test");
    }

    #[test]
    fn test_display_implementation() {
        let alert = Alert {
            message: "display test".to_string(),
        };
        assert_eq!(format!("{}", alert), "display test\n");
    }

    #[test]
    fn test_clone() {
        let alert1 = Alert {
            message: "test".to_string(),
        };
        let alert2 = alert1.clone();
        assert_eq!(alert1, alert2);
    }

    #[test]
    fn test_debug() {
        let alert = Alert {
            message: "debug test".to_string(),
        };
        let debug_str = format!("{:?}", alert);
        assert!(debug_str.contains("message"));
    }
}
