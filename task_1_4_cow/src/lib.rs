use std::env;
use std::borrow::Cow;


pub fn cow_main() {
    let args: Vec<String> = env::args().collect();
    let conf_arg = args.iter().find(|arg| arg.starts_with("--conf="));
    let config_path: Cow<str> = if let Some(arg) = conf_arg {
        let path = arg.split('=').nth(1).unwrap_or("");
        if path.is_empty() {
            eprintln!("Error: --conf argument is empty");
            std::process::exit(1);
        }
        Cow::Borrowed(path)
    } else if let Ok(path) = env::var("APP_CONF") {
        if !path.is_empty() {
            Cow::Owned(path)
        } else {
            Cow::Owned("/etc/app/app.conf".to_string())
        }
    } else {
        Cow::Owned("/etc/app/app.conf".to_string())
    };
    println!("Using config path: {}", config_path);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_path() {
        let args = vec!["program".to_string(), "--conf=/custom/path".to_string()];
        let conf_arg = args.iter().find(|arg| arg.starts_with("--conf="));
        assert_eq!(conf_arg.unwrap(), "--conf=/custom/path");
    }
}