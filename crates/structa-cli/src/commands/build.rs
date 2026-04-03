use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, error};
use std::fs;
use std::io::Write;
use structa_codegen::{CodeGenerator, CodegenOptions};
use structa_parser::Parser;
use structa_lexer::Lexer;

pub async fn run(source: PathBuf, output: PathBuf, watch: bool, config: Option<PathBuf>) -> Result<()> {
    info!("Building Structa project from {:?} to {:?}", source, output);
    
    if !source.exists() {
        error!("Source directory does not exist: {:?}", source);
        anyhow::bail!("Source directory does not exist");
    }
    
    fs::create_dir_all(&output)?;
    
    let options = CodegenOptions::default();
    
    info!("Compiling Structa files...");
    
    let structa_files: Vec<PathBuf> = collect_structa_files(&source)?;
    let mut compiled_count = 0;
    let mut error_count = 0;
    
    for file in &structa_files {
        info!("Processing: {:?}", file);
        match compile_file(file, &output, &options) {
            Ok(_) => {
                compiled_count += 1;
                info!("  Compiled: {:?}", file);
            }
            Err(e) => {
                error_count += 1;
                error!("  Error compiling {:?}: {}", file, e);
            }
        }
    }
    
    if error_count > 0 {
        println!("\n⚠️  Build completed with {} errors", error_count);
    } else {
        println!("\n✅ Build successful!");
    }
    println!("   Files compiled: {}", compiled_count);
    println!("   Output: {:?}", output);
    
    if watch {
        info!("Watch mode enabled. Watching for changes...");
        watch_files(source, output)?;
    }
    
    Ok(())
}

fn compile_file(source: &PathBuf, output_dir: &PathBuf, options: &CodegenOptions) -> Result<()> {
    let content = fs::read_to_string(source)?;
    
    let mut parser = Parser::new(&content);
    let program = parser.parse()?;
    
    let mut generator = CodeGenerator::new(options.clone());
    let ts_code = generator.generate(program)?;
    
    let relative_path = source.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let output_file = output_dir.join(format!("{}.ts", relative_path));
    let mut file = fs::File::create(&output_file)?;
    file.write_all(ts_code.as_bytes())?;
    
    Ok(())
}

fn collect_structa_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.is_file() {
        if let Some(ext) = dir.extension() {
            if ext == "structa" || ext == "yaml" || ext == "yml" {
                files.push(dir.clone());
            }
        }
        return Ok(files);
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            files.extend(collect_structa_files(&path)?);
        } else if let Some(ext) = path.extension() {
            if ext == "structa" || ext == "yaml" || ext == "yml" {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}

fn watch_files(_source: PathBuf, _output: PathBuf) -> Result<()> {
    info!("File watching is not yet implemented");
    Ok(())
}

pub fn compile_source(source: &str, options: CodegenOptions) -> Result<String> {
    let mut parser = Parser::new(source);
    let program = parser.parse()?;
    
    let mut generator = CodeGenerator::new(options);
    let ts_code = generator.generate(program)?;
    
    Ok(ts_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_service() {
        let source = "service UserService {}";
        let options = CodegenOptions::default();
        let result = compile_source(source, options);
        match &result {
            Ok(output) => {
                println!("Output: {}", output);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("UserService"));
        assert!(output.contains("@Injectable"));
    }

    #[test]
    fn test_compile_dto() {
        let source = "dto CreateUserDto { name: string }";
        let options = CodegenOptions::default();
        let result = compile_source(source, options);
        match &result {
            Ok(output) => {
                println!("DTO Output: {}", output);
            }
            Err(e) => {
                println!("DTO Error: {:?}", e);
            }
        }
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("CreateUserDto"));
        assert!(output.contains("name"));
    }
}
