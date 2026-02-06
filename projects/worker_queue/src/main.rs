fn main() {
    println!("worker_queue: 非同期ジョブ処理と再試行制御を学ぶプロジェクト");
    println!("next: tokio task + channel + retry policy");
}

#[cfg(test)]
mod tests {
    #[test]
    fn queue_name_is_stable() {
        let project = "worker_queue";
        assert_eq!(project.split('_').count(), 2);
    }
}
