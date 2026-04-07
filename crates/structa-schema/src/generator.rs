use crate::app::AppSchema;
use crate::dto::{DtoSchema, FieldType};
use crate::entity::{ColumnType, EntitySchema};
use crate::module::ModuleSchema;
use crate::route::RouteSchema;
use crate::service::ServiceSchema;
use crate::SchemaFile;

pub struct CodeGenerator {
    indent: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent)
    }

    pub fn generate(&mut self, schema: SchemaFile) -> String {
        match schema {
            SchemaFile::Dto(dto) => self.generate_dto(&dto),
            SchemaFile::Route(route) => self.generate_route(&route),
            SchemaFile::Module(module) => self.generate_module(&module),
            SchemaFile::Entity(entity) => self.generate_entity(&entity),
            SchemaFile::Service(service) => self.generate_service(&service),
            SchemaFile::App(app) => self.generate_app(&app),
        }
    }

    pub fn generate_all(&mut self, schemas: Vec<SchemaFile>) -> Vec<(String, String)> {
        schemas
            .into_iter()
            .map(|s| {
                let name = match &s {
                    SchemaFile::Dto(d) => format!("{}.dto.structa", d.name),
                    SchemaFile::Route(r) => format!("{}.route.structa", r.name),
                    SchemaFile::Module(m) => format!("{}.module.structa", m.name),
                    SchemaFile::Entity(e) => format!("{}.entity.structa", e.name),
                    SchemaFile::Service(s) => format!("{}.service.structa", s.name),
                    SchemaFile::App(_) => "app.structa".to_string(),
                };
                let code = self.generate(s);
                (name, code)
            })
            .collect()
    }

    fn generate_dto(&mut self, dto: &DtoSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated DTO\n\n");
        output.push_str(&format!("dto {} {{\n", dto.name));

        self.indent += 1;
        for field in &dto.fields {
            let optional = if field.optional { "?" } else { "" };
            let field_type = self.field_type_to_structa(&field.field_type);
            output.push_str(&format!(
                "{}{}: {}{}\n",
                self.indent(),
                field.name,
                field_type,
                optional
            ));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn field_type_to_structa(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::String => "string".to_string(),
            FieldType::Number => "float".to_string(),
            FieldType::Integer => "int".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::UUID => "uuid".to_string(),
            FieldType::Email => "string".to_string(),
            FieldType::Enum(values) => format!("'{}'", values.join(" | ")),
            FieldType::Array(inner) => format!("{}[]", self.field_type_to_structa(inner)),
            FieldType::Object(fields) => {
                let fields_str = fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, self.field_type_to_structa(&f.field_type)))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", fields_str)
            }
            FieldType::Custom(t) => t.clone(),
        }
    }

    fn column_type_to_structa(&self, col_type: &ColumnType) -> String {
        match col_type {
            ColumnType::Int => "int".to_string(),
            ColumnType::BigInt => "bigint".to_string(),
            ColumnType::SmallInt | ColumnType::TinyInt => "int".to_string(),
            ColumnType::Float => "float".to_string(),
            ColumnType::Double => "double".to_string(),
            ColumnType::Decimal => "decimal".to_string(),
            ColumnType::Boolean => "boolean".to_string(),
            ColumnType::String | ColumnType::Text | ColumnType::Varchar | ColumnType::Char => {
                "string".to_string()
            }
            ColumnType::Date => "date".to_string(),
            ColumnType::Time => "time".to_string(),
            ColumnType::DateTime => "datetime".to_string(),
            ColumnType::Timestamp => "timestamp".to_string(),
            ColumnType::Json => "json".to_string(),
            ColumnType::Uuid => "uuid".to_string(),
            ColumnType::Enum(values) => format!("'{}'", values.join(" | ")),
            ColumnType::Custom(s) => s.clone(),
        }
    }

    fn generate_route(&mut self, route: &RouteSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated Controller\n\n");
        output.push_str(&format!("controller {} {{\n", capitalize(&route.name)));
        output.push_str(&format!("    path: \"{}\"\n\n", route.path));

        self.indent += 1;
        for r in &route.routes {
            if let Some(summary) = &r.summary {
                output.push_str(&format!("{}// {}\n", self.indent(), summary));
            }

            let method = r.method.to_uppercase();
            output.push_str(&format!("{}@{}(\"{}\")\n", self.indent(), method, r.path));
            output.push_str(&format!("{}async {}(data) {{\n", self.indent(), r.handler));
            self.indent += 1;
            output.push_str(&format!("{}return {{}}\n", self.indent()));
            self.indent -= 1;
            output.push_str(&format!("{}}}\n\n", self.indent()));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_module(&mut self, module: &ModuleSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated Module\n\n");
        output.push_str(&format!("module {} {{\n", capitalize(&module.name)));

        if !module.controllers.is_empty() {
            output.push_str("    controllers: [");
            output.push_str(&module.controllers.join(", "));
            output.push_str("]\n");
        }

        if !module.services.is_empty() {
            output.push_str("    services: [");
            output.push_str(&module.services.join(", "));
            output.push_str("]\n");
        }

        if !module.exports.is_empty() {
            output.push_str("    exports: [");
            output.push_str(&module.exports.join(", "));
            output.push_str("]\n");
        }

        output.push_str("}\n");
        output
    }

    fn generate_entity(&mut self, entity: &EntitySchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated Entity\n\n");

        let table_name = entity
            .table_name
            .as_deref()
            .unwrap_or(&entity.name.to_lowercase());
        output.push_str(&format!("entity {} {{\n", capitalize(&entity.name)));
        output.push_str(&format!("    table: {}\n\n", table_name));

        self.indent += 1;
        for col in &entity.columns {
            let nullable = if col.nullable { "?" } else { "" };
            let col_type = self.column_type_to_structa(&col.column_type);

            let mut modifiers = String::new();
            if col.primary {
                modifiers.push_str(" pk");
            }
            if col.auto_increment {
                modifiers.push_str(" auto");
            }
            if col.unique {
                modifiers.push_str(" unique");
            }
            if let Some(default) = &col.default {
                modifiers.push_str(&format!(" default({})", default));
            }

            output.push_str(&format!(
                "{}{}: {}{}{}\n",
                self.indent(),
                col.name,
                col_type,
                nullable,
                modifiers
            ));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_service(&mut self, service: &ServiceSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated Service\n\n");
        output.push_str(&format!("service {} {{\n", capitalize(&service.name)));

        if !service.dependencies.is_empty() {
            self.indent += 1;
            for dep in &service.dependencies {
                let dep_lower = dep.to_lowercase();
                output.push_str(&format!("{}@Inject(\"{}\")\n", self.indent(), dep));
                output.push_str(&format!("{}    {}\n\n", self.indent(), dep_lower));
            }
            self.indent -= 1;
        }

        self.indent += 1;
        for method in &service.methods {
            let params = method
                .params
                .iter()
                .map(|p| {
                    let optional = if p.optional { "?" } else { "" };
                    format!("{}{}", p.name, optional)
                })
                .collect::<Vec<_>>()
                .join(", ");

            output.push_str(&format!(
                "{}async {}({}) {{\n",
                self.indent(),
                method.name,
                params
            ));

            if let Some(body) = &method.body {
                self.indent += 1;
                for line in body.lines() {
                    output.push_str(&format!("{}{}\n", self.indent(), line));
                }
                self.indent -= 1;
            } else {
                self.indent += 1;
                output.push_str(&format!("{}return {{}}\n", self.indent()));
                self.indent -= 1;
            }

            output.push_str(&format!("{}}}\n\n", self.indent()));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_app(&mut self, app: &AppSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated App Configuration\n\n");

        output.push_str("// Controllers\n");
        for module in &app.modules {
            output.push_str(&format!(
                "// import {{ {} }} from './{}.structa'\n",
                module,
                module.to_lowercase()
            ));
        }

        output.push_str("\n// Middleware\n");
        for mw in &app.middlewares {
            output.push_str(&format!(
                "// import {{ {} }} from './{}.middleware.structa'\n",
                mw.name,
                mw.name.to_lowercase()
            ));
        }

        output
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
