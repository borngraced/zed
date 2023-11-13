use crate::{PlayerColors, SyntaxTheme};
use gpui::Hsla;
use refineable::Refineable;
use std::sync::Arc;

#[derive(Clone)]
pub struct SystemColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug, serde::Deserialize)]
pub struct StatusColors {
    /// Indicates some kind of conflict, like a file changed on disk while it was open, or
    /// merge conflicts in a Git repository.
    pub conflict: Hsla,

    /// Indicates something new, like a new file added to a Git repository.
    pub created: Hsla,

    /// Indicates that something no longer exists, like a deleted file.
    pub deleted: Hsla,

    /// Indicates a system error, a failed operation or a diagnostic error.
    pub error: Hsla,

    /// Represents a hidden status, such as a file being hidden in a file tree.
    pub hidden: Hsla,

    /// Indicates a hint or some kind of additional information.
    pub hint: Hsla,

    /// Indicates that something is deliberately ignored, such as a file or operation ignored by Git.
    pub ignored: Hsla,

    /// Represents informational status updates or messages.
    pub info: Hsla,

    /// Indicates a changed or altered status, like a file that has been edited.
    pub modified: Hsla,

    /// Indicates something that is predicted, like automatic code completion, or generated code.
    pub predictive: Hsla,

    /// Represents a renamed status, such as a file that has been renamed.
    pub renamed: Hsla,

    /// Indicates a successful operation or task completion.
    pub success: Hsla,

    /// Indicates some kind of unreachable status, like a block of code that can never be reached.
    pub unreachable: Hsla,

    /// Represents a warning status, like an operation that is about to fail.
    pub warning: Hsla,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug, serde::Deserialize)]
pub struct ThemeColors {
    pub border: Hsla,
    /// Border color. Used for deemphasized borders, like a visual divider between two sections
    pub border_variant: Hsla,
    /// Border color. Used for focused elements, like keyboard focused list item.
    pub border_focused: Hsla,
    /// Border color. Used for selected elements, like an active search filter or selected checkbox.
    pub border_selected: Hsla,
    /// Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change.
    pub border_transparent: Hsla,
    /// Border color. Used for disabled elements, like a disabled input or button.
    pub border_disabled: Hsla,
    /// Border color. Used for elevated surfaces, like a context menu, popup, or dialog.
    pub elevated_surface_background: Hsla,
    /// Background Color. Used for grounded surfaces like a panel or tab.
    pub surface_background: Hsla,
    /// Background Color. Used for the app background and blank panels or windows.
    pub background: Hsla,
    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    pub element_background: Hsla,
    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,
    /// Background Color. Used for the active state of an element that should have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    pub element_active: Hsla,
    /// Background Color. Used for the selected state of an element that should have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    pub element_selected: Hsla,
    /// Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    pub element_disabled: Hsla,
    /// Background Color. Used for the area that shows where a dragged element will be dropped.
    pub drop_target_background: Hsla,
    /// Border Color. Used to show the area that shows where a dragged element will be dropped.
    // pub drop_target_border: Hsla,
    /// Used for the background of a ghost element that should have the same background as the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have a different background than the surface it's on, use `element_background`.
    pub ghost_element_background: Hsla,
    /// Background Color. Used for the hover state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub ghost_element_hover: Hsla,
    /// Background Color. Used for the active state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    pub ghost_element_active: Hsla,
    /// Background Color. Used for the selected state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    pub ghost_element_selected: Hsla,
    /// Background Color. Used for the disabled state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    pub ghost_element_disabled: Hsla,
    /// Text Color. Default text color used for most text.
    pub text: Hsla,
    /// Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.
    pub text_muted: Hsla,
    /// Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data.
    pub text_placeholder: Hsla,
    /// Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state.
    pub text_disabled: Hsla,
    /// Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search.
    pub text_accent: Hsla,
    /// Fill Color. Used for the default fill color of an icon.
    pub icon: Hsla,
    /// Fill Color. Used for the muted or deemphasized fill color of an icon.
    ///
    /// This might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight.
    pub icon_muted: Hsla,
    /// Fill Color. Used for the disabled fill color of an icon.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a icon button.
    pub icon_disabled: Hsla,
    /// Fill Color. Used for the placeholder fill color of an icon.
    ///
    /// This might be used to show an icon in an input that disappears when the user enters text.
    pub icon_placeholder: Hsla,
    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    pub icon_accent: Hsla,

    // ===
    // UI Elements
    // ===
    pub status_bar_background: Hsla,
    pub title_bar_background: Hsla,
    pub toolbar_background: Hsla,
    pub tab_bar_background: Hsla,
    pub tab_inactive_background: Hsla,
    pub tab_active_background: Hsla,
    // pub panel_background: Hsla,
    // pub pane_focused_border: Hsla,
    // /// The color of the scrollbar thumb.
    // pub scrollbar_thumb_background: Hsla,
    // /// The color of the scrollbar thumb when hovered over.
    // pub scrollbar_thumb_hover_background: Hsla,
    // /// The border color of the scrollbar thumb.
    // pub scrollbar_thumb_border: Hsla,
    // /// The background color of the scrollbar track.
    // pub scrollbar_track_background: Hsla,
    // /// The border color of the scrollbar track.
    // pub scrollbar_track_border: Hsla,
    // /// The opacity of the scrollbar status marks, like diagnostic states and git status..
    // pub scrollbar_status_opacity: Hsla,

    // ===
    // Editor
    // ===
    pub editor_background: Hsla,
    // pub editor_inactive_background: Hsla,
    pub editor_gutter_background: Hsla,
    pub editor_subheader_background: Hsla,
    pub editor_active_line_background: Hsla,
    pub editor_highlighted_line_background: Hsla,
    /// Text Color. Used for the text of the line number in the editor gutter.
    pub editor_line_number: Hsla,
    /// Text Color. Used for the text of the line number in the editor gutter when the line is highlighted.
    pub editor_active_line_number: Hsla,
    /// Text Color. Used to mark invisible characters in the editor.
    ///
    /// Example: spaces, tabs, carriage returns, etc.
    pub editor_invisible: Hsla,
    pub editor_wrap_guide: Hsla,
    pub editor_active_wrap_guide: Hsla,
    pub editor_document_highlight_read_background: Hsla,
    pub editor_document_highlight_write_background: Hsla,

    // ===
    // Terminal
    // ===
    /// Terminal Background Color
    pub terminal_background: Hsla,
    /// Bright Black Color for ANSI Terminal
    pub terminal_ansi_bright_black: Hsla,
    /// Bright Red Color for ANSI Terminal
    pub terminal_ansi_bright_red: Hsla,
    /// Bright Green Color for ANSI Terminal
    pub terminal_ansi_bright_green: Hsla,
    /// Bright Yellow Color for ANSI Terminal
    pub terminal_ansi_bright_yellow: Hsla,
    /// Bright Blue Color for ANSI Terminal
    pub terminal_ansi_bright_blue: Hsla,
    /// Bright Magenta Color for ANSI Terminal
    pub terminal_ansi_bright_magenta: Hsla,
    /// Bright Cyan Color for ANSI Terminal
    pub terminal_ansi_bright_cyan: Hsla,
    /// Bright White Color for ANSI Terminal
    pub terminal_ansi_bright_white: Hsla,
    /// Black Color for ANSI Terminal
    pub terminal_ansi_black: Hsla,
    /// Red Color for ANSI Terminal
    pub terminal_ansi_red: Hsla,
    /// Green Color for ANSI Terminal
    pub terminal_ansi_green: Hsla,
    /// Yellow Color for ANSI Terminal
    pub terminal_ansi_yellow: Hsla,
    /// Blue Color for ANSI Terminal
    pub terminal_ansi_blue: Hsla,
    /// Magenta Color for ANSI Terminal
    pub terminal_ansi_magenta: Hsla,
    /// Cyan Color for ANSI Terminal
    pub terminal_ansi_cyan: Hsla,
    /// White Color for ANSI Terminal
    pub terminal_ansi_white: Hsla,
    // new colors

    // ===
    // Elevation
    // ===
    // elevation_0_shadow
    // elevation_0_shadow_color
    // elevation_1_shadow
    // elevation_1_shadow_color
    // elevation_2_shadow
    // elevation_2_shadow_color
    // elevation_3_shadow
    // elevation_3_shadow_color
    // elevation_4_shadow
    // elevation_4_shadow_color
    // elevation_5_shadow
    // elevation_5_shadow_color

    // ===
    // UI Text
    // ===
    // pub headline: Hsla,
    // pub paragraph: Hsla,
    // pub link: Hsla,
    // pub link_hover: Hsla,
    // pub code_block_background: Hsla,
    // pub code_block_border: Hsla,
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    pub system: SystemColors,

    #[refineable]
    pub colors: ThemeColors,
    pub status: StatusColors,
    pub player: PlayerColors,
    pub syntax: Arc<SyntaxTheme>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn override_a_single_theme_color() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui::rgb(0xff00ff);

        assert_ne!(colors.text, magenta);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
    }

    #[test]
    fn override_multiple_theme_colors() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui::rgb(0xff00ff);
        let green: Hsla = gpui::rgb(0x00ff00);

        assert_ne!(colors.text, magenta);
        assert_ne!(colors.background, green);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            background: Some(green),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
        assert_eq!(colors.background, green);
    }

    #[test]
    fn deserialize_theme_colors_refinement_from_json() {
        let colors: ThemeColorsRefinement = serde_json::from_value(json!({
            "background": "#ff00ff",
            "text": "#ff0000"
        }))
        .unwrap();

        assert_eq!(colors.background, Some(gpui::rgb(0xff00ff)));
        assert_eq!(colors.text, Some(gpui::rgb(0xff0000)));
    }
}
