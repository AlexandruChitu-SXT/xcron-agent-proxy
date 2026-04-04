# XCron Semantic Agent Proxy

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, high-performance, model-agnostic middleware proxy designed natively for Web3 Artificial Intelligence Agents operating on the MultiversX network (and beyond). 

This proxy fundamentally reduces off-chain LLM inference API costs (OpenAI GPT-4, Anthropic Claude 3) by **up to 70-75%** by intercepting standard Conversational Agent JSON outputs and applying **Strict Semantic Hashing** before hitting the tokenizers.

## 🧠 The Problem
Autonomous Agents (especially in heavy A2A environments using `/tools/invoke` or similar paradigms) burn massive amounts of API tokens transmitting repetitive human-semantic structures, massive JSON brackets, colons, and quotation marks. 

Standard character-reduction ("Shorthand") fails to reduce costs efficiently due to how Byte-Pair Encoding (BPE) or SentencePiece tokenizers split non-standard dictionary words.

## 🛠 The Solution: Semantic Hashing
`xcron-agent-proxy` sits between your backend Agent application and the OpenAI/Anthropic API edge. It natively parses your JSON Agent prompts and replaces them with strict mathematical Dictionary Arrays, eradicating the physical formatting.

### Empirical Benchmarks
Our onboard auditing scripts utilizing raw OpenAI `js-tiktoken` and the `@anthropic-ai/tokenizer` yield the following savings on standard MultiversX A2A Agent instructions:
* **OpenAI (GPT-4)**: 70% Token Reduction
* **Anthropic (Claude 3)**: 74% Token Reduction

## 🚀 Quickstart

**1. Clone & Install**
```bash
git clone https://github.com/XCronProtocol/xcron-agent-proxy.git
cd xcron-agent-proxy
npm install
```

**2. Audit The Math Yourself**
We believe in "Don't Trust, Verify". Run our internal empirical benchmark tests right out of the box:
```bash
node test.js
```

**3. Run the Proxy Middleware**
```bash
node src/server.js
```
The server binds to `http://localhost:8089/v1/chat/completions`. Point your local Agent framework (OpenClaw, etc) to this local endpoint instead of directly to OpenAI.

## 🏗 Architecture
- `/src/server.js` - The fast Express Engine intercepting the API payloads.
- `/src/hasher.js` - The cryptographic dictionary mapper tearing down the JSON formatting.
- `test.js` - Real-world benchmarking against GPT-4 and Claude 3.

## 📜 License
MIT License. Built by XCron Protocol for the builder ecosystem.
