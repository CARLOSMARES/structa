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

    fn with_indent(&mut self, f: impl FnOnce(&mut Self)) -> String {
        self.indent += 1;
        f(self);
        self.indent -= 1;
        String::new()
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
                    SchemaFile::Dto(d) => format!("{}.dto.ts", d.name),
                    SchemaFile::Route(r) => format!("{}.routes.ts", r.name),
                    SchemaFile::Module(m) => format!("{}.module.ts", m.name),
                    SchemaFile::Entity(e) => format!("{}.entity.ts", e.name),
                    SchemaFile::Service(s) => format!("{}.service.ts", s.name),
                    SchemaFile::App(_) => "app.config.ts".to_string(),
                };
                let code = self.generate(s);
                (name, code)
            })
            .collect()
    }

    fn generate_dto(&mut self, dto: &DtoSchema) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Auto-generated DTO: {}\n", dto.name));
        output.push_str("import { IsString, IsNumber, IsBoolean, IsOptional, IsUUID, IsEmail } from 'class-validator';\n");
        output.push_str("import { Type } from 'class-transformer';\n\n");

        output.push_str(&format!("export class {}Dto {{\n", capitalize(&dto.name)));

        self.indent += 1;
        for field in &dto.fields {
            let decorators = self.generate_dto_decorators(&field.field_type, field.optional);

            for dec in decorators {
                output.push_str(&format!("{}{}\n", self.indent(), dec));
            }

            let optional = if field.optional { "?" } else { "" };
            let ts_type = self.field_type_to_ts(&field.field_type);
            output.push_str(&format!(
                "{}{}{}: {};\n\n",
                self.indent(),
                field.name,
                optional,
                ts_type
            ));
        }
        self.indent -= 1;

        output.push_str("}\n\n");

        output.push_str(&format!(
            "export class Create{}Dto extends {}Dto {{}}\n",
            capitalize(&dto.name),
            capitalize(&dto.name)
        ));
        output.push_str(&format!(
            "export class Update{}Dto {{\n",
            capitalize(&dto.name)
        ));
        self.indent += 1;
        for field in &dto.fields {
            let ts_type = self.field_type_to_ts(&field.field_type);
            output.push_str(&format!("{}@IsOptional()\n", self.indent()));
            output.push_str(&format!(
                "{}{}?: {};\n\n",
                self.indent(),
                field.name,
                ts_type
            ));
        }
        self.indent -= 1;
        output.push_str("}\n");

        output
    }

    fn generate_dto_decorators(&self, field_type: &FieldType, optional: bool) -> Vec<String> {
        let mut decorators = Vec::new();

        if optional {
            decorators.push("@IsOptional()".to_string());
        }

        match field_type {
            FieldType::String => decorators.push("@IsString()".to_string()),
            FieldType::Number | FieldType::Integer => decorators.push("@IsNumber()".to_string()),
            FieldType::Boolean => decorators.push("@IsBoolean()".to_string()),
            FieldType::UUID => decorators.push("@IsUUID()".to_string()),
            FieldType::Email => decorators.push("@IsEmail()".to_string()),
            _ => {}
        }

        decorators
    }

    fn field_type_to_ts(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::String => "string".to_string(),
            FieldType::Number => "number".to_string(),
            FieldType::Integer => "number".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Date | FieldType::DateTime => "Date".to_string(),
            FieldType::UUID => "string".to_string(),
            FieldType::Email => "string".to_string(),
            FieldType::Enum(values) => format!("'{}'", values.join("' | '")),
            FieldType::Array(inner) => format!("{}[]", self.field_type_to_ts(inner)),
            FieldType::Object(fields) => {
                let fields_str = fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, self.field_type_to_ts(&f.field_type)))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("{{ {} }}", fields_str)
            }
            FieldType::Custom(t) => t.clone(),
        }
    }

    fn generate_route(&mut self, route: &RouteSchema) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Auto-generated Routes: {}\n", route.name));
        output.push_str("import 'reflect-metadata';\n");
        output.push_str(
            "import { Controller, Get, Post, Put, Patch, Delete } from '@structa/runtime';\n\n",
        );

        output.push_str(&format!("@Controller('{}')\n", route.path));
        output.push_str(&format!(
            "export class {}Controller {{\n",
            capitalize(&route.name)
        ));

        self.indent += 1;
        for r in &route.routes {
            let method_decorator = match r.method.as_str() {
                "GET" => "Get",
                "POST" => "Post",
                "PUT" => "Put",
                "PATCH" => "Patch",
                "DELETE" => "Delete",
                _ => "Get",
            };

            if let Some(summary) = &r.summary {
                output.push_str(&format!("{}// {}\n", self.indent(), summary));
            }

            output.push_str(&format!(
                "{}{}('{}')\n",
                self.indent(),
                method_decorator,
                r.path
            ));
            output.push_str(&format!("{}async {}(", self.indent(), r.handler));
            output.push_str("): Promise<any> {\n");

            self.indent += 1;
            output.push_str(&format!("{}// TODO: Implement handler\n", self.indent()));
            output.push_str(&format!("{}return {{}};\n", self.indent()));
            self.indent -= 1;

            output.push_str(&format("{}}}\n\n", self.indent()));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_module(&mut self, module: &ModuleSchema) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Auto-generated Module: {}\n", module.name));
        output.push_str("import 'reflect-metadata';\n");
        output.push_str("import { Module } from '@structa/runtime';\n\n");

        output.push_str("imports: [");
        if !module.modules.is_empty() {
            output.push('\n');
            self.indent += 1;
            for m in &module.modules {
                output.push_str(&format!("{}{},\n", self.indent(), m));
            }
            self.indent -= 1;
            output.push_str(&self.indent());
        }
        output.push_str("],\n");

        output.push_str("controllers: [");
        if !module.controllers.is_empty() {
            output.push('\n');
            self.indent += 1;
            for c in &module.controllers {
                output.push_str(&format!("{}{},\n", self.indent(), c));
            }
            self.indent -= 1;
            output.push_str(&self.indent());
        }
        output.push_str("],\n");

        output.push_str("providers: [");
        if !module.services.is_empty() {
            output.push('\n');
            self.indent += 1;
            for s in &module.services {
                output.push_str(&format!("{}{},\n", self.indent(), s));
            }
            self.indent -= 1;
            output.push_str(&self.indent());
        }
        output.push_str("],\n");

        output.push_str("exports: [");
        if !module.exports.is_empty() {
            output.push('\n');
            self.indent += 1;
            for e in &module.exports {
                output.push_str(&format!("{}{},\n", self.indent(), e));
            }
            self.indent -= 1;
            output.push_str(&self.indent());
        }
        output.push_str("],\n");

        format!(
            "@Module({{\n{}}})\nexport class {}Module {{}}\n",
            output,
            capitalize(&module.name)
        )
    }

    fn generate_entity(&mut self, entity: &EntitySchema) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Auto-generated Entity: {}\n", entity.name));
        output.push_str("import 'reflect-metadata';\n");
        output.push_str("import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn } from '@structa/orm';\n\n");

        let table_name = entity
            .table_name
            .as_deref()
            .unwrap_or(&entity.name.to_lowercase());
        output.push_str(&format!("@Entity('{}')\n", table_name));
        output.push_str(&format!(
            "export class {} extends BaseEntity {{\n",
            capitalize(&entity.name)
        ));

        self.indent += 1;
        for col in &entity.columns {
            let decorators = self.generate_entity_decorators(col);
            for dec in decorators {
                output.push_str(&format!("{}{}\n", self.indent(), dec));
            }

            let nullable = if col.nullable { "?" } else { "" };
            let ts_type = self.column_type_to_ts(&col.column_type);
            output.push_str(&format!(
                "{}{}{}: {};\n\n",
                self.indent(),
                col.name,
                nullable,
                ts_type
            ));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_entity_decorators(&self, col: &crate::entity::Column) -> Vec<String> {
        let mut decorators = Vec::new();

        let col_options = self.column_options_to_string(col);

        if col.primary && col.auto_increment {
            decorators.push(format!("@PrimaryGeneratedColumn({})", col_options));
        } else if col.primary {
            decorators.push(format!("@PrimaryColumn({})", col_options));
        } else {
            decorators.push(format!("@Column({})", col_options));
        }

        if col.name == "createdAt" {
            decorators = vec!["@CreateDateColumn()".to_string()];
        } else if col.name == "updatedAt" {
            decorators = vec!["@UpdateDateColumn()".to_string()];
        }

        decorators
    }

    fn column_options_to_string(&self, col: &crate::entity::Column) -> String {
        let mut parts = Vec::new();

        let type_str = match &col.column_type {
            ColumnType::Int => "int".to_string(),
            ColumnType::BigInt => "bigint".to_string(),
            ColumnType::SmallInt => "smallint".to_string(),
            ColumnType::TinyInt => "tinyint".to_string(),
            ColumnType::Float => "float".to_string(),
            ColumnType::Double => "double".to_string(),
            ColumnType::Decimal => "decimal".to_string(),
            ColumnType::Boolean => "boolean".to_string(),
            ColumnType::String => "string".to_string(),
            ColumnType::Text => "text".to_string(),
            ColumnType::Varchar => format!(
                "varchar{}",
                col.length.map(|l| format!("({})", l)).unwrap_or_default()
            ),
            ColumnType::Char => format!(
                "char{}",
                col.length.map(|l| format!("({})", l)).unwrap_or_default()
            ),
            ColumnType::Date => "date".to_string(),
            ColumnType::Time => "time".to_string(),
            ColumnType::DateTime => "datetime".to_string(),
            ColumnType::Timestamp => "timestamp".to_string(),
            ColumnType::Json => "json".to_string(),
            ColumnType::Uuid => "uuid".to_string(),
            ColumnType::Enum(values) => format!(
                "enum({})",
                values
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ColumnType::Custom(s) => s.clone(),
        };

        parts.push(format!("type: '{}'", type_str));

        if col.nullable {
            parts.push("nullable: true".to_string());
        }

        if col.unique {
            parts.push("unique: true".to_string());
        }

        if let Some(default) = &col.default {
            parts.push(format!("default: {}", default));
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!("{{ {} }}", parts.join(", "))
        }
    }

    fn column_type_to_ts(&self, col_type: &ColumnType) -> String {
        match col_type {
            ColumnType::Int | ColumnType::BigInt | ColumnType::SmallInt | ColumnType::TinyInt => {
                "number".to_string()
            }
            ColumnType::Float | ColumnType::Double | ColumnType::Decimal => "number".to_string(),
            ColumnType::Boolean => "boolean".to_string(),
            ColumnType::String | ColumnType::Text | ColumnType::Varchar | ColumnType::Char => {
                "string".to_string()
            }
            ColumnType::Date | ColumnType::Time | ColumnType::DateTime | ColumnType::Timestamp => {
                "Date".to_string()
            }
            ColumnType::Json => "object".to_string(),
            ColumnType::Uuid => "string".to_string(),
            ColumnType::Enum(values) => format!("'{}'", values.join("' | '")),
            ColumnType::Custom(s) => s.clone(),
        }
    }

    fn generate_service(&mut self, service: &ServiceSchema) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Auto-generated Service: {}\n", service.name));
        output.push_str("import 'reflect-metadata';\n");
        output.push_str("import { Injectable } from '@structa/runtime';\n\n");

        let deps = if service.dependencies.is_empty() {
            String::new()
        } else {
            service
                .dependencies
                .iter()
                .map(|d| format!("  private readonly {}: {};\n", d.to_lowercase(), d))
                .collect()
        };

        output.push_str(&format!("@Injectable()\n"));
        output.push_str(&format!(
            "export class {}Service {{\n",
            capitalize(&service.name)
        ));
        output.push_str("  constructor(\n");
        output.push_str(&deps);
        output.push_str("  ) {}\n\n");

        self.indent += 1;
        for method in &service.methods {
            let params = method
                .params
                .iter()
                .map(|p| {
                    let optional = if p.optional { "?" } else { "" };
                    format!("{}: {}", p.name, p.param_type.replace('?', "").trim())
                })
                .collect::<Vec<_>>()
                .join(", ");

            output.push_str(&format!(
                "{}async {}({}): {} {{\n",
                self.indent(),
                method.name,
                params,
                method.return_type
            ));

            if let Some(body) = &method.body {
                self.indent += 1;
                for line in body.lines() {
                    output.push_str(&format!("{}{}\n", self.indent(), line));
                }
                self.indent -= 1;
            } else {
                self.indent += 1;
                output.push_str(&format!("{}// TODO: Implement method\n", self.indent()));
                output.push_str(&format!("{}return null as any;\n", self.indent()));
                self.indent -= 1;
            }

            output.push_str(&format("{}}}\n\n", self.indent()));
        }
        self.indent -= 1;

        output.push_str("}\n");
        output
    }

    fn generate_app(&mut self, app: &AppSchema) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated App Configuration\n");
        output.push_str("import 'reflect-metadata';\n");
        output.push_str("import { StructaApp } from '@structa/runtime';\n\n");

        output.push_str("export const appConfig = {\n");
        self.indent += 1;

        output.push_str(&format!("{}name: '{}',\n", self.indent(), app.name));
        output.push_str(&format!("{}version: '{}',\n", self.indent(), app.version));
        output.push_str(&format!("{}port: {},\n", self.indent(), app.port));
        output.push_str(&format!("{}host: '{}',\n", self.indent(), app.host));

        if !app.modules.is_empty() {
            output.push_str(&format!("{}modules: [\n", self.indent()));
            self.indent += 1;
            for module in &app.modules {
                output.push_str(&format!("{}{},\n", self.indent(), module));
            }
            self.indent -= 1;
            output.push_str(&format!("{}],\n", self.indent()));
        }

        if !app.middlewares.is_empty() {
            output.push_str(&format!("{}middlewares: [\n", self.indent()));
            self.indent += 1;
            for mw in &app.middlewares {
                output.push_str(&format!("{}{},\n", self.indent(), mw.name));
            }
            self.indent -= 1;
            output.push_str(&format!("{}],\n", self.indent()));
        }

        if let Some(cors) = &app.cors {
            output.push_str(&format!("{}cors: {{\n", self.indent()));
            self.indent += 1;
            if let Some(origin) = &cors.origin {
                output.push_str(&format!("{}origin: '{}',\n", self.indent(), origin));
            }
            if let Some(methods) = &cors.methods {
                output.push_str(&format!(
                    "{}methods: [{}],\n",
                    self.indent(),
                    methods
                        .iter()
                        .map(|m| format!("'{}'", m))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if let Some(creds) = cors.credentials {
                output.push_str(&format!("{}credentials: {},\n", self.indent(), creds));
            }
            self.indent -= 1;
            output.push_str(&format!("{}}},\n", self.indent()));
        }

        self.indent -= 1;
        output.push_str("};\n");

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
