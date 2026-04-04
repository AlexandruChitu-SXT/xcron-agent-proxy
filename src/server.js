const express = require('express');
const { applySemanticHashing } = require('./hasher');

const app = express();
const PORT = process.env.PORT || 8089;

app.use(express.json());

// Proxy Endpoint intercepting LLM completions
app.post('/v1/chat/completions', async (req, res) => {
    try {
        const agentPayload = req.body;
        
        console.log("\n[INCOMING] Request from MultiversX Agent:");
        console.log(JSON.stringify(agentPayload, null, 2));

        // Compress the payload using Semantic Hashing
        // For demonstration, we compress the first message content if it exists
        if (agentPayload.messages && agentPayload.messages.length > 0) {
            const rawContent = agentPayload.messages[0].content;
            agentPayload.messages[0].content = applySemanticHashing(rawContent);
        }

        console.log("\n[OUTGOING] Compressed Payload to OpenAI API:");
        console.log(JSON.stringify(agentPayload, null, 2));

        // In a real proxy, we would forward `agentPayload` via fetch/axios to https://api.openai.com/v1/chat/completions
        // and return the response to the client.
        
        // Mock Response
        res.json({
            status: "success",
            message: "Proxy interception and compression successful.",
            compressed_sent: agentPayload
        });

    } catch (error) {
        console.error("Proxy Error:", error);
        res.status(500).json({ error: "Failed to process agent payload through proxy." });
    }
});

app.listen(PORT, () => {
    console.log(`\n🚀 XCron Semantic Proxy running on port ${PORT}`);
    console.log(`Listening for Agent2Agent /tools/invoke API requests...`);
});
