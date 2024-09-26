use editor::editor_gui::EditorGUI;
use iced::Font;

fn main() {
    let _ = iced::application("Editor - RRT", EditorGUI::update, EditorGUI::view)
        .default_font(Font::MONOSPACE)
        .centered()
        .run_with(EditorGUI::new);
}
