import {Binary} from "binary-install";
import os from "os";
import packageJson from "./package.json" assert {type: "json"};

const getPlatform = () => {
    const type = os.type();
    const arch = os.arch();

    if (type === "Windows_NT" && arch === "x64") return "win64";
    if (type === "Linux" && arch === "x64") return "linux";
    if (type === "Darwin" && arch === "x64") return "macos";

    throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
    const platform = getPlatform();
    const url = `https://github.com/theonlytails/svecli/releases/download/v${packageJson.version}/svecli-${platform}.tar.gz`;
    const name = "svecli";
    return new Binary(url, {name});
};

export default getBinary;
