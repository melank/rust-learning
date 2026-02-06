fn main() {
    println!("cli_automation: 業務自動化CLIを作るプロジェクト");
    println!("next: clap + file io + structured log");
}

#[cfg(test)]
mod tests {
    #[test]
    fn command_label_is_non_empty() {
        let label = "cli_automation";
        assert!(!label.is_empty());
    }
}
