import { execSync } from "node:child_process";

const MAX_BYTES = 150 * 1024; // 150 KB

// CI already ran `npm run build`. Skip prepack (tsup) so its ANSI logs
// do not pollute --json output and break JSON.parse.
const raw = execSync("npm pack --dry-run --json --ignore-scripts", {
  encoding: "utf8",
  env: { ...process.env, NO_COLOR: "1", npm_config_color: "false" },
});

// Strip ANSI just in case, then locate the JSON array payload.
const cleaned = raw.replace(/\u001b\[[0-9;]*m/g, "");
const start = cleaned.indexOf("[");
const end = cleaned.lastIndexOf("]");
if (start === -1 || end === -1 || end < start) {
  console.error("Could not find JSON array in npm pack output:\n", cleaned);
  process.exit(1);
}

let json;
try {
  json = JSON.parse(cleaned.slice(start, end + 1));
} catch (err) {
  console.error("Failed to parse npm pack JSON:\n", cleaned.slice(start, end + 1));
  throw err;
}

const size = json[0]?.size ?? json[0]?.packedSize;
if (typeof size !== "number") {
  console.error("Could not parse pack size", json);
  process.exit(1);
}

console.log(`packed size: ${size} bytes (limit ${MAX_BYTES})`);
if (size > MAX_BYTES) {
  console.error("FAIL: package exceeds size budget");
  process.exit(1);
}
console.log("OK");
