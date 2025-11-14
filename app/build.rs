fn main() {
    // Compile the Slint UI. The main UI file is named `mine_window.slint` in this project.
    slint_build::compile("slint-ui/mine_window.slint").unwrap();
}
