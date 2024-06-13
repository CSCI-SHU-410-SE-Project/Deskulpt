// https://www.npmjs.com/package/internal-ip
// `internalIpV4Sync` and `internalIpV6Sync` simply return `undefined` in browser
// environement, so it is nice for testing the final bundle.
import { internalIpV4Sync, internalIpV6Sync } from "internal-ip";

// Duplicate `internalIpv4Sync`, and `internalIpv6Sync` duplicate named as `myFunc`
import "./file1.js";

// Duplicate `internalIpv6Sync`, and `internalIpv4Sync` duplicate named as `myFunc`
import "./file2.js";

console.log(internalIpV4Sync, internalIpV6Sync);
