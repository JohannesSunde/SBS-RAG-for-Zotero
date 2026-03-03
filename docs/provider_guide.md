# LLM Provider System Guide

##  Overview

RAG Assistant for Zotero supports multiple LLM providers through a clean, provider-agnostic architecture:

- **Ollama** (Local, free)
- **OpenAI** (GPT-4, GPT-3.5)
- **Anthropic** (Claude)
- **Mistral** (Mistral Large, Mixtral)
- **Google** (Gemini)
- **Groq** (Fast Llama)
- **OpenRouter** (Unified access)


### Configure via UI

1. Open Settings page
2. Enable desired providers
3. Add API keys
4. Test connections
5. Select active provider/model
6. Save and start chatting!

##  Getting API Keys

### OpenAI
1. Go to https://platform.openai.com/api-keys
2. Create new secret key
3. Copy and paste into Settings

### Anthropic
1. Go to https://console.anthropic.com/account/keys
2. Create new API key
3. Copy and paste into Settings

### Google (Gemini)
1. Go to https://makersuite.google.com/app/apikey
2. Create API key
3. Copy and paste into Settings


##  Security

-  API keys masked in UI
-  Stored locally in `~/.zotero-llm/settings.json`
-  Never logged or exposed
-  Per-provider validation

##  Features

### Provider Management
- Enable/disable any provider
- Multiple providers enabled simultaneously
- Switch active provider anytime

### Model Selection
- Dynamic model loading per provider

