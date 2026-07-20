import { execSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";

const MAX_BYTES = 150 * 1024; // 150 KB

const out = execSync("npm pack --dry-run --json", { encoding: "utf8" });
// npm pack --json may print warnings before JSON
const start = out.indexOf("[");
const json = JSON.parse(out.slice(start));
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
