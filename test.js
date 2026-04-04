const { encodingForModel } = require("js-tiktoken");
const anthropicTokenizer = require("@anthropic-ai/tokenizer");
const { applySemanticHashing } = require("./src/hasher");

async function runBenchmark() {
    // Dynamic import for ES module
    const llamaTokenizer = (await import("llama-tokenizer-js")).default;
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
    const originalLlamaTokens = llamaTokenizer.encode(originalText).length;

    console.log("\n=============================================");
    console.log("[+] ORIGINAL AGENT A2A PAYLOAD (JSON):");
    console.log(originalText);
    console.log("-> OPENAI (GPT-4) TOKENS BURNED:", originalGptTokens);
    console.log("-> ANTHROPIC (Claude 3) TOKENS BURNED:", originalClaudeTokens);
    console.log("-> META (Llama 3) TOKENS BURNED:", originalLlamaTokens);

    const compressedText = applySemanticHashing(originalText);
    const compressedGptTokens = gptEnc.encode(compressedText).length;
    const compressedClaudeTokens = anthropicTokenizer.countTokens(compressedText);
    const compressedLlamaTokens = llamaTokenizer.encode(compressedText).length;

    console.log("\n=============================================");
    console.log("[+] XCron PROXY COMPRESSED PAYLOAD (SEMANTIC HASH):");
    console.log(compressedText);
    console.log("-> OPENAI (GPT-4) TOKENS BURNED:", compressedGptTokens);
    console.log("-> ANTHROPIC (Claude 3) TOKENS BURNED:", compressedClaudeTokens);
    console.log("-> META (Llama 3) TOKENS BURNED:", compressedLlamaTokens);

    const gptReduction = Math.round((1 - compressedGptTokens / originalGptTokens) * 100);
    const claudeReduction = Math.round((1 - compressedClaudeTokens / originalClaudeTokens) * 100);
    const llamaReduction = Math.round((1 - compressedLlamaTokens / originalLlamaTokens) * 100);

    console.log("\n=============================================");
    console.log("=> EMPIRICAL RESULT: TOTAL UNIVERSAL SAVINGS");
    console.log("=> SAVED", gptReduction + "% on OpenAI GPT-4 Costs");
    console.log("=> SAVED", claudeReduction + "% on Anthropic Claude 3 Costs");
    console.log("=> SAVED", llamaReduction + "% on Meta Llama 3 Costs");
    console.log("=============================================\n");
}

runBenchmark();
