fn main() {
    println!("tui_dashboard: TUIの状態管理と描画を学ぶプロジェクト");
    println!("next: ratatui + crossterm + app state loop");
}

#[cfg(test)]
mod tests {
    #[test]
    fn dashboard_keyword_exists() {
        let name = "tui_dashboard";
        assert!(name.ends_with("dashboard"));
    }
}
