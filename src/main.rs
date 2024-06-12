use crate::datasaurus::datasaurus;

mod charming_scatter;
mod datasaurus;
mod plotters_scatter;

fn main() {
    // データの読み込み
    let dino_data: Vec<Vec<f64>> = datasaurus(
        "data/datasaurus.csv"
    ).expect(
        "データの読み込みに失敗しました"
    );

    charming_scatter::scatter_plot(
        dino_data.clone(),
        "output/datasaurus_charming.png"
    );

    plotters_scatter::scatter_plot(
        dino_data.clone(),
        "output/datasaurus_plotters.png"
    ).expect(
        "プロットに失敗しました"
    );
}
