const fs = require("fs");
const crypto = require("crypto");

const packageText = fs.readFileSync("Cargo.toml", { encoding: "utf8" });
const version = packageText.match(/version\s*=\s*\"(\d+\.\d+\.\d+)\"/)[1];

if (!/^\d+\.\d+\.\d+$/.test(version)) {
    throw new Error("Error extracting version.");
}

const outputFile = {
    schemaVersion: 1,
    name: "dprint-plugin-taplo",
    version,
    "mac-x86_64": getPlatformObject("dprint-plugin-taplo-x86_64-apple-darwin.zip"),
    "linux-x86_64": getPlatformObject("dprint-plugin-taplo-x86_64-unknown-linux-gnu.zip"),
    "windows-x86_64": getPlatformObject("dprint-plugin-taplo-x86_64-pc-windows-msvc.zip"),
};
fs.writeFileSync("taplo.exe-plugin", JSON.stringify(outputFile, undefined, 2), { encoding: "utf8" });

function getPlatformObject(zipFileName) {
    const fileBytes = fs.readFileSync(zipFileName);
    const hash = crypto.createHash("sha256");
    hash.update(fileBytes);
    const checksum = hash.digest("hex");
    console.log(zipFileName + ": " + checksum);
    return {
        "reference": `https://github.com/TiceCosmos/dprint-plugin-taplo/releases/download/${version}/${zipFileName}`,
        "checksum": checksum,
    };
}
