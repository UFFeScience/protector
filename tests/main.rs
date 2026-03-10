use assert_cmd::Command;

#[test]
fn all_instances_can_be_run_without_errors() {
    let folders = std::fs::read_dir("instances").unwrap();

    let instances = folders.map(|instance| instance.unwrap().file_name());

    for instance in instances {
        Command::cargo_bin("main")
            .unwrap()
            .arg(format!("instances/{}/graph", instance.to_str().unwrap()))
            .arg("--executions=1")
            .arg("--iterations=1")
            .arg("--metaheuristic=grasp")
            .assert()
            .success();
    }
}
