# Nickel-authored defaults for the local Minecraft compatibility smoke runner.
# Export with:
#   nix shell nixpkgs#nickel -c nickel export config/mc-compat/default.ncl > config/mc-compat/generated/default.json

let Port = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 1 && value <= 65535
) in
let PositiveSeconds = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 1
) in
let Protocol = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 0
) in
let Backend = std.contract.from_predicate (fun value =>
  value == "valence" || value == "paper"
) in
let SmokeConfig = {
  client_dir | String,
  valence_repo | String,
  valence_rev | String,
  valence_worktree | String,
  valence_example | String,
  valence_log | String,
  valence_target_dir | String,
  valence_pid_file | String,
  server_backend | Backend,
  target_dir | String,
  server_name | String,
  server_version | String,
  server_protocol | Protocol,
  server_port | Port | optional,
  client_username | String,
  docker_image | String,
  client_timeout_secs | PositiveSeconds,
  client_success_patterns | Array String,
  receipt_path | String | optional,
  receipt_dir | String | optional,
} in
{
  client_dir = "./stevenarella",
  valence_repo = "./valence",
  valence_rev = "8ad9c85",
  valence_worktree = "/tmp/valence-compat-758",
  valence_example = "terrain",
  valence_log = "/tmp/mc-compat-valence.log",
  valence_target_dir = "/tmp/valence-compat-758-target",
  valence_pid_file = "/tmp/mc-compat-valence.pid",
  server_backend = "valence",
  target_dir = "/tmp/stevenarella-target2",
  server_name = "mc-compat-1-18-2",
  server_version = "1.18.2",
  server_protocol = 758,
  client_username = "compatbot",
  docker_image = "itzg/minecraft-server:java17",
  client_timeout_secs = 20,
  client_success_patterns = [
    "Detected server protocol version",
    "Dimension type:",
    "Received chat message",
  ],
} | SmokeConfig
