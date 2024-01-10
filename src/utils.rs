use simple_home_dir::home_dir;
use std::path::PathBuf;

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

    // TODO: Add support for other OSes
    #[cfg(not(target_os = "macos"))]
    unimplemented!("PrusaSlicer config dir not implemented for this OS");

    Some(home)
}
