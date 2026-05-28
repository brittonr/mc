;; mc-compat Steel runtime config v1
;; Sandbox profile: mc-compat/pure-v1
;; Host capabilities: pure arithmetic and explicit context access only.

(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")

;; Runner/server values.
(define server-backend "valence")
(define server-version "1.20.1")
(define server-protocol 763)
(define server-port 25565)
(define valence-rev "main")
(define valence-example "ctf")
(define valence-worktree "/tmp/valence-compat-763")
(define valence-target-dir "/tmp/valence-compat-763-target")
(define valence-log "/tmp/mc-compat-valence.log")
(define valence-pid-file "/tmp/mc-compat-valence.pid")

;; Stevenarella client values.
(define client-username "compatbot")
(define client-timeout-secs 120)
(define client-success-patterns
  (list "Detected server protocol version"
        "Dimension type:"
        "Received chat message"))

;; Evidence/runtime outputs.
(define receipt-dir "target/mc-compat-steel")
(define scenario "projectile-damage-attribution")

;; Representative hotloaded gameplay rule.
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)

(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
