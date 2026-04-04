use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySchema {
    pub name: String,
    pub table_name: Option<String>,
    pub columns: Vec<Column>,
    pub relations: Vec<Relation>,
    pub indices: Vec<Index>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub primary: bool,
    pub auto_increment: bool,
    pub unique: bool,
    pub default: Option<serde_json::Value>,
    pub length: Option<usize>,
    pub precision: Option<usize>,
    pub scale: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnType {
    Int,
    BigInt,
    SmallInt,
    TinyInt,
    Float,
    Double,
    Decimal,
    Boolean,
    String,
    Text,
    Varchar,
    Char,
    Date,
    Time,
    DateTime,
    Timestamp,
    Json,
    Uuid,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub name: String,
    pub relation_type: RelationType,
    pub target: String,
    pub foreign_key: Option<String>,
    pub nullable: bool,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub unique: bool,
}

pub fn parse_entity(name: &str, content: &str) -> Result<EntitySchema, String> {
    let mut schema = EntitySchema {
        name: name.to_string(),
        table_name: None,
        columns: Vec::new(),
        relations: Vec::new(),
        indices: Vec::new(),
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if line.starts_with("table ") || line.starts_with("tableName ") {
            if let Some(name) = line.strip_prefix("table ") {
                schema.table_name = Some(name.trim().to_string());
            } else if let Some(name) = line.strip_prefix("tableName ") {
                schema.table_name = Some(name.trim().to_string());
            }
        } else if line.starts_with("index ") {
            if let Some(index) = parse_index(line) {
                schema.indices.push(index);
            }
        } else if let Some(relation) = parse_relation(line) {
            schema.relations.push(relation);
        } else if let Some(column) = parse_column(line) {
            schema.columns.push(column);
        }
    }

    Ok(schema)
}

fn parse_column(line: &str) -> Option<Column> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let name = parts[0].trim().to_string();
    let type_str = parts[1].trim();

    let mut nullable = false;
    let mut primary = false;
    let mut auto_increment = false;
    let mut unique = false;
    let mut length: Option<usize> = None;
    let mut default: Option<serde_json::Value> = None;

    let modifiers: Vec<&str> = parts[2..].to_vec();
    for m in modifiers {
        let m = m.trim();
        match m {
            "?" | "nullable" => nullable = true,
            "pk" | "primary" => primary = true,
            "auto" | "autoincrement" | "auto_increment" => auto_increment = true,
            "unique" => unique = true,
            s if s.starts_with("length(") && s.ends_with(')') => {
                length = s
                    .trim_start_matches("length(")
                    .trim_end_matches(')')
                    .parse()
                    .ok();
            }
            s if s.starts_with("default(") && s.ends_with(')') => {
                let val = s
                    .trim_start_matches("default(")
                    .trim_end_matches(')')
                    .trim();
                default = Some(if val == "null" {
                    serde_json::Value::Null
                } else if val == "true" {
                    serde_json::Value::Bool(true)
                } else if val == "false" {
                    serde_json::Value::Bool(false)
                } else if val.starts_with('"') || val.starts_with('\'') {
                    serde_json::Value::String(val.trim_matches('"').trim_matches('\'').to_string())
                } else {
                    serde_json::Value::String(val.to_string())
                });
            }
            _ => {}
        }
    }

    let column_type = match type_str {
        "int" | "integer" => ColumnType::Int,
        "bigint" => ColumnType::BigInt,
        "smallint" => ColumnType::SmallInt,
        "tinyint" => ColumnType::TinyInt,
        "float" => ColumnType::Float,
        "double" => ColumnType::Double,
        "decimal" => ColumnType::Decimal,
        "bool" | "boolean" => ColumnType::Boolean,
        "string" | "str" => ColumnType::String,
        "text" => ColumnType::Text,
        "varchar" => ColumnType::Varchar,
        "char" => ColumnType::Char,
        "date" => ColumnType::Date,
        "time" => ColumnType::Time,
        "datetime" | "timestamp" => ColumnType::DateTime,
        "json" => ColumnType::Json,
        "uuid" => ColumnType::Uuid,
        s if s.starts_with("enum(") && s.ends_with(')') => {
            let values_str = s.trim_start_matches("enum(").trim_end_matches(')');
            let values: Vec<String> = values_str
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            ColumnType::Enum(values)
        }
        s => ColumnType::Custom(s.to_string()),
    };

    Some(Column {
        name,
        column_type,
        nullable,
        primary,
        auto_increment,
        unique,
        default,
        length,
        precision: None,
        scale: None,
    })
}

fn parse_relation(line: &str) -> Option<Relation> {
    if !line.starts_with('@') {
        return None;
    }

    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }

    let annotation = parts[0].trim_start_matches('@');
    let target = parts[1].trim().to_string();

    let relation_type = match annotation {
        "OneToOne" | "oneToOne" | "one-to-one" => RelationType::OneToOne,
        "OneToMany" | "oneToMany" | "one-to-many" => RelationType::OneToMany,
        "ManyToOne" | "manyToOne" | "many-to-one" => RelationType::ManyToOne,
        "ManyToMany" | "manyToMany" | "many-to-many" => RelationType::ManyToMany,
        _ => return None,
    };

    let name = target.to_lowercase();

    Some(Relation {
        name,
        relation_type,
        target,
        foreign_key: None,
        nullable: true,
        on_delete: None,
        on_update: None,
    })
}

fn parse_index(line: &str) -> Option<Index> {
    if !line.starts_with("index ") && !line.starts_with("unique ") {
        return None;
    }

    let (unique, content) = if line.starts_with("unique ") {
        (true, line.strip_prefix("unique ").unwrap())
    } else {
        (false, line.strip_prefix("index ").unwrap())
    };

    let parts: Vec<&str> = content.split(':').collect();
    let name = parts.get(1).map(|s| s.trim().to_string());
    let columns: Vec<String> = parts[0].split(',').map(|s| s.trim().to_string()).collect();

    Some(Index {
        name,
        columns,
        unique,
    })
}
