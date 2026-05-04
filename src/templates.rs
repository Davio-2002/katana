pub fn cmake_lists(name: &str, version: u8) -> String {
    format!(
        r#"cmake_minimum_required(VERSION 3.20)
project({name} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD {version})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS src/*.cpp)
add_executable(${{PROJECT_NAME}} ${{SOURCES}})

target_include_directories(${{PROJECT_NAME}} PUBLIC ${{CMAKE_SOURCE_DIR}}/include)

target_compile_options(${{PROJECT_NAME}} PRIVATE
    $<$<CXX_COMPILER_ID:GNU,Clang>:-Wall -Wextra -Wpedantic>
)

execute_process(
    COMMAND ${{CMAKE_COMMAND}} -E copy_if_different
        "${{CMAKE_BINARY_DIR}}/compile_commands.json"
        "${{CMAKE_SOURCE_DIR}}/compile_commands.json"
    ERROR_QUIET
)
add_custom_target(sync_compile_commands ALL
    COMMAND ${{CMAKE_COMMAND}} -E copy_if_different
        "${{CMAKE_BINARY_DIR}}/compile_commands.json"
        "${{CMAKE_SOURCE_DIR}}/compile_commands.json"
    VERBATIM
)
"#
    )
}

pub fn clangd(version: u8) -> String {
    format!(
        r#"CompileFlags:
  Add:
    - "-std=c++{version}"
    - "-Wall"
    - "-Wextra"
    - "-I${{workspaceFolder}}/include"

Diagnostics:
  ClangTidy:
    Add:
      - modernize-*
      - performance-*
      - readability-*
      - bugprone-*
      - clang-analyzer-*
    Remove:
      - modernize-use-trailing-return-type

Index:
  Background: Build

InlayHints:
  Enabled: Yes
  ParameterNames: Yes
  DeducedTypes: Yes
"#
    )
}

pub fn zed_settings() -> String {
    r#"{
  "language_servers": ["clangd"],
  "languages": {
    "C++": {
      "language_servers": ["clangd"],
      "format_on_save": "on",
      "formatter": { "external": { "command": "clang-format", "arguments": ["-style=file", "-"] } },
      "tab_size": 5
    }
  },
  "lsp": {
    "clangd": {
      "binary": {
        "path": "clangd",
        "arguments": [
          "--background-index", "--clang-tidy", "--completion-style=detailed",
          "--header-insertion=iwyu", "--suggest-missing-includes",
          "--all-scopes-completion", "--pch-storage=memory",
          "--limit-results=100", "--offset-encoding=utf-16",
          "--compile-commands-dir=."
        ]
      }
    }
  },
  "inlay_hints": { "enabled": true, "show_type_hints": true, "show_parameter_hints": true }
}"#
    .to_string()
}

pub fn zed_tasks(name: &str, watchexec: bool) -> String {
    let watch_cmd = if watchexec {
        "watchexec -e cpp,h,hpp,cmake -w src/ -w include/ -w CMakeLists.txt -- bash -c 'cmake --preset debug -Wno-dev && cmake --build --preset debug -- -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)'".to_string()
    } else {
        "echo 'watchexec not installed. Run: cargo install watchexec-cli'".to_string()
    };

    format!(
        r#"[
  {{ "label": "⛩️  Configure (Debug)",    "command": "cmake --preset debug -Wno-dev",                        "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }},
  {{ "label": "⚔️  Build (Debug)",         "command": "cmake --build --preset debug -- -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)", "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }},
  {{ "label": "▶️  Run (Debug)",            "command": "./build/debug/{name}",                                "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }},
  {{ "label": "🐉 Build & Run (Debug)",    "command": "cmake --preset debug -Wno-dev && cmake --build --preset debug -- -j$(nproc 2>/dev/null || sysctl -n hw.ncpu) && ./build/debug/{name}", "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }},
  {{ "label": "👁️  Watch & Auto-build",    "command": "{watch_cmd}",                                         "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }},
  {{ "label": "🌸 Clean",                  "command": "rm -rf build/",                                       "cwd": "$ZED_WORKTREE_ROOT", "reveal": "always" }}
]"#
    )
}

pub fn cmake_presets() -> String {
    r#"{
  "version": 3,
  "configurePresets": [
    {
      "name": "debug",
      "displayName": "Debug",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build/debug",
      "cacheVariables": { "CMAKE_BUILD_TYPE": "Debug" }
    },
    {
      "name": "release",
      "displayName": "Release",
      "generator": "Ninja",
      "binaryDir": "${sourceDir}/build/release",
      "cacheVariables": { "CMAKE_BUILD_TYPE": "Release" }
    }
  ],
  "buildPresets": [
    { "name": "debug",   "configurePreset": "debug"   },
    { "name": "release", "configurePreset": "release" }
  ]
}"#
    .to_string()
}

pub fn clang_format() -> &'static str {
    "BasedOnStyle: LLVM\nIndentWidth: 4\nColumnLimit: 100\nAllowShortFunctionsOnASingleLine: Inline\nAllowShortIfStatementsOnASingleLine: Never\nBreakBeforeBraces: Attach\nSortIncludes: CaseSensitive\nIncludeBlocks: Regroup\n"
}

pub fn clang_tidy() -> &'static str {
    "Checks: >\n  clang-diagnostic-*,\n  clang-analyzer-*,\n  modernize-*,\n  performance-*,\n  readability-*,\n  bugprone-*,\n  -modernize-use-trailing-return-type\nHeaderFilterRegex: \"include/.*\"\n"
}

pub fn gitignore() -> &'static str {
    "build/\n.cache/\n*.o *.a *.so *.out\ncompile_commands.json\n.DS_Store\n"
}

pub fn main_cpp(name: &str) -> String {
    format!(
        "#include <iostream>\n\nint main() {{\n    std::cout << \"Hello from {name}!\\n\";\n    return 0;\n}}\n"
    )
}

pub fn example_h() -> &'static str {
    "#pragma once\n\n// Headers in include/ are available as:\n//   #include <example.h>\n"
}

pub fn readme(name: &str, version: u8) -> String {
    format!(
        "# {name}\n\n> Generated by **katana** 🗡 — C++{version}\n\n## Build\n\n```bash\ncmake --preset debug\ncmake --build --preset debug\n./build/debug/{name}\n```\n"
    )
}
