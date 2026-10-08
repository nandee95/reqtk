import fs from "fs";
import path from "path";

const iconMapPath = path.resolve("node_modules/vscode-material-icons/generated/icon-map.json");
const iconsPath = path.resolve("node_modules/vscode-material-icons/generated/icons");

const iconMap = JSON.parse(fs.readFileSync(iconMapPath, "utf8"));

console.log("pub static VSCODE_MATERIAL_ICONS: phf::Map<&'static str, &'static str> = phf::phf_map! {");

type IconDefinition = {
    iconPath: string;
};

for (const [icon, value] of Object.entries(
    iconMap.iconDefinitions as Record<string, IconDefinition>
)) {
    if (!value.iconPath) continue;

    const file = path.join(iconsPath, value.iconPath);

    const svg = fs.readFileSync(file, "utf8");
    const match = svg.match(/<path\b[^>]*\bd="([^"]+)"/);

    if (!match) continue;

    console.log(`    "${icon}" => "${match[1]}",`);
}
console.log("};");

console.log("pub static VSCODE_MATERIAL_FILENAMES: phf::Map<&'static str, &'static str> = phf::phf_map! {");
for (const [name, value] of Object.entries(iconMap.fileNames)) {
    console.log(`    "${name}" => "${value}",`);
}

console.log("};");


console.log("pub static VSCODE_MATERIAL_EXTENSIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {");
for (const [name, value] of Object.entries(iconMap.fileExtensions)) {
    console.log(`    "${name}" => "${value}",`);
}

console.log("};");