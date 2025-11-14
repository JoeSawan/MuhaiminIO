// Re-export Slint-generated UI module so other code can use `commands::MainWindow`.
slint::include_modules!();

// The Slint file is `mine_window.slint` and the generated module is `mine_window`.
// Re-export its MainWindow type under the `commands` module name. Use a
// local path (`self::`) to refer to the module created inside this module
// (the macro expands here), which avoids ambiguity during compile.
pub use self::mine_window::MainWindow;
