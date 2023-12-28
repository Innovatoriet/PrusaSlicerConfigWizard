
/// Write ini file
pub fn format_ini( out: &mut String, contents: Vec<(String, String)>) {

    for (k, v) in contents {
        let formated = format!("{} = {}\n", k, v);
        out.push_str(formated.as_str());
    }

}
