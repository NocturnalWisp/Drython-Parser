#[derive(Debug, Clone, PartialEq)]
pub enum VariableModifier
{
    Unkown(String),
    // Able to externally reference other scripts.
    External,
    // Listable with other public values for external use.
    Public,
    // Make the variable non modifiable.
    Const,
    // Define aliases for variables and functions.
    Alias,
}

impl From<&str> for VariableModifier
{
    fn from(s: &str) -> Self
    {
        match s.to_lowercase().as_str()
        {
            "external" => VariableModifier::External,
            "pub"|"public" => VariableModifier::Public,
            "const"|"constant" => VariableModifier::Const,
            "alias" => VariableModifier::Alias,
            _ => VariableModifier::Unkown(s.to_string())
        }
    }
}

impl VariableModifier
{
    pub fn check_scope_allowed(&self) -> bool
    {
        match self
        {
            VariableModifier::Const => true,
            VariableModifier::Alias => true,
            VariableModifier::Public => false,
            VariableModifier::External => false,
            _ => false,
        }
    }
}
