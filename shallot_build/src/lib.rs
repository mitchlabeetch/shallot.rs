use std::path::PathBuf;
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub name: String,
    pub version: String,
    pub targets: Vec<BuildTarget>,
    pub features: Vec<String>,
    pub optimization: OptimizationLevel,
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BuildTarget {
    WebAssembly,
    NativeDesktop,
    ServerSideRendering,
    StaticSite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Development,
    Release,
    SizeOptimized,
    SpeedOptimized,
}

impl BuildTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            BuildTarget::WebAssembly => "wasm",
            BuildTarget::NativeDesktop => "native",
            BuildTarget::ServerSideRendering => "ssr",
            BuildTarget::StaticSite => "static",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BuildTarget::WebAssembly => "WebAssembly build for browsers",
            BuildTarget::NativeDesktop => "Native desktop application",
            BuildTarget::ServerSideRendering => "Server-side rendering build",
            BuildTarget::StaticSite => "Static site generation",
        }
    }
}

impl OptimizationLevel {
    pub fn cargo_profile(&self) -> &'static str {
        match self {
            OptimizationLevel::Development => "dev",
            OptimizationLevel::Release => "release",
            OptimizationLevel::SizeOptimized => "release",
            OptimizationLevel::SpeedOptimized => "release",
        }
    }

    pub fn rustc_flags(&self) -> Vec<&'static str> {
        match self {
            OptimizationLevel::Development => vec![],
            OptimizationLevel::Release => vec!["-C", "opt-level=3"],
            OptimizationLevel::SizeOptimized => vec!["-C", "opt-level=z", "-C", "lto=fat"],
            OptimizationLevel::SpeedOptimized => vec!["-C", "opt-level=3", "-C", "lto=thin"],
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildSystem {
    config: BuildConfig,
}

impl BuildSystem {
    pub fn new(config: BuildConfig) -> Self {
        Self { config }
    }

    pub fn build_all(&self) -> Result<Vec<BuildResult>, BuildError> {
        let mut results = Vec::new();

        for target in &self.config.targets {
            match self.build_target(*target) {
                Ok(result) => results.push(result),
                Err(error) => return Err(error),
            }
        }

        Ok(results)
    }

    pub fn build_target(&self, target: BuildTarget) -> Result<BuildResult, BuildError> {
        println!("Building for target: {}", target.description());

        match target {
            BuildTarget::WebAssembly => self.build_wasm(),
            BuildTarget::NativeDesktop => self.build_native(),
            BuildTarget::ServerSideRendering => self.build_ssr(),
            BuildTarget::StaticSite => self.build_static(),
        }
    }

    fn build_wasm(&self) -> Result<BuildResult, BuildError> {
        // Ensure wasm-pack is installed
        self.check_wasm_pack()?;

        let output_dir = self.config.output_dir.join("wasm");
        fs::create_dir_all(&output_dir)?;

        let mut cmd = Command::new("wasm-pack");
        cmd.arg("build")
            .arg("--target")
            .arg("web")
            .arg("--out-dir")
            .arg(&output_dir)
            .arg("--name")
            .arg(&self.config.name);

        match self.config.optimization {
            OptimizationLevel::Development => {
                cmd.arg("--dev");
            }
            OptimizationLevel::Release |
            OptimizationLevel::SizeOptimized |
            OptimizationLevel::SpeedOptimized => {
                cmd.arg("--release");
            }
        }

        if !self.config.features.is_empty() {
            cmd.arg("--features")
                .arg(self.config.features.join(","));
        }

        let output = cmd.output()?;

        if !output.status.success() {
            return Err(BuildError::WasmBuildFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(BuildResult {
            target: BuildTarget::WebAssembly,
            output_dir: output_dir.clone(),
            artifacts: self.find_wasm_artifacts(&output_dir)?,
            size: self.calculate_directory_size(&output_dir)?,
            optimization: self.config.optimization,
        })
    }

    fn build_native(&self) -> Result<BuildResult, BuildError> {
        let output_dir = self.config.output_dir.join("native");
        fs::create_dir_all(&output_dir)?;

        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target-dir")
            .arg(&output_dir);

        match self.config.optimization {
            OptimizationLevel::Development => {
                // Default is dev build
            }
            OptimizationLevel::Release |
            OptimizationLevel::SizeOptimized |
            OptimizationLevel::SpeedOptimized => {
                cmd.arg("--release");
            }
        }

        if !self.config.features.is_empty() {
            cmd.arg("--features")
                .arg(self.config.features.join(" "));
        }

        // Add optimization flags
        for flag in self.config.optimization.rustc_flags() {
            cmd.env("RUSTFLAGS", format!("{} {}", 
                std::env::var("RUSTFLAGS").unwrap_or_default(),
                flag
            ));
        }

        let output = cmd.output()?;

        if !output.status.success() {
            return Err(BuildError::NativeBuildFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(BuildResult {
            target: BuildTarget::NativeDesktop,
            output_dir: output_dir.clone(),
            artifacts: self.find_native_artifacts(&output_dir)?,
            size: self.calculate_directory_size(&output_dir)?,
            optimization: self.config.optimization,
        })
    }

    fn build_ssr(&self) -> Result<BuildResult, BuildError> {
        let output_dir = self.config.output_dir.join("ssr");
        fs::create_dir_all(&output_dir)?;

        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target-dir")
            .arg(&output_dir);

        match self.config.optimization {
            OptimizationLevel::Development => {
                // Default is dev build
            }
            OptimizationLevel::Release |
            OptimizationLevel::SizeOptimized |
            OptimizationLevel::SpeedOptimized => {
                cmd.arg("--release");
            }
        }

        // Add SSR-specific features
        let mut features = self.config.features.clone();
        features.push("ssr".to_string());
        cmd.arg("--features")
            .arg(features.join(" "));

        let output = cmd.output()?;

        if !output.status.success() {
            return Err(BuildError::SsrBuildFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(BuildResult {
            target: BuildTarget::ServerSideRendering,
            output_dir: output_dir.clone(),
            artifacts: self.find_ssr_artifacts(&output_dir)?,
            size: self.calculate_directory_size(&output_dir)?,
            optimization: self.config.optimization,
        })
    }

    fn build_static(&self) -> Result<BuildResult, BuildError> {
        let output_dir = self.config.output_dir.join("static");
        fs::create_dir_all(&output_dir)?;

        // Generate static HTML, CSS, and JS files
        self.generate_static_files(&output_dir)?;

        Ok(BuildResult {
            target: BuildTarget::StaticSite,
            output_dir: output_dir.clone(),
            artifacts: self.find_static_artifacts(&output_dir)?,
            size: self.calculate_directory_size(&output_dir)?,
            optimization: self.config.optimization,
        })
    }

    fn check_wasm_pack(&self) -> Result<(), BuildError> {
        let output = Command::new("wasm-pack")
            .arg("--version")
            .output()?;

        if !output.status.success() {
            return Err(BuildError::MissingDependency(
                "wasm-pack not found. Install with: cargo install wasm-pack".to_string()
            ));
        }

        Ok(())
    }

    fn find_wasm_artifacts(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, BuildError> {
        let mut artifacts = Vec::new();
        
        if dir.join("pkg").exists() {
            artifacts.push(dir.join("pkg"));
        }

        Ok(artifacts)
    }

    fn find_native_artifacts(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, BuildError> {
        let mut artifacts = Vec::new();
        
        // Look for executable files
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let metadata = fs::metadata(&path)?;
                    let permissions = metadata.permissions();
                    
                    if permissions.mode() & 0o111 != 0 {
                        artifacts.push(path);
                    }
                }
                
                #[cfg(windows)]
                {
                    if path.extension().map_or(false, |ext| ext == "exe") {
                        artifacts.push(path);
                    }
                }
            }
        }

        Ok(artifacts)
    }

    fn find_ssr_artifacts(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, BuildError> {
        // SSR artifacts are similar to native but with specific naming
        self.find_native_artifacts(dir)
    }

    fn find_static_artifacts(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, BuildError> {
        let mut artifacts = Vec::new();
        
        // Look for HTML, CSS, and JS files
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    match ext.to_str() {
                        Some("html") | Some("css") | Some("js") | Some("wasm") => {
                            artifacts.push(path);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(artifacts)
    }

    fn generate_static_files(&self, output_dir: &PathBuf) -> Result<(), BuildError> {
        // Generate index.html
        let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <h1>{} Component Library</h1>
        <p>Static site generated successfully!</p>
    </div>
    <script src="app.js"></script>
</body>
</html>"#, self.config.name, self.config.name);

        fs::write(output_dir.join("index.html"), index_html)?;

        // Generate styles.css
        let styles_css = r#"
:root {
    --primary-color: #8b5cf6;
    --secondary-color: #ec4899;
    --background-color: #ffffff;
    --text-color: #1a1a1a;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    margin: 0;
    padding: 2rem;
    background-color: var(--background-color);
    color: var(--text-color);
    line-height: 1.6;
}

#app {
    max-width: 800px;
    margin: 0 auto;
    text-align: center;
}

h1 {
    color: var(--primary-color);
    font-size: 2.5rem;
    margin-bottom: 1rem;
}
"#;

        fs::write(output_dir.join("styles.css"), styles_css)?;

        // Generate app.js
        let app_js = r#"
console.log('Shallot.rs Component Library loaded successfully!');
"#;

        fs::write(output_dir.join("app.js"), app_js)?;

        Ok(())
    }

    fn calculate_directory_size(&self, dir: &PathBuf) -> Result<u64, BuildError> {
        let mut total_size = 0;
        
        for entry in walkdir::WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                total_size += entry.metadata()?.len();
            }
        }

        Ok(total_size)
    }

    pub fn generate_build_report(&self, results: &[BuildResult]) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("Build Report for {} v{}\n", self.config.name, self.config.version));
        report.push_str(&"=".repeat(50));
        report.push('\n');
        
        for result in results {
            report.push_str(&format!(
                "Target: {}\n",
                result.target.description()
            ));
            report.push_str(&format!(
                "Output Directory: {}\n",
                result.output_dir.display()
            ));
            report.push_str(&format!(
                "Optimization: {:?}\n",
                result.optimization
            ));
            report.push_str(&format!(
                "Total Size: {:.2} MB\n",
                result.size as f64 / 1_048_576.0
            ));
            
            if !result.artifacts.is_empty() {
                report.push_str("Artifacts:\n");
                for artifact in &result.artifacts {
                    report.push_str(&format!("  - {}\n", artifact.display()));
                }
            }
            
            report.push('\n');
        }
        
        report
    }
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub target: BuildTarget,
    pub output_dir: PathBuf,
    pub artifacts: Vec<PathBuf>,
    pub size: u64,
    pub optimization: OptimizationLevel,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    
    #[error("WebAssembly build failed: {0}")]
    WasmBuildFailed(String),
    
    #[error("Native build failed: {0}")]
    NativeBuildFailed(String),
    
    #[error("SSR build failed: {0}")]
    SsrBuildFailed(String),
    
    #[error("Static build failed: {0}")]
    StaticBuildFailed(String),
    
    #[error("Walk directory error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

// Default configurations for different scenarios
impl BuildConfig {
    pub fn development_default(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            targets: vec![BuildTarget::WebAssembly, BuildTarget::ServerSideRendering],
            features: vec!["dev".to_string()],
            optimization: OptimizationLevel::Development,
            output_dir: PathBuf::from("target/build"),
        }
    }

    pub fn production_default(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            targets: vec![
                BuildTarget::WebAssembly,
                BuildTarget::NativeDesktop,
                BuildTarget::ServerSideRendering,
                BuildTarget::StaticSite,
            ],
            features: vec!["prod".to_string()],
            optimization: OptimizationLevel::Release,
            output_dir: PathBuf::from("dist"),
        }
    }

    pub fn size_optimized(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            targets: vec![BuildTarget::WebAssembly],
            features: vec!["size-opt".to_string()],
            optimization: OptimizationLevel::SizeOptimized,
            output_dir: PathBuf::from("dist/minimal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config_creation() {
        let config = BuildConfig::development_default("test", "1.0.0");
        assert_eq!(config.name, "test");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.targets.len(), 2);
    }

    #[test]
    fn test_build_target_descriptions() {
        assert_eq!(BuildTarget::WebAssembly.description(), "WebAssembly build for browsers");
        assert_eq!(BuildTarget::NativeDesktop.description(), "Native desktop application");
    }

    #[test]
    fn test_optimization_level_flags() {
        let flags = OptimizationLevel::SizeOptimized.rustc_flags();
        assert!(flags.contains(&"-C"));
        assert!(flags.contains(&"opt-level=z"));
    }
}