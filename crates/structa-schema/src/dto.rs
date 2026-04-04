use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoSchema {
    pub name: String,
    pub fields: Vec<DtoField>,
    pub validation: Option<Vec<ValidationRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoField {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
    pub nullable: bool,
    pub default: Option<serde_json::Value>,
    pub validation: Option<Vec<ValidationRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Integer,
    Boolean,
    Date,
    DateTime,
    UUID,
    Email,
    Enum(Vec<String>),
    Array(Box<FieldType>),
    Object(Vec<DtoField>),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule: String,
    pub params: Option<Vec<serde_json::Value>>,
    pub message: Option<String>,
}

pub fn parse_dto(name: &str, content: &str) -> Result<DtoSchema, String> {
    let mut schema = DtoSchema {
        name: name.to_string(),
        fields: Vec::new(),
        validation: None,
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if let Some(field) = parse_field(line) {
            schema.fields.push(field);
        }
    }

    Ok(schema)
}

fn parse_field(line: &str) -> Option<DtoField> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let name = parts[0].trim().to_string();
    let type_str = parts[1].trim();

    let field_type = match type_str {
        "string" | "str" => FieldType::String,
        "number" | "num" | "float" => FieldType::Number,
        "int" | "integer" => FieldType::Integer,
        "bool" | "boolean" => FieldType::Boolean,
        "date" => FieldType::Date,
        "datetime" | "timestamp" => FieldType::DateTime,
        "uuid" => FieldType::UUID,
        "email" => FieldType::Email,
        s if s.starts_with("enum(") && s.ends_with(')') => {
            let values_str = s.trim_start_matches("enum(").trim_end_matches(')');
            let values: Vec<String> = values_str
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            FieldType::Enum(values)
        }
        s if s.starts_with("array<") && s.ends_with('>') => {
            let inner_str = s.trim_start_matches("array<").trim_end_matches('>');
            let inner_type = match inner_str {
                "string" => FieldType::String,
                "number" => FieldType::Number,
                "int" => FieldType::Integer,
                "bool" => FieldType::Boolean,
                _ => FieldType::Custom(inner_str.to_string()),
            };
            FieldType::Array(Box::new(inner_type))
        }
        s if s.starts_with('{') && s.ends_with('}') => {
            let inner_content = s.trim_start_matches('{').trim_end_matches('}');
            let inner_fields: Vec<DtoField> = inner_content
                .split(',')
                .filter_map(|p| {
                    let parts: Vec<&str> = p.split(':').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim().to_string();
                        let type_str = parts[1].trim();
                        Some(DtoField {
                            name,
                            field_type: parse_simple_type(type_str),
                            optional: false,
                            nullable: false,
                            default: None,
                            validation: None,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            FieldType::Object(inner_fields)
        }
        s => FieldType::Custom(s.to_string()),
    };

    let optional = type_str.ends_with('?') || type_str.ends_with(" | null");
    let nullable = type_str.contains(" | null") || type_str.contains('?');

    Some(DtoField {
        name,
        field_type,
        optional,
        nullable,
        default: None,
        validation: None,
    })
}

fn parse_simple_type(type_str: &str) -> FieldType {
    match type_str.trim_end_matches("?").trim_end_matches(" | null") {
        "string" => FieldType::String,
        "number" => FieldType::Number,
        "int" => FieldType::Integer,
        "bool" => FieldType::Boolean,
        "date" => FieldType::Date,
        "datetime" => FieldType::DateTime,
        "uuid" => FieldType::UUID,
        "email" => FieldType::Email,
        s => FieldType::Custom(s.to_string()),
    }
}
