# Valence nested source diff for Steel projectile milestone repair

Purpose: make nested repo commit `2663ed7` reviewable from parent evidence scope.

Command:

```sh
git -C valence show --format=fuller --stat --patch 2663ed7 -- examples/ctf.rs
```

Output:

```diff
commit 2663ed7d6c948d86dd0ab35f368376b406e32c99
Author:     brittonr <b@robitzs.ch>
AuthorDate: Thu May 28 01:09:30 2026 -0400
Commit:     brittonr <b@robitzs.ch>
CommitDate: Thu May 28 01:09:30 2026 -0400

    bind projectile milestones to damage evidence
---
 examples/ctf.rs | 6 ++++--
 1 file changed, 4 insertions(+), 2 deletions(-)

diff --git a/examples/ctf.rs b/examples/ctf.rs
index 5a784e1..531ec4d 100644
--- a/examples/ctf.rs
+++ b/examples/ctf.rs
@@ -1858,10 +1858,11 @@ fn handle_combat_events(
                 .expect("projectile probe hit has arrow decision");
             let projectile_use = format!(
                 "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} item={:?} \
-                 policy={} generation={} clamped={}",
+                 damage={:.1} policy={} generation={} clamped={}",
                 attacker.username.as_str(),
                 victim.username.as_str(),
                 stack.item,
+                damage,
                 decision.policy_id,
                 decision.generation,
                 decision.clamped
@@ -2024,11 +2025,12 @@ fn handle_projectile_events(
         info!("{}", milestone);
         println!("{}", milestone);
         let hit = format!(
-            "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} \
+            "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
              victim_health_before={:.1} victim_health_after={:.1} policy={} generation={} \
              clamped={}",
             attacker_name,
             victim_username.as_str(),
+            decision.damage,
             before,
             victim_health.0,
             decision.policy_id,
```
