import { Type } from "@earendil-works/pi-ai";
import { defineTool, type ExtensionAPI, withFileMutationQueue } from "@earendil-works/pi-coding-agent";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve, relative, isAbsolute } from "node:path";

const TOOL_NAME = "mc_validate_vanilla_combat_checker";
const TOOL_LABEL = "MC Validate Vanilla Combat Checker";
const DEFAULT_SOURCE_PATH = "tools/check_vanilla_combat_reference_parity.rs";
const DEFAULT_OUTPUT_PATH = "target/check-vanilla-combat-reference-parity";
const SELF_TEST_FLAG = "--self-test";
const NIX_COMMAND = "nix";
const NIX_DEVELOP_SUBCOMMAND = "develop";
const NO_UPDATE_LOCK_FILE_FLAG = "--no-update-lock-file";
const NIX_COMMAND_SEPARATOR_FLAG = "-c";
const RUSTC_COMMAND = "rustc";
const RUST_EDITION_FLAG = "--edition=2021";
const RUST_OUTPUT_FLAG = "-o";
const B3SUM_COMMAND = "b3sum";
const COMPILE_STEP = "compile";
const SELF_TEST_STEP = "self-test";
const BLAKE3_STEP = "blake3";
const SUCCESS_EXIT_CODE = 0;
const COMPILE_TIMEOUT_MS = 120_000;
const SELF_TEST_TIMEOUT_MS = 60_000;
const BLAKE3_TIMEOUT_MS = 30_000;
const MAX_OUTPUT_CHARS = 20_000;
const DIGEST_HEX_LENGTH = 64;
const PATH_PARENT = "..";
const AT_PREFIX = "@";
const UTF8_ENCODING = "utf8";
const RUN_LOG_SUFFIX = ".run.log";
const BLAKE3_SUFFIX = ".b3";
const LOWERCASE_HEX_DIGITS = /^[0-9a-f]+$/;

interface ValidationParams {
	repoDir?: string;
	sourcePath?: string;
	logPath?: string;
}

interface ResolvedValidationPlan {
	repoDir: string;
	sourcePath: string;
	outputPath: string;
	logPath?: string;
	logRelPath?: string;
	blake3Path?: string;
	compileArgs: string[];
	selfTestCommand: string;
	selfTestArgs: string[];
	blake3Args?: string[];
}

interface CommandOutcome {
	step: string;
	code: number;
	stdout: string;
	stderr: string;
}

interface ValidationOutcome {
	status: "pass" | "fail";
	exitStatus: number;
	plan: ResolvedValidationPlan;
	compile: CommandOutcome;
	selfTest?: CommandOutcome;
	blake3?: CommandOutcome;
	blake3Digest?: string;
	blake3Path?: string;
	logPath?: string;
}

function stripAtPrefix(path: string): string {
	return path.startsWith(AT_PREFIX) ? path.slice(AT_PREFIX.length) : path;
}

function resolveInputPath(baseDir: string, path: string | undefined, fallback: string): string {
	const selected = stripAtPrefix(path && path.length > 0 ? path : fallback);
	return isAbsolute(selected) ? selected : resolve(baseDir, selected);
}

function assertInsideRepo(repoDir: string, path: string, label: string): string {
	const rel = relative(repoDir, path);
	if (rel === "" || (!rel.startsWith(PATH_PARENT) && !isAbsolute(rel))) {
		return rel;
	}
	throw new Error(`${label} must stay inside repoDir: ${path}`);
}

function deriveBlake3Path(logPath: string): string {
	if (logPath.endsWith(RUN_LOG_SUFFIX)) {
		return `${logPath.slice(0, -RUN_LOG_SUFFIX.length)}${BLAKE3_SUFFIX}`;
	}
	return `${logPath}${BLAKE3_SUFFIX}`;
}

function buildPlan(ctxCwd: string, params: ValidationParams): ResolvedValidationPlan {
	const repoDir = resolveInputPath(ctxCwd, params.repoDir, ctxCwd);
	const sourcePath = resolveInputPath(repoDir, params.sourcePath, DEFAULT_SOURCE_PATH);
	const sourceRel = assertInsideRepo(repoDir, sourcePath, "sourcePath");
	if (sourceRel !== DEFAULT_SOURCE_PATH) {
		throw new Error(`${TOOL_NAME} only validates ${DEFAULT_SOURCE_PATH}`);
	}

	const outputPath = resolve(repoDir, DEFAULT_OUTPUT_PATH);
	const outputRel = assertInsideRepo(repoDir, outputPath, "outputPath");
	const logPath = params.logPath
		? resolveInputPath(repoDir, params.logPath, params.logPath)
		: undefined;
	const logRelPath = logPath ? assertInsideRepo(repoDir, logPath, "logPath") : undefined;
	const blake3Path = logPath ? deriveBlake3Path(logPath) : undefined;
	if (blake3Path) {
		assertInsideRepo(repoDir, blake3Path, "blake3Path");
	}

	return {
		repoDir,
		sourcePath: sourceRel,
		outputPath: outputRel,
		logPath,
		logRelPath,
		blake3Path,
		compileArgs: [
			NIX_DEVELOP_SUBCOMMAND,
			NO_UPDATE_LOCK_FILE_FLAG,
			NIX_COMMAND_SEPARATOR_FLAG,
			RUSTC_COMMAND,
			RUST_EDITION_FLAG,
			sourceRel,
			RUST_OUTPUT_FLAG,
			outputRel,
		],
		selfTestCommand: outputPath,
		selfTestArgs: [SELF_TEST_FLAG],
		blake3Args: logRelPath
			? [
					NIX_DEVELOP_SUBCOMMAND,
					NO_UPDATE_LOCK_FILE_FLAG,
					NIX_COMMAND_SEPARATOR_FLAG,
					B3SUM_COMMAND,
					logRelPath,
				]
			: undefined,
	};
}

function truncateOutput(text: string): string {
	if (text.length <= MAX_OUTPUT_CHARS) {
		return text;
	}
	return `${text.slice(0, MAX_OUTPUT_CHARS)}\n[truncated to ${MAX_OUTPUT_CHARS} chars]`;
}

function renderCommandOutcome(outcome: CommandOutcome): string {
	return [
		`## ${outcome.step}`,
		`exit_code=${outcome.code}`,
		"### stdout",
		truncateOutput(outcome.stdout),
		"### stderr",
		truncateOutput(outcome.stderr),
	].join("\n");
}

function renderValidationLog(outcome: ValidationOutcome): string {
	const sections = [
		"checkpoint: vanilla combat reference checker rustc/nix validation",
		`tool=${TOOL_NAME}`,
		`repo_dir=${outcome.plan.repoDir}`,
		`source_path=${outcome.plan.sourcePath}`,
		`output_path=${outcome.plan.outputPath}`,
		`status=${outcome.status}`,
		renderCommandOutcome(outcome.compile),
	];
	if (outcome.selfTest) {
		sections.push(renderCommandOutcome(outcome.selfTest));
	}
	if (outcome.blake3Digest) {
		sections.push(`blake3=${outcome.blake3Digest}`);
	}
	if (outcome.blake3Path) {
		sections.push(`blake3_manifest=${outcome.blake3Path}`);
	}
	sections.push(`exit_status=${outcome.exitStatus}`);
	return `${sections.join("\n\n")}\n`;
}

function summarizeOutcome(outcome: ValidationOutcome): string {
	const logSuffix = outcome.logPath ? `\nlog: ${outcome.logPath}` : "";
	const blake3Suffix = outcome.blake3Path ? `\nblake3: ${outcome.blake3Path}` : "";
	if (outcome.status === "pass") {
		return `vanilla combat checker validation passed${logSuffix}${blake3Suffix}`;
	}
	return `vanilla combat checker validation failed with exit_status=${outcome.exitStatus}${logSuffix}${blake3Suffix}`;
}

async function writeTextFileQueued(path: string, text: string): Promise<void> {
	await withFileMutationQueue(path, async () => {
		await mkdir(dirname(path), { recursive: true });
		await writeFile(path, text, UTF8_ENCODING);
	});
}

function parseBlake3Digest(stdout: string): string {
	for (const line of stdout.split(/\r?\n/)) {
		const [candidate] = line.trim().split(/\s+/);
		if (
			candidate &&
			candidate.length === DIGEST_HEX_LENGTH &&
			LOWERCASE_HEX_DIGITS.test(candidate)
		) {
			return candidate;
		}
	}
	throw new Error("b3sum output did not contain a BLAKE3 digest");
}

async function writeBlake3Manifest(path: string, digest: string, logRelPath: string): Promise<void> {
	await writeTextFileQueued(path, `${digest}  ${logRelPath}\n`);
}

function createMcValidateVanillaCombatCheckerTool(pi: ExtensionAPI) {
	return defineTool({
	name: TOOL_NAME,
	label: TOOL_LABEL,
	description:
		"Compile and self-test the allowlisted mc vanilla combat reference parity Rust checker through nix develop.",
	promptSnippet:
		"Run the allowlisted mc vanilla combat reference parity Rust checker compile/self-test via nix develop.",
	promptGuidelines: [
		"Use mc_validate_vanilla_combat_checker only for tools/check_vanilla_combat_reference_parity.rs; do not use it as a generic shell runner.",
		"When mc_validate_vanilla_combat_checker writes evidence logs, verify the log contains exit_status=0 before citing it in Cairn tasks.",
	],
	parameters: Type.Object({
		repoDir: Type.Optional(
			Type.String({
				description: "Repository root. Defaults to the current cwd.",
			}),
		),
		sourcePath: Type.Optional(
			Type.String({
				description:
					"Must be tools/check_vanilla_combat_reference_parity.rs. Defaults to that allowlisted path.",
			}),
		),
		logPath: Type.Optional(
			Type.String({
				description:
					"Optional repo-local path for the validation log, for example docs/evidence/name.run.log.",
			}),
		),
	}),

	async execute(_toolCallId, params, signal, onUpdate, ctx) {
		const plan = buildPlan(ctx.cwd, params as ValidationParams);
		onUpdate?.({
			content: [
				{
					type: "text",
					text: `Compiling ${plan.sourcePath} through nix develop...`,
				},
			],
			details: { step: COMPILE_STEP, plan },
		});

		const compileResult = await pi.exec(NIX_COMMAND, plan.compileArgs, {
			cwd: plan.repoDir,
			signal,
			timeout: COMPILE_TIMEOUT_MS,
		});
		const compile: CommandOutcome = {
			step: COMPILE_STEP,
			code: compileResult.code,
			stdout: compileResult.stdout,
			stderr: compileResult.stderr,
		};

		let selfTest: CommandOutcome | undefined;
		if (compile.code === SUCCESS_EXIT_CODE) {
			onUpdate?.({
				content: [{ type: "text", text: `Running ${SELF_TEST_FLAG}...` }],
				details: { step: SELF_TEST_STEP, plan },
			});
			const selfTestResult = await pi.exec(plan.selfTestCommand, plan.selfTestArgs, {
				cwd: plan.repoDir,
				signal,
				timeout: SELF_TEST_TIMEOUT_MS,
			});
			selfTest = {
				step: SELF_TEST_STEP,
				code: selfTestResult.code,
				stdout: selfTestResult.stdout,
				stderr: selfTestResult.stderr,
			};
		}

		const exitStatus = compile.code !== SUCCESS_EXIT_CODE ? compile.code : (selfTest?.code ?? SUCCESS_EXIT_CODE);
		const outcome: ValidationOutcome = {
			status: exitStatus === SUCCESS_EXIT_CODE ? "pass" : "fail",
			exitStatus,
			plan,
			compile,
			selfTest,
			logPath: plan.logPath,
		};

		if (plan.logPath) {
			await writeTextFileQueued(plan.logPath, renderValidationLog(outcome));
		}

		if (
			outcome.status === "pass" &&
			plan.logPath &&
			plan.logRelPath &&
			plan.blake3Path &&
			plan.blake3Args
		) {
			onUpdate?.({
				content: [{ type: "text", text: "Writing BLAKE3 evidence sidecar..." }],
				details: { step: BLAKE3_STEP, plan },
			});
			const blake3Result = await pi.exec(NIX_COMMAND, plan.blake3Args, {
				cwd: plan.repoDir,
				signal,
				timeout: BLAKE3_TIMEOUT_MS,
			});
			outcome.blake3 = {
				step: BLAKE3_STEP,
				code: blake3Result.code,
				stdout: blake3Result.stdout,
				stderr: blake3Result.stderr,
			};
			if (outcome.blake3.code !== SUCCESS_EXIT_CODE) {
				outcome.status = "fail";
				outcome.exitStatus = outcome.blake3.code;
			} else {
				outcome.blake3Digest = parseBlake3Digest(outcome.blake3.stdout);
				outcome.blake3Path = plan.blake3Path;
				await writeBlake3Manifest(plan.blake3Path, outcome.blake3Digest, plan.logRelPath);
			}
		}

		const summary = summarizeOutcome(outcome);
		if (outcome.status !== "pass") {
			throw new Error(summary);
		}

		return {
			content: [{ type: "text", text: summary }],
			details: outcome,
		};
	},
	});
}

export default function (pi: ExtensionAPI) {
	pi.registerTool(createMcValidateVanillaCombatCheckerTool(pi));
}
