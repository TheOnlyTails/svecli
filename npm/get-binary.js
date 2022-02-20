const {Binary} = require("@cloudflare/binary-install");
const os = require("os");
const {version} = require("./package.json");

const getPlatform = () => {
    const type = os.type();
    const arch = os.arch();

    if (type === "Windows_NT" && arch === "x64") return "win64";
    if (type === "Linux" && arch === "x64") return "linux";
    if (type === "Darwin" && arch === "x64") return "macos";
    if (type === "Darwin" && arch === "arm64") return "macos-arm";

    throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
    const platform = getPlatform();
    const url = `https://github.com/theonlytails/svecli/releases/download/${version}/svecli-${platform}.tar.gz`;
    const name = "svecli";
    return new Binary(url, {name});
};

module.exports = getBinary;
