#!/usr/bin/env node

const { spawn } = require("node:child_process");
const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");

/**
 * Wrapper around Trunk to avoid macOS AppleDouble (._*) files causing
 * distribution apply failures on filesystems without extended attribute
 * support (e.g., exFAT).
 *
 * We build into a temp folder on the local disk (to avoid generating AppleDouble
 * files) and then copy the output into the project `dist/` folder, skipping any
 * dot-Apple files if the underlying filesystem insists on creating them.
 */

const mode = process.argv[2] ?? "build";
if (!["build", "serve"].includes(mode)) {
  console.error(`Unknown trunk mode "${mode}". Use "build" or "serve".`);
  process.exit(1);
}

const projectDist = path.resolve("dist");
const trunkDist =
  process.env.TRUNK_BUILD_DIST ??
  path.join(os.tmpdir(), "sql_intelliscan-trunk-dist");
const trunkTarget =
  process.env.CARGO_TARGET_DIR ??
  path.join(os.tmpdir(), "sql_intelliscan-trunk-target");
const stagePath = path.join(trunkDist, ".stage");

run().catch((error) => {
  console.error(error);
  process.exit(1);
});

async function run() {
  await removeAppleDouble(path.join(trunkTarget, "wasm-bindgen"));
  await runTailwindBuild();

  if (mode === "serve") {
    await resetDistRoots();
    const buildExitCode = await runTrunk("build", trunkDist);
    if (buildExitCode !== 0) {
      process.exit(buildExitCode ?? 1);
    }

    await finalizeProjectDist();
    process.exit(await runTrunk("serve", trunkDist));
  }

  await resetDistRoots();
  const exitCode = await runTrunk("build", trunkDist);

  // On success, clean any AppleDouble files from the final dist just in case.
  if (exitCode === 0) {
    await finalizeProjectDist();
    return;
  }

  const stageExists = await exists(stagePath);
  if (!stageExists) {
    process.exit(exitCode ?? 1);
  }

  console.warn(
    "Trunk failed while applying the distribution. Cleaning AppleDouble artifacts and finalizing manually…"
  );

  await removeAppleDouble(stagePath);
  await removeAppleDouble(trunkDist);

  // Preserve the staging directory while clearing the rest of dist.
  await clearDistExceptStage();
  await copyStageIntoDist();
  await removeAppleDouble(trunkDist);
  await syncToProjectDist();
  await removeAppleDouble(projectDist);
  await sanitizeHtmlReferences(projectDist);
  await fs.rm(stagePath, { recursive: true, force: true });
}

function runTailwindBuild() {
  return new Promise((resolve, reject) => {
    const npm = process.platform === "win32" ? "npm.cmd" : "npm";
    const child = spawn(npm, ["run", "tailwind:build"], {
      stdio: "inherit",
      env: {
        ...process.env,
        NO_COLOR: "true",
      },
    });

    child.on("exit", (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      }

      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`tailwind:build failed with exit code ${code ?? 1}`));
      }
    });
  });
}

async function resetDistRoots() {
  await fs.rm(trunkDist, { recursive: true, force: true });
  await fs.rm(projectDist, { recursive: true, force: true });
  await fs.mkdir(trunkDist, { recursive: true });
  await fs.mkdir(projectDist, { recursive: true });
}

function runTrunk(trunkMode, dist) {
  return new Promise((resolve) => {
    const child = spawn("trunk", [trunkMode, "--dist", dist], {
      stdio: "inherit",
      env: {
        ...process.env,
        CARGO_TARGET_DIR: trunkTarget,
        COPYFILE_DISABLE: "1",
        NO_COLOR: "true",
      },
    });

    child.on("exit", (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      }
      resolve(code ?? 1);
    });
  });
}

async function finalizeProjectDist() {
  await removeAppleDouble(trunkDist);
  await syncToProjectDist();
  await removeAppleDouble(projectDist);
  await sanitizeHtmlReferences(projectDist);
}

async function copyStageIntoDist() {
  await fs.mkdir(trunkDist, { recursive: true });
  await copyDir(stagePath, trunkDist, { skipStage: true });
}

async function syncToProjectDist() {
  await fs.rm(projectDist, { recursive: true, force: true });
  await fs.mkdir(projectDist, { recursive: true });
  await copyDir(trunkDist, projectDist, { skipStage: true });
}

async function copyDir(from, to, { skipStage = false } = {}) {
  const entries = await fs.readdir(from, { withFileTypes: true });
  for (const entry of entries) {
    // Skip AppleDouble files to avoid double-move issues.
    if (entry.name.startsWith("._") || entry.name === ".DS_Store") {
      continue;
    }
    if (skipStage && entry.name === ".stage") {
      continue;
    }

    const source = path.join(from, entry.name);
    const destination = path.join(to, entry.name);

    if (entry.isDirectory()) {
      await fs.mkdir(destination, { recursive: true });
      await copyDir(source, destination);
    } else {
      await fs.copyFile(source, destination);
    }
  }
}

async function clearDistExceptStage() {
  const entries = (await exists(trunkDist))
    ? await fs.readdir(trunkDist, { withFileTypes: true })
    : [];

  for (const entry of entries) {
    if (entry.name === ".stage") {
      continue;
    }

    const target = path.join(trunkDist, entry.name);
    await fs.rm(target, { recursive: true, force: true });
  }
}

async function removeAppleDouble(root) {
  if (!(await exists(root))) return;

  const entries = await fs.readdir(root, { withFileTypes: true });
  await Promise.all(
    entries.map(async (entry) => {
      const target = path.join(root, entry.name);
      if (entry.isDirectory()) {
        await removeAppleDouble(target);
      }

      if (entry.name.startsWith("._") || entry.name === ".DS_Store") {
        await fs.rm(target, { recursive: true, force: true });
      }
    })
  );
}

async function sanitizeHtmlReferences(root) {
  const indexPath = path.join(root, "index.html");
  if (!(await exists(indexPath))) return;

  const html = await fs.readFile(indexPath, "utf8");
  const sanitized = html.replace(
    /<link[^>]+href="[^"]*\/\._[^"]+"[^>]*>/g,
    ""
  );

  if (sanitized !== html) {
    await fs.writeFile(indexPath, sanitized);
  }
}

async function exists(target) {
  try {
    await fs.access(target);
    return true;
  } catch {
    return false;
  }
}
