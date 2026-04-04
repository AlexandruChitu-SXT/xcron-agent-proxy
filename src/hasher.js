/**
 * XCron Semantic Hasher (Token Optimizer)
 * 
 * Replaces human-semantic structures and heavy JSON arrays 
 * with pre-defined Token Dictionary Mappings (Semantic Hashing).
 */

const DICTIONARY = {
    "tools/invoke": "Z1",
    "sessions_send": "Z2",
    "erd1qqqqqqqqqqqqqpgqtzylnzxc20xmd5a9krt8t9l8kndr90rtv7ls639v39": "W1",
    "execute arbitrage": "Z3",
    "analyze mempool": "Z4",
    "multiversx": "MX"
};

/**
 * Strips JSON formatting and applies the Semantic Dictionary.
 */
function applySemanticHashing(payload) {
    let str = typeof payload === 'string' ? payload : JSON.stringify(payload);
    
    // 1. Dictionary Substitution (Semantic Compression)
    for (const [humanText, hash] of Object.entries(DICTIONARY)) {
        const regex = new RegExp(humanText, "gi");
        str = str.replace(regex, hash);
    }

    // 2. Schema Flattening (JSON Obj -> Positional Array)
    // Convert {"action":"Z1", "target":"W1"} -> [Z1,W1]
    // Here we aggressively strip all JSON quotation marks, brackets, and keys to create a flat stream
    str = str.replace(/"[a-zA-Z_]+"\s*:\s*"?([^",}]+)"?/g, "$1|");
    
    // Cleanup remaining JSON structure characters
    str = str.replace(/\{|\}|\[|\]/g, "");
    
    return str.replace(/\|+/g, "|").replace(/\|$/, "").trim().toUpperCase();
}

module.exports = {
    applySemanticHashing,
    DICTIONARY
};
