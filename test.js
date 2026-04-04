const { encodingForModel } = require("js-tiktoken");
const anthropicTokenizer = require("@anthropic-ai/tokenizer");
const { applySemanticHashing } = require("./src/hasher");

async function runBenchmark() {
    // OpenAI GPT Tokenizer
    const gptEnc = encodingForModel("gpt-4");
    // Anthropic Claude Tokenizer
    const claudeEnc = anthropicTokenizer.getTokenizer();

    const agentPayload = {
        "system_routine": "tools/invoke",
        "target_orchestrator": "erd1qqqqqqqqqqqqqpgqtzylnzxc20xmd5a9krt8t9l8kndr90rtv7ls639v39",
        "task": {
            "action": "analyze mempool",
            "condition": "if volatile",
            "execution": "execute arbitrage"
        }
    };

    const originalText = JSON.stringify(agentPayload);
    const originalGptTokens = gptEnc.encode(originalText).length;
    const originalClaudeTokens = anthropicTokenizer.countTokens(originalText);

    console.log("\n=============================================");
    console.log("[+] ORIGINAL AGENT A2A PAYLOAD (JSON):");
    console.log(originalText);
    console.log("-> OPENAI (GPT-4) TOKENS BURNED:", originalGptTokens);
    console.log("-> ANTHROPIC (Claude 3) TOKENS BURNED:", originalClaudeTokens);

    const compressedText = applySemanticHashing(originalText);
    const compressedGptTokens = gptEnc.encode(compressedText).length;
    const compressedClaudeTokens = anthropicTokenizer.countTokens(compressedText);

    console.log("\n=============================================");
    console.log("[+] XCron PROXY COMPRESSED PAYLOAD (SEMANTIC HASH):");
    console.log(compressedText);
    console.log("-> OPENAI (GPT-4) TOKENS BURNED:", compressedGptTokens);
    console.log("-> ANTHROPIC (Claude 3) TOKENS BURNED:", compressedClaudeTokens);

    const gptReduction = Math.round((1 - compressedGptTokens / originalGptTokens) * 100);
    const claudeReduction = Math.round((1 - compressedClaudeTokens / originalClaudeTokens) * 100);

    console.log("\n=============================================");
    console.log("=> EMPIRICAL RESULT:");
    console.log("=> SAVED", gptReduction + "% on OpenAI GPT-4 Costs");
    console.log("=> SAVED", claudeReduction + "% on Anthropic Claude 3 Costs");
    console.log("=============================================\n");
}

runBenchmark();
