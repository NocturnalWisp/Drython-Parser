#[derive(Debug, Clone, Copy)]
pub enum ScriptType
{
    None,
    Component,
    System
}

impl ScriptType
{
    // Returns the enum type associated with a particular string.
    pub fn from_string(string: &str) -> ScriptType
    {
        match string
        {
            "Component" => return ScriptType::Component,
            "System" => return ScriptType::System,
            _ => return ScriptType::None
        }
    }
}
