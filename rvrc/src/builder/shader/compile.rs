use crate::{
    Error,
    builder::shader::CompileError,
};
use shaderc::{ResolvedInclude, IncludeType, ShaderKind};
use tinypath::Path;
use std::fs::File;
use memmap::MmapOptions;

fn include(include_path: &str, source_path: &str) -> Result<ResolvedInclude, Error> {
    let include_path = Path::from_str(include_path)?;
    let source_path = Path::from_str(source_path)?;

    let resolved_name = include_path.relative_to(&source_path).to_platform_string();

    let file = File::open(&resolved_name)?;
    let map = unsafe { MmapOptions::new().map(&file)? };

    let content = std::str::from_utf8(&map)?.to_string();

    Ok(ResolvedInclude {
        resolved_name,
        content,
    })
}

pub fn compile(entry_point: &str, source: &str, file_name: &str, shader_kind: ShaderKind) -> Result<Vec<u32>, Error> {
    let mut compiler = shaderc::Compiler::new().ok_or(CompileError::Initialization)?;
    let mut options = shaderc::CompileOptions::new().ok_or(CompileError::Initialization)?;

    options.set_include_callback(|include_path, include_type, source_path, _| {
        Ok(include(include_path, source_path).map_err(|error| error.to_string())?)
    });

    let result = compiler.compile_into_spirv(
        source,
        shader_kind,
        file_name,
        entry_point,
        Some(&options),
    )?;

    Ok(result.as_binary().iter().cloned().collect())
}
