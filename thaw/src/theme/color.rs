use thaw_macro::WriteCSSVars;

#[derive(Clone, WriteCSSVars)]
pub struct ColorTheme {
    pub color_neutral_background_disabled: String,
    pub color_neutral_background_1: String,
    pub color_neutral_background_1_hover: String,
    pub color_neutral_background_1_pressed: String,
    
    pub color_neutral_foreground_disabled: String,
    pub color_neutral_foreground_1: String,
    pub color_neutral_foreground_1_hover: String,
    pub color_neutral_foreground_1_pressed: String,
    pub color_neutral_foreground_2: String,
    pub color_neutral_foreground_2_hover: String,
    pub color_neutral_foreground_2_pressed: String,
    pub color_neutral_foreground_2_brand_hover: String,
    pub color_neutral_foreground_2_brand_pressed: String,
    pub color_neutral_foreground_on_brand: String,

    pub color_neutral_stroke_disabled: String,
    pub color_neutral_stroke_1: String,
    pub color_neutral_stroke_1_hover: String,
    pub color_neutral_stroke_1_pressed: String,
    pub color_neutral_stroke_accessible: String,

    pub color_compound_brand_stroke: String,

    pub color_brand_background: String,
    pub color_brand_background_hover: String,
    pub color_brand_background_pressed: String,
    pub color_subtle_background: String,
    pub color_subtle_background_hover: String,
    pub color_subtle_background_pressed: String,
    pub color_transparent_background: String,
    pub color_transparent_background_hover: String,
    pub color_transparent_background_pressed: String,
}

impl ColorTheme {
    pub fn light() -> Self {
        Self {
            color_neutral_background_disabled: "#f0f0f0".into(),
            color_neutral_background_1: "#fff".into(),
            color_neutral_background_1_hover: "#f5f5f5".into(),
            color_neutral_background_1_pressed: "#e0e0e0".into(),
            color_neutral_foreground_disabled: "#bdbdbd".into(),
            color_neutral_foreground_1: "#242424".into(),
            color_neutral_foreground_1_hover: "#242424".into(),
            color_neutral_foreground_1_pressed: "#242424".into(),
            color_neutral_foreground_2: "#424242".into(),
            color_neutral_foreground_2_hover: "#242424".into(),
            color_neutral_foreground_2_pressed: "#242424".into(),
            color_neutral_foreground_2_brand_hover: "#0f6cbd".into(),
            color_neutral_foreground_2_brand_pressed: "#115ea3".into(),
            color_neutral_foreground_on_brand: "#fff".into(),

            color_neutral_stroke_disabled: "#e0e0e0".into(),
            color_neutral_stroke_1: "#d1d1d1".into(),
            color_neutral_stroke_1_hover: "#c7c7c7".into(),
            color_neutral_stroke_1_pressed: "#b3b3b3".into(),
            color_neutral_stroke_accessible: "#616161".into(),

            color_compound_brand_stroke: "#0f6cbd".into(),

            color_brand_background: "#0f6cbd".into(),
            color_brand_background_hover: "#115ea3".into(),
            color_brand_background_pressed: "#0c3b5e".into(),
            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#f5f5f5".into(),
            color_subtle_background_pressed: "#e0e0e0".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            color_neutral_background_disabled: "#141414".into(),
            color_neutral_background_1: "#292929".into(),
            color_neutral_background_1_hover: "#3d3d3d".into(),
            color_neutral_background_1_pressed: "#1f1f1f".into(),
            color_neutral_foreground_disabled: "#5c5c5c".into(),
            color_neutral_foreground_1: "#fff".into(),
            color_neutral_foreground_1_hover: "#fff".into(),
            color_neutral_foreground_1_pressed: "#fff".into(),
            color_neutral_foreground_2: "#d6d6d6".into(),
            color_neutral_foreground_2_hover: "#fff".into(),
            color_neutral_foreground_2_pressed: "#fff".into(),
            color_neutral_foreground_2_brand_hover: "#479ef5".into(),
            color_neutral_foreground_2_brand_pressed: "#2886de".into(),
            color_neutral_foreground_on_brand: "#fff".into(),

            color_neutral_stroke_disabled: "#424242".into(),
            color_neutral_stroke_1: "#666666".into(),
            color_neutral_stroke_1_hover: "#757575".into(),
            color_neutral_stroke_1_pressed: "#6b6b6b".into(),
            color_neutral_stroke_accessible: "#adadad".into(),

            color_compound_brand_stroke: "#479ef5".into(),

            color_brand_background: "#115ea3".into(),
            color_brand_background_hover: "#0f6cbd".into(),
            color_brand_background_pressed: "#0c3b5e".into(),
            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#383838".into(),
            color_subtle_background_pressed: "#2e2e2e".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
        }
    }
}