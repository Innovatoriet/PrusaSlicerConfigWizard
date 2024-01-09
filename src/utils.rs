use simple_home_dir::home_dir;
use std::path::PathBuf;

/// Get the config dir depending on the OS
///
/// For Linux and macOS it is `~/.config/custom_prusa_config_wizard` and for
/// Windows it is `~/AppData/Roaming/custom_prusa_config_wizard`
///
/// # Returns
///
/// Wither a path to the config dir or `None` if the OS is not supported or
/// getting the home dir failed
pub fn get_config_dir() -> Option<PathBuf> {
    let mut home = home_dir()?;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    home.push(".config/prusa_custom_config_wizard");

    #[cfg(target_os = "windows")]
    home.push("AppData/Roaming/prusa_custom_config_wizard");

    // TODO: Add support for other OS
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    unimplemented!("Config dir not implemented for this OS");

    Some(home)
}

/// Get the PrusaSlicer config dir depending on the OS
///
/// For macOS it is `~/.config/PrusaSlicer`
/// No other OS is supported yet
///
/// # Returns
///
/// Wither a path to the PrusaSlicer config dir or `None` if the OS is not
/// supported or getting the home dir failed
pub fn get_prusa_dir() -> Option<PathBuf> {
    let mut home = home_dir()?;

    #[cfg(target_os = "macos")]
    home.push("Library/Application Support/PrusaSlicer");

    // TODO: Add support for other OS
    #[cfg(not(target_os = "macos"))]
    unimplemented!("PrusaSlicer config dir not implemented for this OS");

    Some(home)
}
