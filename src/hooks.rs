use crossterm::style::Stylize;
use glob::glob;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::{File, create_dir_all};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// A constant string representing the file extension pattern for C# project files.
///
/// This constant is typically used to identify or filter files with the `.csproj`
/// extension in a directory or during build/deployment processes.
///
/// # Example
/// ```rust
/// use breathes::hooks::CS_PROJ;
///
/// let file_pattern = CS_PROJ;
/// assert_eq!(file_pattern, "*.csproj");
/// ```
///
/// # Value
/// - `"*.csproj"`: A glob pattern to match any C# project file.
///
/// # Use Case
/// - Filtering `.csproj` files in a filesystem search.
/// - Identifying C# project files for parsing or processing.
///
pub const CS_PROJ: &str = "*.csproj";

///
/// A constant representing the filename of a Maven Project Object Model (POM) file.
///
/// # Description
/// The POM file (typically named `pom.xml`) is an XML file used by Maven,
/// a build automation tool for Java projects. The POM file contains information
/// about the project and configuration details used by Maven to build and manage it.
///
/// This constant can be used in applications or tools that interact with
/// Maven-based projects to refer to the standard POM file name.
///
/// # Example
///
/// ```rust
/// let pom_file = breathes::hooks::MAVEN_POM;
/// println!("The Maven POM file is named: {pom_file}");
/// ```
///
/// # Value
/// - `"pom.xml"`: The default and conventional name for Maven's POM file.
///
/// # Usage
/// This constant is read-only and can be used wherever a reference to
/// the canonical POM file name is needed.
///
/// # See Also
/// - [Maven Project Object Model documentation](https://maven.apache.org/pom.html)
pub const MAVEN_POM: &str = "pom.xml";

///
/// A constant representing the filename of the Cargo.toml file.
///
/// `RUST_FILE` is a string slice that holds the name of the default
/// configuration file used by Rust's package manager, Cargo. This file
/// defines the metadata and dependencies of a Rust project.
///
/// # Example
/// ```rust
/// use breathes::hooks::RUST_FILE;
/// println!("The Cargo configuration file is: {RUST_FILE}");
/// ```
pub const RUST_FILE: &str = "Cargo.toml";

///
/// A constant representing the name of the Go module file.
///
/// This constant is used to identify and reference the standard
/// `go.mod` file in projects that use the Go programming language.
/// The `go.mod` file defines the module properties and manages
/// dependencies for a Go project.
///
/// # Examples
///
/// ```rust
/// use breathes::hooks::GO_FILE;
///
/// assert_eq!(GO_FILE, "go.mod");
/// ```
///
/// # Usage
///
/// This constant can be used in tools or utilities that interact
/// with Go projects to check for the presence of a `go.mod` file
/// or to manipulate it programmatically.
pub const GO_FILE: &str = "go.mod";

/// A constant representing the filename for the Composer JSON file typically used in PHP projects.
///
/// This constant holds the name of the file `composer.json`, which is commonly associated with
/// dependency management and autoloading configurations in PHP development.
///
/// # Example
///
/// ```rust
/// // Usage of the PHP_FILE constant
/// use breathes::hooks::PHP_FILE;
/// println!("The file name is: {PHP_FILE}");
/// ```
/// # Purpose
///
/// This constant can be used in scenarios such as
/// - Identifying the Composer configuration file in file operations
/// - Validating the presence of `composer.json` in a directory
/// - Referring to the file name consistently across a codebase
///
/// # Value
/// `"composer.json"` - The standard filename for Composer's configuration.
pub const PHP_FILE: &str = "composer.json";

///
/// A constant representing the name of the file commonly used to define metadata
/// and dependencies for Node.js projects.
///
/// # Value
/// "package.json"
///
/// # Purpose
/// This constant provides a convenient and reusable identifier for the "package.json" file,
/// which is a standard file in Node.js environments that contains project information,
/// such as the name, version, scripts, dependencies, and other configuration details.
///
/// # Example
/// ```rust
/// use breathes::hooks::NODE_FILE;
/// println!("The Node.js project file is: {NODE_FILE}");
/// ```
pub const NODE_FILE: &str = "package.json";

///
/// A constant representing the default name of the CMake configuration file.
///
/// This file, typically named `CMakeLists.txt`, is used by the CMake build system
/// to define build configurations, targets, dependencies, and other project settings.
///
/// # Examples
///
/// ```rust
/// use breathes::hooks::CMAKE_FILE;
/// println!("CMake configuration file: {CMAKE_FILE}");
/// ```
///
/// This will output:
/// ```text
/// CMake configuration file: CMakeLists.txt
/// ```
pub const CMAKE_FILE: &str = "CMakeLists.txt";

/// A constant representing the filename of the primary configuration file for Elixir projects.
///
/// # Description
/// `ELIXIR_FILE` is a string constant that holds the name of the default configuration file
/// used in Elixir projects, which is `mix.exs`. This file typically contains build configuration,
/// dependencies, and other settings required for the Elixir project's build tool, Mix.
///
/// # Example
/// ```rust
/// use breathes::hooks::ELIXIR_FILE;
/// println!("Elixir configuration file: {ELIXIR_FILE}");
/// ```
///
/// This constant can be useful when working with file system operations or tools that
/// interact with Elixir projects.
///
pub const ELIXIR_FILE: &str = "mix.exs";

/// A constant representing the default filename for a Ruby Gemfile.
///
/// # Description
/// The `RUBY_FILE` constant is used to specify the name of the standard file
/// associated with Ruby projects for managing dependencies. By default, this file
/// is typically named "Gemfile".
///
/// # Usage
/// This constant can be used whenever there is a need to refer to the Ruby Gemfile
/// by its standard name in a program, ensuring consistency and reusability.
///
/// # Example
/// ```
/// use breathes::hooks::RUBY_FILE;
///
/// fn main() {
///     println!("The Ruby dependency file is named: {RUBY_FILE}");
/// }
/// ```
///
pub const RUBY_FILE: &str = "Gemfile";

/// A constant that holds the file name of the `pubspec.yaml` file.
///
/// # Description
/// `pubspec.yaml` is a configuration file used in Dart and Flutter projects.
/// It typically contains metadata about the project (e.g., name, description, version),
/// as well as dependencies and other settings.
///
/// # Example
/// ```rust
/// use breathes::hooks::DART_FILE;
/// let dart_config_file = DART_FILE;
/// println!("The Dart configuration file is: {dart_config_file}");
/// ```
///
/// # Usage
/// This constant can be used wherever the name of the `pubspec.yaml` file
/// is required, reducing hardcoding and the risk of mistakes.
pub const DART_FILE: &str = "pubspec.yaml";

///
/// A constant representing the filename of the Kotlin build script.
///
/// This constant is typically used to identify and reference the `settings.gradle.kts`
/// file in a project. The file is a common entry point for configuring build settings
/// in projects that use Gradle with Kotlin DSL.
///
/// # Example
///
/// ```rust
/// use breathes::hooks::KOTLIN_FILE;
///
/// println!("The Kotlin build script file is: {KOTLIN_FILE}");
/// ```
///
/// # Value
/// - `"settings.gradle.kts"`: The standard name for the settings file written in Kotlin DSL.
pub const KOTLIN_FILE: &str = "settings.gradle.kts";

/// The constant `GRADLE_FILE` represents the default filename
/// for the Gradle settings file in a project.
///
/// This filename is commonly used in Java and Android projects
/// to define project-specific settings for Gradle, such as
/// including subprojects or configuring the build environment.
///
/// # Value
/// - `"settings.gradle"`
///
/// # Example
/// ```rust
/// use breathes::hooks::GRADLE_FILE;
/// let gradle_file = GRADLE_FILE;
/// println!("The Gradle settings file is: {gradle_file}");
/// ```
/// pub
pub const GRADLE_FILE: &str = "settings.gradle";

/// A constant representing the filename of the Swift package manifest.
///
/// # Description
/// `SWIFT_FILE` is a string constant that holds the name of the `Package.swift` file,
/// which is the manifest file used in Swift projects to define package properties,
/// dependencies, and build configurations.
///
/// # Example
/// ```rust
/// use breathes::hooks::SWIFT_FILE;
/// let manifest_file = SWIFT_FILE;
/// assert_eq!(manifest_file, "Package.swift");
/// ```
///
/// # Usage
/// This constant can be used wherever you need to refer to the Swift package manifest file
/// in tools or scripts that interact with Swift projects.
///
/// # Notes
/// - The `Package.swift` file is specific to the Swift Package Manager (SPM).
/// - Ensure that the file exists in the project directory when working with Swift projects.
pub const SWIFT_FILE: &str = "Package.swift";

/// A constant representing the default filename for a Python requirements file.
///
/// This constant is commonly used to reference the `requirements.txt` file, which is
/// a standard file in Python projects for specifying dependencies.
///
/// # Example
/// ```rust
/// use breathes::hooks::PYTHON_FILE;
/// println!("Python requirements file: {PYTHON_FILE}");
/// ```
///
/// The value of this constant is `"requirements.txt"`.
pub const PYTHON_FILE: &str = "requirements.txt";

/// A constant representing the default TypeScript configuration file name.
///
/// This constant specifies the filename of the TypeScript configuration file
/// commonly used in TypeScript projects. It is typically used to reference or
/// locate the "tsconfig.json" file in a project's directory structure.
///
/// # Example
/// ```rust
/// use breathes::hooks::TYPESCRIPT_FILE;
///
/// println!("The TypeScript config file is: {TYPESCRIPT_FILE}");
/// // Output: The TypeScript config file is: tsconfig.json
/// ```
///
/// # Value
/// - `"tsconfig.json"`: The standard filename for TypeScript configuration.
///
/// # Use Cases
/// - Referencing the `tsconfig.json` file in file-management utilities.
/// - Validating or modifying the TypeScript configuration programmatically.
pub const TYPESCRIPT_FILE: &str = "tsconfig.json";

/// A constant that represents the file pattern for Haskell Cabal files.
///
/// This constant is used to match files with the `.cabal` extension,
/// typically associated with Haskell projects that use the Cabal build system.
///
/// # Example
/// ```rust
/// use breathes::hooks::HASKELL_FILE;
/// let file_name = "project.cabal";
/// if file_name.ends_with(HASKELL_FILE.trim_start_matches('*')) {
///     println!("This is a Haskell Cabal file!");
/// }
/// ```
///
/// # Value
/// - `"*.cabal"`: Denotes a glob pattern for identifying Haskell Cabal files.
pub const HASKELL_FILE: &str = "*.cabal";

/// A constant representing the default filename for the Dub configuration file.
///
/// # Description
/// This constant is used to refer to the default name of the primary Dub configuration
/// file, which is `dub.json`. It is commonly utilized in applications that interact
/// with the Dub package manager to configure project settings.
///
/// # Example
/// ```rust
/// use breathes::hooks::D_FILE;
/// let config_file = D_FILE;
/// println!("The Dub configuration file is: {config_file}");
/// ```
///
/// # Usage
/// Use this constant wherever the `dub.json` filename needs to be referenced in
/// a consistent and centralized manner within your codebase.
///
pub const D_FILE: &str = "dub.json";

///
/// An enumeration representing various programming languages.
///
/// This enum is used to categorize and identify different programming
/// languages. It derives several traits, including:
/// - `Clone`: Allows the enum to be cloned.
/// - `Copy`: Allows the enum to be copied rather than moved.
/// - `Debug`: Enables formatting the enum value using the `{:?}` formatter.
/// - `Hash`: Allows the enum to be used as a key in hashed collections, such as `HashMap`.
/// - `Eq` and `PartialEq`: Enables equality and partial equality comparisons.
///
/// # Variants
/// * `Unknown`: Represents an unspecified or unrecognized programming language.
/// * `R`: Represents the R programming language.
/// * `Javascript`: Represents the JavaScript programming language.
/// * `Typescript`: Represents the TypeScript programming language.
/// * `Haskell`: Represents the Haskell programming language.
/// * `D`: Represents the D programming language.
/// * `Rust`: Represents the Rust programming language.
/// * `Python`: Represents the Python programming language.
/// * `Go`: Represents the Go programming language.
/// * `Php`: Represents the PHP programming language.
/// * `Ruby`: Represents the Ruby programming language.
/// * `CMake`: Represents the CMake build scripting language.
/// * `CSharp`: Represents the C# programming language.
/// * `Maven`: Represents Maven, a build automation tool for Java.
/// * `Kotlin`: Represents the Kotlin programming language.
/// * `Gradle`: Represents Gradle, a build automation tool for Java and Android.
/// * `Swift`: Represents the Swift programming language.
/// * `Dart`: Represents the Dart programming language.
/// * `Elixir`: Represents the Elixir programming language.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Language {
    Unknown,
    R,
    Javascript,
    Typescript,
    Haskell,
    D,
    Rust,
    Python,
    Go,
    Php,
    Ruby,
    CMake,
    CSharp,
    Maven,
    Kotlin,
    Gradle,
    Swift,
    Dart,
    Elixir,
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        if value.eq("Javascript") {
            return Self::Javascript;
        }
        if value.eq("Typescript") {
            return Self::Typescript;
        }
        if value.eq("Rust") {
            return Self::Rust;
        }
        if value.eq("Python") {
            return Self::Python;
        }
        if value.eq("Go") {
            return Self::Go;
        }
        if value.eq("Php") {
            return Self::Php;
        }
        if value.eq("Ruby") {
            return Self::Ruby;
        }
        if value.eq("CMake") {
            return Self::CMake;
        }
        if value.eq("CSharp") {
            return Self::CSharp;
        }
        if value.eq("Maven") {
            return Self::Maven;
        }
        if value.eq("Kotlin") {
            return Self::Kotlin;
        }
        if value.eq("Gradle") {
            return Self::Gradle;
        }
        if value.eq("Swift") {
            return Self::Swift;
        }
        if value.eq("Dart") {
            return Self::Dart;
        }
        if value.eq("Elixir") {
            return Self::Elixir;
        }
        if value.eq("D") {
            return Self::D;
        }
        if value.eq("Haskell") {
            return Self::Haskell;
        }
        Self::Unknown
    }
}

impl Language {
    #[must_use]
    pub const fn get_file(language: Self) -> &'static str {
        match language {
            Self::Javascript => NODE_FILE,
            Self::Typescript => TYPESCRIPT_FILE,
            Self::Haskell => HASKELL_FILE,
            Self::Rust => RUST_FILE,
            Self::Python => PYTHON_FILE,
            Self::Go => GO_FILE,
            Self::Php => PHP_FILE,
            Self::Ruby => RUBY_FILE,
            Self::CMake => CMAKE_FILE,
            Self::CSharp => CS_PROJ,
            Self::Maven => MAVEN_POM,
            Self::Kotlin => KOTLIN_FILE,
            Self::Gradle => GRADLE_FILE,
            Self::Swift => SWIFT_FILE,
            Self::Dart => DART_FILE,
            Self::Elixir => ELIXIR_FILE,
            Self::D => D_FILE,
            Self::R | Self::Unknown => "",
        }
    }
}

///
/// A constant array defining a mapping between programming `Language` variants
/// and their corresponding representative file extensions or filenames.
///
/// Each tuple in the array contains:
/// - A `Language` enum variant representing a programming language.
/// - A `&str` literal representing the associated file extension or identifier
///   commonly used for that language (e.g., `.rs` for Rust, `pom.xml` for Maven).
///
/// # Examples
///
/// ```rust
/// use breathes::hooks::LANGUAGES;
/// use breathes::hooks::Language;
///
///
/// ```
///
/// # Array Contents
///
/// - `Language::Rust` => `RUST_FILE`
/// - `Language::Typescript` => `TYPESCRIPT_FILE`
/// - `Language::Haskell` => `HASKELL_FILE`
/// - `Language::Javascript` => `NODE_FILE`
/// - `Language::CSharp` => `CS_PROJ`
/// - `Language::Maven` => `MAVEN_POM`
/// - `Language::Go` => `GO_FILE`
/// - `Language::Ruby` => `RUBY_FILE`
/// - `Language::Dart` => `DART_FILE`
/// - `Language::Gradle` => `GRADLE_FILE`
/// - `Language::Kotlin` => `KOTLIN_FILE`
/// - `Language::Swift` => `SWIFT_FILE`
/// - `Language::Php` => `PHP_FILE`
/// - `Language::CMake` => `CMAKE_FILE`
/// - `Language::Elixir` => `ELIXIR_FILE`
/// - `Language::Python` => `PYTHON_FILE`
///
/// # Notes
///
/// This array can be used for operations such as:
/// - Determining file types for code generation.
/// - Associating files to specific programming languages in a codebase.
/// Ensure to update this constant if new languages or file types are added to
/// the `Language` enum in the future.
pub const LANGUAGES: [(Language, &str); 16] = [
    (Language::Rust, RUST_FILE),
    (Language::Typescript, TYPESCRIPT_FILE),
    (Language::Haskell, HASKELL_FILE),
    (Language::Javascript, NODE_FILE),
    (Language::CSharp, CS_PROJ),
    (Language::Maven, MAVEN_POM),
    (Language::Go, GO_FILE),
    (Language::Ruby, RUBY_FILE),
    (Language::Dart, DART_FILE),
    (Language::Gradle, GRADLE_FILE),
    (Language::Kotlin, KOTLIN_FILE),
    (Language::Swift, SWIFT_FILE),
    (Language::Php, PHP_FILE),
    (Language::CMake, CMAKE_FILE),
    (Language::Elixir, ELIXIR_FILE),
    (Language::Python, PYTHON_FILE),
];
impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Javascript => write!(f, "Javascript"),
            Self::Typescript => write!(f, "Typescript"),
            Self::Rust => write!(f, "Rust"),
            Self::Python => write!(f, "Python"),
            Self::Go => write!(f, "Go"),
            Self::Php => write!(f, "Php"),
            Self::Ruby => write!(f, "Ruby"),
            Self::CMake => write!(f, "CMake"),
            Self::CSharp => write!(f, "CSharp"),
            Self::Maven => write!(f, "Maven"),
            Self::Kotlin => write!(f, "Kotlin"),
            Self::Gradle => write!(f, "Gradle"),
            Self::Swift => write!(f, "Swift"),
            Self::Dart => write!(f, "Dart"),
            Self::Elixir => write!(f, "Elixir"),
            Self::D => write!(f, "D"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Haskell => write!(f, "Haskell"),
            Self::R => write!(f, "R"),
        }
    }
}
#[derive(Clone)]
pub struct Hook {
    pub language: Language,
    pub description: &'static str,
    pub success: &'static str,
    pub failure: &'static str,
    pub file: &'static str,
    pub command: &'static str,
}

impl Hook {
    pub fn d(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::D,
            description: "Building your project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "dub build",
        });
        hooks.push(Self {
            language: Language::D,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "dub test",
        });
    }

    pub fn haskell(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Haskell,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "cabal outdated",
        });
        hooks.push(Self {
            language: Language::Haskell,
            description: "Running tests for your Haskell project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "cabal test",
        });
    }
    pub fn typescript(hooks: &mut Vec<Self>) {
        Self::javascript(hooks);
        hooks.push(Self {
            language: Language::Typescript,
            description: "Checking for types",
            success: "Types are valid",
            failure: "Type errors found",
            file: "types.log",
            command: "npx tsc --noEmit",
        });
        hooks.push(Self {
            language: Language::Typescript,
            description: "Checking for code formatting in your project",
            success: "Code is formatted correctly",
            failure: "Code formatting issues found",
            file: "fmt.log",
            command: "npx prettier --check .",
        });
    }
    pub fn maven(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Maven,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "mvn dependency-check:check",
        });
        hooks.push(Self {
            language: Language::Maven,
            description: "Running tests for your Maven project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "mvn test",
        });
        hooks.push(Self {
            language: Language::Maven,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "mvn versions:display-dependency-updates",
        });
    }
    pub fn gradle(hooks: &mut Vec<Self>) {
        if cfg!(target_os = "windows") {
            hooks.push(Self {
                language: Language::Gradle,
                description: "Building your application",
                success: "Build successful",
                failure: "Build failed",
                file: "build.log",
                command: "gradlew.bat build",
            });
            hooks.push(Self {
                language: Language::Gradle,
                description: "Running unit test",
                success: "Test passed",
                failure: "Test failed",
                file: "test.log",
                command: "gradlew.bat test",
            });
        } else {
            hooks.push(Self {
                language: Language::Gradle,
                description: "Building your application",
                success: "Build successful",
                failure: "Build failed",
                file: "build.log",
                command: "gradlew build",
            });
            hooks.push(Self {
                language: Language::Gradle,
                description: "Running unit test",
                success: "Test passed",
                failure: "Test failed",
                file: "test.log",
                command: "gradlew test",
            });
        }
    }

    pub fn javascript(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Javascript,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "npm outdated",
        });
        hooks.push(Self {
            language: Language::Javascript,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "npm run test",
        });
        hooks.push(Self {
            language: Language::Javascript,
            description: "Auditing your project",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "npm audit",
        });
        hooks.push(Self {
            language: Language::Javascript,
            description: "Checking for code formatting in your project",
            success: "Linting passed",
            failure: "Lint error found",
            file: "lint.log",
            command: "npm run lint",
        });
    }
    pub fn rust(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Rust,
            description: "Checking the configuration",
            success: "Project is valid",
            failure: "Project not valid",
            file: "project.log",
            command: "cargo verify-project",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checking build capability",
            success: "Can build the project",
            failure: "Cargo check detect failure",
            file: "check.log",
            command: "cargo check",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "cargo audit",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checks for formatting issues in your Rust code",
            file: "fmt.log",
            success: "Code format standard respected",
            failure: "Code format standard not respected",
            command: "cargo fmt --check",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checks for linting issues and suggests code improvements",
            success: "No warnings found",
            failure: "Warnings found",
            file: "clippy.log",
            command: "cargo clippy -- -D clippy::all -W warnings -D clippy::pedantic -D clippy::nursery -A clippy::multiple_crate_versions",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "cargo test --no-fail-fast",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Generating documentation for your project",
            success: "Documentation generated",
            failure: "Failed to generate documentation",
            file: "doc.log",
            command: "cargo doc --no-deps --document-private-items",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "cargo outdated",
        });
    }

    pub fn python(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Python,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "pip list --outdated",
        });
        hooks.push(Self {
            language: Language::Python,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "pip audit",
        });
    }
    pub fn go(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Go,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "go test -v",
        });
        hooks.push(Self {
            language: Language::Go,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "go list -u -m -json all",
        });
    }
    pub fn php(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Php,
            description: "Checking platform requirements",
            success: "All requirements are met",
            failure: "Missing requirements found",
            file: "reqs.log",
            command: "composer check-platform-reqs",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "composer audit",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Checking outdated packages",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "composer outdated",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Running tests for your PHP project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "composer run test",
        });
    }

    pub fn ruby(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Ruby,
            description: "Checking for outdated gems",
            success: "No outdated gems found",
            failure: "Outdated gems found",
            file: "outdated.log",
            command: "bundle outdated",
        });
        hooks.push(Self {
            language: Language::Ruby,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "bundle audit",
        });
        hooks.push(Self {
            language: Language::Ruby,
            description: "Running tests for your Ruby project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "bundle exec rspec",
        });
    }
    pub fn cmake(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::CMake,
            description: "Generating build configuration",
            success: "Configuration generated successfully",
            failure: "Configuration failed",
            file: "cmake.log",
            command: "cmake -S . -B build",
        });
        hooks.push(Self {
            language: Language::CMake,
            description: "Compiling the project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "cmake --build build",
        });
        hooks.push(Self {
            language: Language::CMake,
            description: "Running tests",
            success: "All tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "ctest --test-dir build --output-on-failure",
        });
    }
    pub fn csharp(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::CSharp,
            description: "Checking for code formatting",
            success: "Code formatting is correct",
            failure: "Code formatting issues found",
            file: "format.log",
            command: "dotnet format --verify-no-changes",
        });
        hooks.push(Self {
            language: Language::CSharp,
            description: "Running unit tests",
            success: "All tests passed",
            failure: "Some tests failed",
            file: "test.log",
            command: "dotnet test",
        });
        hooks.push(Self {
            language: Language::CSharp,
            description: "Building the project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "dotnet build",
        });
        hooks.push(Self {
            language: Language::CSharp,
            description: "Checking for dependency updates",
            success: "Dependencies are up to date",
            failure: "Dependency updates available",
            file: "deps.log",
            command: "dotnet restore",
        });
        hooks.push(Self {
            language: Language::CSharp,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "dotnet audit",
        });
    }

    pub fn swift(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Swift,
            description: "Checking for code formatting",
            success: "Code formatting is correct",
            failure: "Code formatting issues found",
            file: "format.log",
            command: "swiftformat --lint .",
        });
        hooks.push(Self {
            language: Language::Swift,
            description: "Running unit tests",
            success: "All tests passed",
            failure: "Some tests failed",
            file: "test.log",
            command: "swift test",
        });
        hooks.push(Self {
            language: Language::Swift,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "swift package audit",
        });
        hooks.push(Self {
            language: Language::Swift,
            description: "Building the project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "swift build",
        });
        hooks.push(Self {
            language: Language::Swift,
            description: "Running integration tests",
            success: "All integration tests passed",
            failure: "Some integration tests failed",
            file: "integration.log",
            command: "swift test --parallel",
        });
    }
    pub fn dart(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Dart,
            description: "Checking for code formatting",
            success: "Code formatting is correct",
            failure: "Code formatting issues found",
            file: "format.log",
            command: "dart format --set-exit-if-changed",
        });
        hooks.push(Self {
            language: Language::Dart,
            description: "Running unit tests",
            success: "All tests passed",
            failure: "Some tests failed",
            file: "test.log",
            command: "dart test",
        });
        hooks.push(Self {
            language: Language::Dart,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "dart pub audit",
        });
        hooks.push(Self {
            language: Language::Dart,
            description: "Building the project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "dart compile exe bin/main.dart",
        });
    }
    pub fn kotlin(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Kotlin,
            description: "Running unit tests",
            success: "All tests passed",
            failure: "Some tests failed",
            file: "test.log",
            command: "gradle test",
        });
    }
    pub fn elixir(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Elixir,
            description: "Checking for code formatting",
            success: "Code formatting is correct",
            failure: "Code formatting issues found",
            file: "format.log",
            command: "mix format --check-formatted",
        });
        hooks.push(Self {
            language: Language::Elixir,
            description: "Running unit tests",
            success: "All tests passed",
            failure: "Some tests failed",
            file: "test.log",
            command: "mix test",
        });
        hooks.push(Self {
            language: Language::Elixir,
            description: "Generating documentation",
            success: "Documentation generated successfully",
            failure: "Documentation generation failed",
            file: "docs.log",
            command: "mix docs",
        });
        hooks.push(Self {
            language: Language::Elixir,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "mix audit",
        });
        hooks.push(Self {
            language: Language::Elixir,
            description: "Building the project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "mix compile",
        });
    }
    #[must_use]
    pub fn get(language: Language) -> Vec<Self> {
        let mut hooks: Vec<Self> = vec![];
        match language {
            Language::Unknown | Language::R => {}
            Language::Kotlin => Self::kotlin(&mut hooks),
            Language::Typescript => Self::typescript(&mut hooks),
            Language::D => Self::d(&mut hooks),
            Language::Haskell => Self::haskell(&mut hooks),
            Language::Maven => Self::maven(&mut hooks),
            Language::Gradle => Self::gradle(&mut hooks),
            Language::Javascript => Self::javascript(&mut hooks),
            Language::Rust => Self::rust(&mut hooks),
            Language::Python => Self::python(&mut hooks),
            Language::Go => Self::go(&mut hooks),
            Language::Php => Self::php(&mut hooks),
            Language::Ruby => Self::ruby(&mut hooks),
            Language::CMake => Self::cmake(&mut hooks),
            Language::CSharp => Self::csharp(&mut hooks),
            Language::Swift => Self::swift(&mut hooks),
            Language::Dart => Self::dart(&mut hooks),
            Language::Elixir => Self::elixir(&mut hooks),
        }
        hooks
    }
}
///
/// Executes a set of parallel verification hooks for detected programming languages and provides
/// a summarized view of the results, including their status (success, failure, or error) and execution time.
///
/// # Description
/// The `run_hooks` function performs the following steps:
/// 1. Detects the available programming languages using the `detect` function.
/// 2. If no languages are detected, it returns an error.
/// 3. Initializes a progress bar to visually inform the user about the progress of the hook executions.
/// 4. Runs the verification hooks for each detected language in parallel using `into_par_iter()`.
/// 5. Collects the results of the hook executions, including whether each one succeeded or failed,
///    along with the time taken for execution.
/// 6. Returns a success or failure based on the aggregate status of the hooks.
///
/// # Returns
/// - `Ok(0)` on successful execution when all hooks are successfully verified.
/// - `Err(Error)` if an error occurs or one or more hooks fail.
///
/// # Errors
/// - Returns `Err(Error::other("No language detected"))` if no programming languages are found.
/// # Panics
/// - This function will panic if the provided progress bar style template cannot be set.
pub fn run_hooks() -> Result<i32, Error> {
    let start = Instant::now();
    let l = detect();
    let multi = MultiProgress::new();
    if l.is_empty() {
        return Err(Error::other("No language detected"));
    }
    let pb = ProgressBar::new(l.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{bar:40.white}] {pos}/{len} {msg}",
        )
        .expect("Failed to set progress bar style")
        .progress_chars("#>-"),
    );
    // Informe l'utilisateur que le scan commence
    println!("Running hooks in parallel...");

    // Exécution parallèle : map chaque langage vers son résultat de vérification
    let results: Vec<(Language, Result<(bool, u64), Error>)> = l
        .into_par_iter()
        .map(|lang| {
            let pb_lang = multi.add(ProgressBar::new(Hook::get(lang).len() as u64));
            pb_lang.set_style(
                ProgressStyle::with_template(
                    "{spinner:.white} {prefix:.bold} [{bar:20.white}] {pos}/{len} {msg}",
                )
                .unwrap(),
            );
            pb_lang.set_prefix(lang.to_string());
            pb_lang.set_message(lang.to_string());
            let hooks = Hook::get(lang);
            let res = verify(&hooks, &pb_lang);
            pb_lang.finish_and_clear();
            pb.inc(1);
            (lang, res)
        })
        .collect();
    pb.finish_and_clear();
    let mut global_success = true;

    for (_lang, res) in &results {
        match res {
            Ok((status, _duration)) => {
                if !status {
                    global_success = false;
                }
            }
            Err(_e) => {
                global_success = false;
            }
        }
    }

    let final_status = if global_success { "SUCCESS" } else { "FAILURE" };
    println!("\nOverall Status: {} (Total time: {}s)", final_status, start.elapsed().as_secs());

    if !global_success {
        return Err(Error::other("Checks failed. Check logs in ./breathes/"));
    }
    Ok(0)
}
/// Executes a given command, checks its exit status, and returns an appropriate result.
///
/// # Arguments
///
/// * `cmd` - A mutable reference to a `Command` that represents the command to execute.
/// * `failure` - A string slice that provides an error message in case the command fails.
///
/// # Returns
///
/// * `Ok(())` - If the command executes successfully and returns an exit status of 0.
/// * `Err(Error)` - If the command fails to execute or returns a non-zero exit status. The error
///                  contains the provided failure message.
///
/// # Errors
///
/// This function can return an error under the following circumstances:
///
/// * If the command fails to spawn a child process.
/// * If waiting for the child process to complete fails.
/// * If the command completes execution but returns a non-zero exit status.
///
/// # Example
///
/// ```rust
/// use std::process::Command;
/// use breathes::hooks::ok;
/// use std::io::Error;
/// fn main() -> Result<(), Error> {
///     let mut cmd = Command::new("echo");
///     cmd.arg("Hello, world!");
///
///     ok("Running unit tests", &mut cmd, "Success", "Command execution failed")?;
///     Ok(())
/// }
/// ```
/// ```
pub fn ok(_desc: &str, cmd: &mut Command, _success: &str, failure: &str) -> Result<(), Error> {
    let status = cmd.current_dir(".").spawn()?.wait()?.code();

    if status == Some(0) {
        Ok(())
    } else {
        Err(Error::other(failure))
    }
}

///
/// Verifies the execution of hooks, organizes logs, and provides a summary of results.
///
/// This function takes a slice of `Hook` objects and performs the following operations:
/// - Creates necessary directories to store logs
/// - Executes each hook's command, capturing its standard output and error into separate files
/// - Tracks the success/failure of each hook's execution
/// - Measures the total execution time
///
/// # Arguments
/// - `hooks`: A slice of [`Hook`] objects that represent the commands to be executed
///
/// # Returns
/// Returns a `Result` containing:
/// - A tuple `(bool, u64)`:
///   - `bool`: Indicates whether all hooks were successfully executed (`true`) or if there were any failures (`false`)
///   - `u64`: The total elapsed time in seconds for the verification process
/// - An [`Error`] if any filesystem or execution operation fails
///
/// # Behavior
/// - If the `language` of the first hook is `Language::Unknown`, the function will immediately return `(true, 0)`
///   without further processing.
/// - Creates directories named:
///   - `breathes/<language>/stdout`
///   - `breathes/<language>/stderr`
///   where `<language>` is derived from the `language` field of the first hook.
/// - Executes each hook's command using the appropriate shell:
///   - On Windows: uses `cmd.exe` with the `/C` flag.
///   - On other platforms: uses `sh` with the `-c` flag.
/// - Redirects the standard output and error of each executed command to files located in the respective directories.
/// - Tracks whether each hook executes successfully:
///   - Uses the `ok()` function to determine success or failure for a command execution.
///   - Updates the `status` vector to maintain the results for all hooks.
///
/// # Directories and Files
/// - For each hook, the function generates log files:
///   - `breathes/<language>/stdout/<hook.file>`: Stores standard output of the hook's command
///   - `breathes/<language>/stderr/<hook.file>`: Stores standard error of the hook's command
///
/// # Errors
/// - Fails if:
///   - Directory creation using `create_dir_all` fails
///   - Unable to create log files for standard output or error
///   - Hook command execution fails at any point
///
/// # Example
///
/// ```rust
/// use std::process::Command;
/// use breathes::hooks::verify;
/// use breathes::hooks::Hook;
/// use breathes::hooks::Language;
/// use std::io::Error;
/// use indicatif::ProgressBar;
///
/// fn main() -> Result<(), Error> {
///     let hooks = vec![
///         Hook {
///         language: Language::Rust,
///         description: "Running unit tests",
///         success: "All tests passed",
///         failure: "Some tests failed",
///         file: "test.log",
///         command: "cargo tree",
///     }];
///     let pb = ProgressBar::new(1);
///     let (success, duration) = verify(&hooks, &pb)?;
///     assert!(success);
///     Ok(())
/// }
/// ```
///
/// # Notes
/// - Error handling for the `ok()` function is not explicitly documented here; ensure that the function handles
///   execution failures appropriately and updates `status` correctly.
///
pub fn verify(hooks: &[Hook], pb: &ProgressBar) -> Result<(bool, u64), Error> {
    let start = Instant::now();
    let mut status: Vec<bool> = Vec::new();

    create_dir_all("breathes")?;

    if let Some(first_hook) = hooks.first() {
        let lang = first_hook.language;
        if lang == Language::Unknown {
            return Ok((true, 0));
        }

        // On prépare les chemins proprement une seule fois
        let mut base_path = PathBuf::from("breathes");
        base_path.push(lang.to_string());

        let stdout_dir = base_path.join("stdout");
        let stderr_dir = base_path.join("stderr");

        // Création des dossiers
        create_dir_all(&stdout_dir)?;
        create_dir_all(&stderr_dir)?;

        for hook in hooks {
            if hook.language == Language::Unknown {
                continue;
            }
            pb.set_message(hook.description);
            // On construit le chemin du fichier de log final
            let out_file = stdout_dir.join(hook.file);
            let err_file = stderr_dir.join(hook.file);

            let mut cmd = if cfg!(target_os = "windows") {
                let mut c = Command::new("cmd");
                c.arg("/C").arg(hook.command);
                c
            } else {
                let mut c = Command::new("sh");
                c.arg("-c").arg(hook.command);
                c
            };

            // Configuration commune de la commande
            cmd.current_dir(".")
                .stdout(File::create(out_file)?)
                .stderr(File::create(err_file)?);

            // On exécute
            let result = ok(hook.description, &mut cmd, hook.success, hook.failure);

            match result {
                Ok(_) => pb.println(format!("  {} {}", "✓".green(), hook.description)),
                Err(_) => pb.println(format!("  {} {}", "!".red(), hook.description)),
            }
            pb.inc(1);
            if result.is_err() {
                status.push(false);
            } else {
                status.push(true);
            }
        }
    }
    Ok((!status.contains(&false), start.elapsed().as_secs()))
}

/// Adds the specified `language` to the given vector `vec` if certain file conditions are met.
///
/// # Parameters
///
/// * `file`: A string representing a file path or a glob pattern to match files.
/// * `language`: An enumeration value of type `Language` representing the programming language to be added.
/// * `vec`: A mutable reference to a `Vec<Language>` where the language may be appended.
///
/// # Behavior
///
/// - For `language` being `Language::CSharp` or `Language::Haskell`:
///   - It attempts to resolve the `file` parameter as a glob pattern using the `glob` function.
///   - If the result is successful, it iterates over the resolved file paths.
///   - For each file path in the list, if the file exists and is a regular file, the function adds `language` to `vec`.
///
/// - For other languages:
///   - It checks if the provided `file` is a regular file (using `Path::new(file).is_file()`).
///   - If the file exists and is a regular file, the function adds `language` to `vec`.
///
/// # Notes
///
/// - The `glob` function must be implemented or available via an external crate for matching file patterns.
/// - The functionality uses pattern matching and guards to validate file system paths.
/// - The function does nothing if the specified `file` or glob pattern does not match any regular file.
///
/// # Example
///
/// ```rust
/// use breathes::hooks::add_if_exists;
/// use breathes::hooks::Language;
/// let mut languages = vec![];
/// add_if_exists("*.cs", Language::CSharp, &mut languages);
/// add_if_exists("Main.hs", Language::Haskell, &mut languages);
/// assert_eq!(languages.len(), 0);
/// ```
///
pub fn add_if_exists(file: &str, language: Language, vec: &mut Vec<Language>) {
    if language == Language::CSharp
        && let Ok(files) = glob(file)
    {
        for file in files {
            if let Ok(file) = file
                && file.is_file()
            {
                vec.push(language);
            }
        }
    } else if language == Language::Haskell
        && let Ok(files) = glob(file)
    {
        for file in files {
            if let Ok(file) = file
                && file.is_file()
            {
                vec.push(language);
            }
        }
    } else if Path::new(file).is_file() {
        vec.push(language);
    }
}

/// Detects and returns a list of programming languages based on predefined criteria.
///
/// This function iterates through a collection of predefined programming languages and their
/// associated file information (`LANGUAGES`). For each entry, it checks if the corresponding
/// files exist, and if they do, adds the language to the resulting list.
///
/// # Returns
/// A `Vec<Language>` containing all programming languages that were detected based on
/// the existence of their associated files.
///
/// # Attributes
/// * `#[must_use]` - Indicates that the return value of this function should not be ignored
///   as it provides meaningful information.
///
/// # Example
/// ```
/// use breathes::hooks::detect;
/// let detected_languages = detect();
/// for lang in &detected_languages {
///     println!("{lang}");
/// }
/// ```
///
/// # Notes
/// - This function depends on the global `LANGUAGES` collection, which maps programming
///   languages to their associated file data.
/// - The `add_if_exists` function is responsible for determining if a language is added
///   to the result based on file existence.
///
/// # See Also
/// - `add_if_exists` function for further details on how languages are added.
///
#[must_use]
pub fn detect() -> Vec<Language> {
    let mut all: Vec<Language> = Vec::new();
    for (l, file) in &LANGUAGES {
        add_if_exists(file, *l, &mut all);
    }
    all
}
