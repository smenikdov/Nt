local path_utils = require("vdelosnap.utils.path")

return {
    config = {
        mac_window_bar = false,
        save_path = "~/Pictures",
        has_breadcrumbs = true,
        bg_theme = "grape",
        show_workspace = false,
        breadcrumbs_separator = "/",
        has_line_number = true,
        bg_x_padding = 122,
        bg_y_padding = 82,
        bg_padding = nil,
        min_width = 0,
        watermark = "VDELO.DEV",
        watermark_font_family = "JetBrains Mono",
        code_font_family = "JetBrains Mono",
        title = "VDELO.DEV",
    },

    cwd = path_utils.back(path_utils.back(debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])"))),
    preview_switch = true,
}
