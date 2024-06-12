use charming::{
    component::{
    title::Title,
    axis::Axis,
    },
    element::{TextStyle, Padding},
    series::Scatter,
    theme::Theme,
    Chart,
    ImageRenderer,
    ImageFormat,
};

pub fn scatter_plot(
    data: Vec<Vec<f64>>,
    output_path: &str
)
{
    let chart = Chart::new()
        .title(
            Title::new()
                .text("The Datasaurus")
                .text_style(
                    TextStyle::new()
                        .font_size(32)
                        .font_weight("bold")
                )
                .padding(Padding::Double(10.0, 100.0))
        )
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .series(
            Scatter::new()
                .symbol_size(16)
                .data(data)
        );

    ImageRenderer::new(1080, 720)
        .theme(Theme::PurplePassion)
        .save_format(
            ImageFormat::Png,
            &chart,
            output_path
        ).expect(
        "レンダリングに失敗しました"
    );
}