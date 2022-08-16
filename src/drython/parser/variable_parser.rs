pub fn parse_var(line: &str) -> Result<(String, String), String>
{
    let mut var: (String, String) = ("".to_string(), "".to_string());
    
    let split: Vec<&str> = line.split("=").collect();

    if split.len() !=2
    {
        return Err("Failed to parse variable assignment.".to_string());
    }

    var.0 = split[0].to_string();
    var.1 = split[1].to_string();

    Ok(var)
}
