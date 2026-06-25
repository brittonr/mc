;; Ralph Octet enforcement loop for the mc workspace.
;;
;; This script is intentionally not Cairn-aware. Default mode prints a
;; deterministic non-Cairn prompt/plan for enforcing the repo-pinned Octet lint
;; set across owned Rust workspaces. --run launches the configured Pi command
;; (default: pi -p) once with the generated prompt.

(require "steel/result")
(require-builtin steel/process)

(define default-root ".")
(define default-agent-command "pi -p")
(define default-session-name "mc Octet monorepo lint enforcement")
(define default-run-prompt-file "target/ralph-octet.prompt.run.txt")
(define command-line-arg-drop-count 2)
(define failing-status-probe 7)
(define process-cd-script "cd \"$1\" && shift && exec \"$@\"")
(define process-prompt-file-script "cd \"$1\" && shift && prompt_file=\"$1\" && shift && exec \"$@\" \"$(cat \"$prompt_file\")\"")
(define process-shell-name "sh")
(define path-separator "/")
(define run-step-count 5)
(define owned-workspaces
  (list
   "compat/runner"
   "clients/stevenarella"
   "servers/valence"))
(define progress-phases
  (list
   "1/7 inventory workspaces, Octet metadata, config, lint inventory, and existing findings"
   "2/7 run baseline Octet checks and classify pre-existing failures"
   "3/7 implement one smallest scoped fix or config slice"
   "4/7 rerun the smallest relevant Octet gate for that slice"
   "5/7 add or update the aggregate monorepo Octet gate"
   "6/7 add positive and negative validation for drift, missing config, and new findings"
   "7/7 run final validation, commit locally, and report blockers"))

(struct Config
  (root agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?))

(define (say value)
  (display value)
  (newline))

(define (string-prefix? prefix text)
  (let ([prefix-length (string-length prefix)] [text-length (string-length text)])
    (and (<= prefix-length text-length)
         (string=? prefix (substring text 0 prefix-length)))))

(define (space? value)
  (char=? value #\space))

(define (split-words text)
  (let ([text-length (string-length text)])
    (let loop ([index 0] [token-start #false] [tokens '()])
      (cond
        [(= index text-length)
         (reverse
          (if token-start
              (cons (substring text token-start text-length) tokens)
              tokens))]
        [(space? (string-ref text index))
         (if token-start
             (loop (+ index 1) #false (cons (substring text token-start index) tokens))
             (loop (+ index 1) #false tokens))]
        [else
         (if token-start
             (loop (+ index 1) token-start tokens)
             (loop (+ index 1) index tokens))]))))

(define (ends-with-slash? text)
  (let ([text-length (string-length text)])
    (and (> text-length 0) (char=? (string-ref text (- text-length 1)) #\/))))

(define (path-join left right)
  (if (ends-with-slash? left)
      (string-append left right)
      (string-append left path-separator right)))

(define (join-lines lines)
  (if (null? lines)
      ""
      (string-append (car lines) "\n" (join-lines (cdr lines)))))

(define (usage-lines)
  (list
   "Usage: steel scripts/ralph-octet-loop.scm [options]"
   ""
   "Options:"
   "  --root PATH        mc repo root (default: .)"
   "  --agent COMMAND    Pi command used by --run (default: pi -p)"
   "  --name NAME        Pi session name used by --run"
   "  --run              Write the prompt file, then launch Pi with that prompt"
   "                     Run mode prints deterministic progress logs before and after Pi"
   "                     If Pi needs foreground TTY control, rerun the printed fallback command directly"
   "  --print-prompt     Print only the raw prompt for shell redirection or command substitution"
   "  --prompt-file PATH Write only the raw prompt to PATH; with --run, use PATH as the run prompt file"
   "  --no-prompt        Omit the prompt block in plan mode"
   "  --no-commands      Omit shell command examples in plan mode"
   "  --self-test        Run positive and negative script self-tests"
   "  --help             Show this help"
   ""
   "Examples:"
   "  steel scripts/ralph-octet-loop.scm"
   "  steel scripts/ralph-octet-loop.scm --prompt-file target/ralph-octet.prompt.txt"
   "  pi -p --name \"mc Octet monorepo lint enforcement\" \"$(cat target/ralph-octet.prompt.txt)\""
   "  steel scripts/ralph-octet-loop.scm --run"
   "  steel scripts/ralph-octet-loop.scm --run --agent \"pi -p --model sonnet:high\""))

(define (parse-args raw-args)
  (let loop ([args raw-args]
             [root default-root]
             [agent-command default-agent-command]
             [session-name default-session-name]
             [emit-prompt? #true]
             [emit-commands? #true]
             [run? #false]
             [print-prompt? #false]
             [prompt-file #false]
             [self-test? #false]
             [help? #false])
    (cond
      [(null? args)
       (Config root agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--root")
       (if (null? (cdr args))
           (error "--root requires a path")
           (loop (cddr args) (cadr args) agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?))]
      [(string-prefix? "--root=" (car args))
       (loop (cdr args) (substring (car args) (string-length "--root=") (string-length (car args))) agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--agent")
       (if (null? (cdr args))
           (error "--agent requires a command")
           (loop (cddr args) root (cadr args) session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?))]
      [(string-prefix? "--agent=" (car args))
       (loop (cdr args) root (substring (car args) (string-length "--agent=") (string-length (car args))) session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--name")
       (if (null? (cdr args))
           (error "--name requires a session name")
           (loop (cddr args) root agent-command (cadr args) emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?))]
      [(string-prefix? "--name=" (car args))
       (loop (cdr args) root agent-command (substring (car args) (string-length "--name=") (string-length (car args))) emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--run")
       (loop (cdr args) root agent-command session-name emit-prompt? emit-commands? #true print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--print-prompt")
       (loop (cdr args) root agent-command session-name emit-prompt? emit-commands? run? #true prompt-file self-test? help?)]
      [(string=? (car args) "--prompt-file")
       (if (null? (cdr args))
           (error "--prompt-file requires a path")
           (loop (cddr args) root agent-command session-name emit-prompt? emit-commands? run? print-prompt? (cadr args) self-test? help?))]
      [(string-prefix? "--prompt-file=" (car args))
       (loop (cdr args) root agent-command session-name emit-prompt? emit-commands? run? print-prompt? (substring (car args) (string-length "--prompt-file=") (string-length (car args))) self-test? help?)]
      [(string=? (car args) "--no-prompt")
       (loop (cdr args) root agent-command session-name #false emit-commands? run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--no-commands")
       (loop (cdr args) root agent-command session-name emit-prompt? #false run? print-prompt? prompt-file self-test? help?)]
      [(string=? (car args) "--self-test")
       (loop (cdr args) root agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file #true help?)]
      [(string=? (car args) "--help")
       (loop (cdr args) root agent-command session-name emit-prompt? emit-commands? run? print-prompt? prompt-file self-test? #true)]
      [else (error (string-append "unknown argument: " (car args)))])))

(define (workspace-lines)
  (map (lambda (workspace) (string-append "- `" workspace "`")) owned-workspaces))

(define (progress-phase-lines)
  (map (lambda (phase) (string-append "- " phase)) progress-phases))

(define (log-step index label)
  (say (string-append "[ralph-octet-loop] step " (number->string index) "/" (number->string run-step-count) ": " label)))

(define (log-detail label value)
  (say (string-append "[ralph-octet-loop]   " label ": " value)))

(define (octet-prompt-lines config)
  (append
   (list
    "Goal: enforce every lint from the repo-pinned Octet input across the owned Rust workspaces in this monorepo."
    ""
    "Scope:"
    "- Include owned mc Rust workspaces:")
   (workspace-lines)
   (list
    "- Treat `hyperion/` as an independent nested repo unless explicitly selected."
    "- Treat `Leafish/` as reference-only and exclude it from default gates."
    "- Derive the lint set from the pinned Octet tool/config, not from a hard-coded list."
    ""
    "Success criteria:"
    "- Every included workspace has Octet runner metadata and consumer-owned `dylint.toml`."
    "- Every current Octet lint is enabled at deny level for the enforced scope, unless an exception is explicitly documented with owner, rationale, affected crates, and removal condition."
    "- A repo-level Nix check or script runs all included Octet gates."
    "- Existing findings are fixed or captured in a reviewed baseline that fails on new unaccepted findings."
    "- Positive and negative validation prove the gate catches lint drift, missing config, and new findings."
    "- Final validation passes and a why-focused commit is created. Do not push unless explicitly asked."
    ""
    "Constraints:"
    "- Do not create, sync, archive, or edit Cairn changes for this work."
    "- Preserve public behavior and compatibility non-claims."
    "- Use repo-pinned `.#cargo-octet` / `.#octet`; do not use an unpinned sibling checkout for final evidence."
    "- Run baseline Octet checks before edits and record pre-existing failures."
    "- Fix lints by improving code quality, not by broad `allow`, `disabled_lints`, or threshold loosening."
    "- Any suppression must be narrow, justified, owned, and tested."
    "- Use functional-core / imperative-shell decomposition and Tiger Style."
    "- Stop and report blockers for toolchain failures, nested-repo boundary conflicts, excessive generated-code churn, or findings requiring product decisions."
    ""
    "Immediate references to read:"
    "- `AGENTS.md`"
    "- `README.md`"
    "- `docs/check-tiers.md`"
    "- `docs/configuration.md`"
    "- `servers/valence/AGENTS.md` and `servers/valence/dylint.toml`"
    "- `clients/stevenarella/AGENTS.md`"
    "- Octet consumer docs from the pinned input or local sibling if needed: `README.md`, `docs/consumer-guide.md`, `docs/lint-reference.md`, and `templates/dylint.toml`"
    ""
    "Loop:")
   (progress-phase-lines)
   (list
    ""
    "Progress logging requirements:"
    "- Start each major update with `PROGRESS n/7 <phase>: <state>`."
    "- Include current workspace, command/check being run, finding counts when known, artifact/log path, and next action."
    "- For long checks, use pueue, report the task id immediately, then report status/log-tail updates until complete."
    "- When blocked, report `BLOCKED <phase>` with the failing command, exit status, relevant log path, and the smallest next decision needed."
    "- Final report must include completed scopes, checks run, aggregate gate status, commit if created, and remaining blockers."
    ""
    "Do not wait for permission between safe non-destructive checks. Execute, inspect, fix, and continue until complete or genuinely blocked.")))

(define (octet-prompt config)
  (join-lines (octet-prompt-lines config)))

(define (write-text-file! path text)
  (call-with-output-file path (lambda (out) (display text out))))

(define (write-prompt-file! config)
  (write-text-file! (Config-prompt-file config) (octet-prompt config))
  (say (string-append "[ralph-octet-loop] wrote prompt_file: " (Config-prompt-file config))))

(define (run-prompt-file config)
  (if (Config-prompt-file config)
      (Config-prompt-file config)
      default-run-prompt-file))

(define (manual-pi-command config prompt-file)
  (string-append (Config-agent-command config)
                 " --name \"" (Config-session-name config) "\" \"$(cat " prompt-file ")\""))

(define (command-lines config)
  (let ([root (Config-root config)]
        [target-dir (path-join (Config-root config) "target/octet")])
    (list
     "```sh"
     (string-append "cd " root)
     "MC_ROOT=$(pwd)"
     "nix run .#cargo-octet -- --help"
     (string-append "mkdir -p " target-dir)
     "(cd \"$MC_ROOT/servers/valence\" && nix run \"path:$MC_ROOT#cargo-octet\" -- check --output-format json --artifact-dir \"$MC_ROOT/target/octet/valence\")"
     "(cd \"$MC_ROOT/compat/runner\" && nix run \"path:$MC_ROOT#cargo-octet\" -- check --output-format json --artifact-dir \"$MC_ROOT/target/octet/compat-runner\")"
     "(cd \"$MC_ROOT/clients/stevenarella\" && nix run \"path:$MC_ROOT#cargo-octet\" -- check --output-format json --artifact-dir \"$MC_ROOT/target/octet/stevenarella\")"
     "# After the aggregate check exists:"
     "nix build .#checks.x86_64-linux.mc-octet-monorepo --no-link -L"
     "```")))

(define (plan-lines config)
  (append
   (list
    "# mc Ralph Octet Enforcement Loop"
    ""
    (string-append "Root: `" (Config-root config) "`")
    (string-append "Agent command: `" (Config-agent-command config) "`")
    (string-append "Session name: `" (Config-session-name config) "`")
    "Cairn lifecycle: disabled"
    ""
    "Default mode only prints this plan. Re-run with `--run` to launch the configured Pi command once with the generated prompt."
    ""
    "## Owned Rust workspaces")
   (workspace-lines)
   (append (list "" "## Progress phases") (progress-phase-lines))
   (if (Config-emit-commands? config)
       (append (list "" "## Useful command skeleton") (command-lines config))
       '())
   (if (Config-emit-prompt? config)
       (append (list "" "## Ralph prompt" "```text") (octet-prompt-lines config) (list "```"))
       '())))

(define (external-command-status root program args)
  (unwrap-ok
   (process-wait
    (unwrap-ok
     (spawn-process
      (command process-shell-name
               (append
                (list "-c" process-cd-script process-shell-name root program)
                args)))))))

(define (external-prompt-file-command-status root prompt-file program args)
  (unwrap-ok
   (process-wait
    (unwrap-ok
     (spawn-process
      (command process-shell-name
               (append
                (list "-c" process-prompt-file-script process-shell-name root prompt-file program)
                args)))))))

(define (agent-command-parts config)
  (let ([parts (split-words (Config-agent-command config))])
    (if (null? parts)
        (error "--agent must not be empty")
        parts)))

(define (run-pi! config)
  (log-step 1 "parse agent command")
  (let* ([parts (agent-command-parts config)]
         [program (car parts)]
         [base-args (cdr parts)])
    (log-detail "root" (Config-root config))
    (log-detail "session" (Config-session-name config))
    (log-detail "program" program)
    (log-detail "base_arg_count" (number->string (length base-args)))
    (log-step 2 "render and write prompt file")
    (let* ([prompt (octet-prompt config)]
           [prompt-file (run-prompt-file config)]
           [args (append base-args (list "--name" (Config-session-name config)))])
      (write-text-file! prompt-file prompt)
      (log-detail "prompt_file" prompt-file)
      (log-detail "prompt_characters" (number->string (string-length prompt)))
      (log-detail "owned_workspace_count" (number->string (length owned-workspaces)))
      (log-detail "progress_phase_count" (number->string (length progress-phases)))
      (log-step 3 "prepare pi invocation")
      (log-detail "argv_without_prompt_count" (number->string (length args)))
      (log-detail "cairn_lifecycle" "disabled")
      (log-detail "fallback_command" (manual-pi-command config prompt-file))
      (log-step 4 "launch pi from prompt file")
      (say "[ralph-octet-loop]   note: no more script progress appears until the child Pi process exits")
      (say "[ralph-octet-loop]   note: if fish reports the Pi job stopped or this waits on TTY control, press Ctrl-C and run the fallback_command directly from the shell")
      (let ([status (external-prompt-file-command-status (Config-root config) prompt-file program args)])
        (log-step 5 "collect exit status")
        (log-detail "exit_status" (number->string status))
        (if (= status 0)
            (say "[ralph-octet-loop] completed")
            (error (string-append "pi command failed with exit " (number->string status))))))))

(define (print-lines lines)
  (for-each say lines))

(define (check label condition)
  (if condition
      (say (string-append "ok - " label))
      (error (string-append "self-test failed: " label))))

(define (run-self-test)
  (let* ([sample-config (Config "/tmp/mc" default-agent-command default-session-name #true #true #false #false #false #false #false)]
         [sample-plan (plan-lines sample-config)]
         [sample-prompt (octet-prompt sample-config)])
    (check "prefix positive" (string-prefix? "--root=" "--root=/tmp/mc"))
    (check "prefix negative" (not (string-prefix? "--root=" "--roo=/tmp/mc")))
    (check "split agent command" (equal? (split-words "pi -p --model sonnet:high") (list "pi" "-p" "--model" "sonnet:high")))
    (check "parse root" (string=? (Config-root (parse-args (list "--root" "/tmp/mc"))) "/tmp/mc"))
    (check "parse run" (Config-run? (parse-args (list "--run"))))
    (check "parse print prompt" (Config-print-prompt? (parse-args (list "--print-prompt"))))
    (check "parse prompt file" (string=? (Config-prompt-file (parse-args (list "--prompt-file" "target/prompt.txt"))) "target/prompt.txt"))
    (check "plan disables cairn lifecycle" (member "Cairn lifecycle: disabled" sample-plan))
    (check "plan includes valence workspace" (member "- `servers/valence`" sample-plan))
    (check "plan includes progress phases" (member "## Progress phases" sample-plan))
    (check "prompt forbids cairn changes" (member "- Do not create, sync, archive, or edit Cairn changes for this work." (octet-prompt-lines sample-config)))
    (check "prompt requires progress logging" (member "- Start each major update with `PROGRESS n/7 <phase>: <state>`." (octet-prompt-lines sample-config)))
    (check "prompt names octet goal" (string-prefix? "Goal: enforce every lint" sample-prompt))
    (check "command skeleton uses pinned flake package" (member "nix run .#cargo-octet -- --help" (command-lines sample-config)))
    (check "default run prompt file" (string=? (run-prompt-file sample-config) default-run-prompt-file))
    (write-text-file! "/tmp/ralph-octet-loop-self-test.prompt" "hello")
    (check "external prompt-file command success" (= (external-prompt-file-command-status "." "/tmp/ralph-octet-loop-self-test.prompt" "sh" (list "-c" "test \"$1\" = hello" "sh")) 0))
    (check "external command success" (= (external-command-status "." "true" '()) 0))
    (check "external command failure" (= (external-command-status "." "sh" (list "-c" (string-append "exit " (number->string failing-status-probe)))) failing-status-probe))
    (say "self-test complete")))

(define (script-args)
  (drop (command-line) command-line-arg-drop-count))

(define (main)
  (let ([config (parse-args (script-args))])
    (cond
      [(Config-help? config) (print-lines (usage-lines))]
      [(Config-self-test? config) (run-self-test)]
      [(Config-run? config) (run-pi! config)]
      [(Config-prompt-file config) (write-prompt-file! config)]
      [(Config-print-prompt? config) (print-lines (octet-prompt-lines config))]
      [else (print-lines (plan-lines config))])))

(main)
