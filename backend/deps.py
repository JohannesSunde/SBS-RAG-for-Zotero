from backend.profile_manager import ProfileManager
from backend.interface import ZoteroChatbot
from pathlib import Path
import os

# Home directory for default paths
_home = Path.home()
DEFAULT_DB_PATH = str(_home / "Zotero" / "zotero.sqlite")

# Singleton managers (will migrate to DI later)
profile_manager = ProfileManager()

def get_profile_manager():
    return profile_manager

def load_settings(profile_id: str = None):
    """Load settings from profile storage."""
    pm = get_profile_manager()
    if profile_id is None:
        active = pm.get_active_profile()
        if not active:
            raise RuntimeError("No active profile")
        profile_id = active['id']
    
    # Defaults
    profile_chroma_path = pm.get_profile_chroma_path(profile_id)
    default_settings = {
        "activeProviderId": "ollama",
        "activeModel": "",
        "embeddingModel": "bge-base",
        "zoteroPath": DEFAULT_DB_PATH,
        "chromaPath": profile_chroma_path,
        "providers": {
            provider: {"enabled": provider == "ollama", "credentials": {"base_url": "http://localhost:11434" if provider == "ollama" else ""}}
            for provider in ["ollama", "lmstudio", "openai", "anthropic", "mistral", "google", "groq", "openrouter", "github_models"]
        }
    }
    
    saved = pm.load_profile_settings(profile_id)
    if not saved:
        return default_settings
        
    # Simple merge for now (mirroring existing logic)
    merged = default_settings.copy()
    merged.update(saved)
    return merged

def get_chatbot():
    """Dependency for getting the chatbot instance."""
    # For now, we reuse the legacy global approach but wrap it in a function
    # In a full DI refactor, this would be managed by the app lifecycle
    from backend import main # Avoid circular import if needed, but better to move chatbot init here
    return main.chatbot
