/**
 * Keccak256 event signature finder utility.
 *
 * Tries different event name and parameter type combinations to find
 * a match for a target keccak256 hash.
 */
import { keccak256, toUtf8Bytes } from "ethers";

export interface KeccakOptions {
  target: string;
  names: string[];
  typeSets: string[][];
}

function* permutations<T>(arr: T[]): Generator<T[]> {
  if (arr.length <= 1) {
    yield arr;
  } else {
    for (let i = 0; i < arr.length; i++) {
      const rest = [...arr.slice(0, i), ...arr.slice(i + 1)];
      for (const p of permutations(rest)) {
        yield [arr[i], ...p];
      }
    }
  }
}

export function findKeccakMatch(opts: KeccakOptions): string | null {
  for (const name of opts.names) {
    for (const typeSet of opts.typeSets) {
      const tried = new Set<string>();
      for (const perm of permutations(typeSet)) {
        const sig = `${name}(${perm.join(",")})`;
        if (tried.has(sig)) continue;
        tried.add(sig);
        const sigHash = keccak256(toUtf8Bytes(sig));
        if (sigHash.toLowerCase() === opts.target.toLowerCase()) {
          return sig;
        }
      }
    }
  }
  return null;
}

export function computeKeccak(signature: string): string {
  return keccak256(toUtf8Bytes(signature));
}
