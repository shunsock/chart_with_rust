use polars::prelude::*;

pub fn datasaurus(file_path: &str) -> Result<Vec<Vec<f64>>> {
    // CSV ファイルを読み込み DataFrame にする
    let df: DataFrame = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;

    // 'dataset' column が 'dino' である行のみに行を絞る
    let dino_df: DataFrame = df.filter(&df.column("dataset")?.equal("dino")?)?;

    // x， y columns を Vec<Vec<f64>> にキャストしてreturnする
    let x_col: &Float64Chunked = dino_df.column("x")?.f64()?;
    let y_col: &Float64Chunked = dino_df.column("y")?.f64()?;

    let mut result: Vec<Vec<f64>> = Vec::new();
    for (x, y) in x_col.into_iter().zip(y_col.into_iter()) {
        if let (Some(x), Some(y)) = (x, y) {
            result.push(vec![x, y]);
        }
    }

    Ok(result)
}