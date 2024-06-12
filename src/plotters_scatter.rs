use plotters::prelude::*;

pub fn scatter_plot(
    data: Vec<Vec<f64>>,
    file_name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // 描画領域を指定
    let drawing_area = BitMapBackend::new(
        file_name,
        (1080, 720)
    ).into_drawing_area();

    drawing_area.fill(&WHITE)?;

    // チャートの初期化
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("The Datasaurus", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    // グリッド線とラベルの描画
    chart.configure_mesh().draw()?;

    // データポイントのプロット
    chart.draw_series(
        data.iter().map(|point| {
            Circle::new(
                (point[0], point[1]),
                5,
                ShapeStyle::from(&RED).filled()
            )
        })
    )?;

    // 画像ファイルへの書き込み
    drawing_area.present()?;

    Ok(())
}