import fs from "fs";
import path from "path";

const root = path.resolve("node_modules/@material-design-icons/svg/filled");

console.log("pub static MATERIAL_ICONS: phf::Map<&'static str, &'static str> = phf::phf_map! {");

for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    if (!entry.name.endsWith(".svg")) continue;
    const file = path.join(root, entry.name);

    const icon = entry.name.replace(/\.svg$/, "");

    if (!icon) continue;

    const svg = fs.readFileSync(file, "utf8");
    const match = svg.match(/<path\b[^>]*\bd="([^"]+)"/);

    if (!match) continue;

    console.log(`    "${icon}" => "${match[1]}",`);
}

console.log("};");