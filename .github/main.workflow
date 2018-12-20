workflow "Check and test" {
  on = "push"
  resolves = ["Check", "Test"]
}

action "Check" {
  uses = "docker://rust:1.31.0"
  runs = "cargo check"
}

action "Test" {
  uses = "docker://rust:1.31.0"
  needs = ["Check"]
  runs = "cargo"
  args = "test"
}
