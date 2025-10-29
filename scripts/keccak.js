const { keccak256, toUtf8Bytes } = require("ethers");

const target = "0x16624d4e070855bf4f06e05f0fc8f60958fbb7fc14336e1d4f94df210e2d585a";
const names = ["TokenCreated", "TokenCreate", "NewToken", "TokenLaunched", "CreateToken", "TokenFactoryCreate"];
const typeSets = [
    ["address", "address", "uint256", "uint256", "string", "string"],
];

function* permutations(arr) {
    if (arr.length <= 1) yield arr;
    else for (let i = 0; i < arr.length; i++) {
        const rest = [...arr.slice(0, i), ...arr.slice(i + 1)];
        for (const p of permutations(rest)) yield [arr[i], ...p];
    }
}

for (const n of names) {
    for (const ts of typeSets) {
        const tried = new Set();
        for (const perm of permutations(ts)) {
            const sig = `${n}(${perm.join(",")})`;
            if (tried.has(sig)) continue;
            const sigHash = keccak256(toUtf8Bytes(sig));
            if (sigHash.toLowerCase() === target.toLowerCase()) {
                console.log("MATCH:", sig);
                process.exit(0);
            }
        }
    }
}
console.log("No match in tested variants.");
