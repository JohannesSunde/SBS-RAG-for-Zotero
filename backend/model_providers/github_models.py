"""
GitHub Models provider.

Uses the GitHub Models API (https://models.inference.ai.azure.com),
which is OpenAI-compatible and authenticated with a GitHub personal
access token. Works with GitHub Copilot subscriptions and free accounts
(subject to rate limits).
"""

from typing import Dict, Any, List
from .base import Message, ChatResponse, ModelInfo
from .openai import OpenAIProvider


GITHUB_MODELS_BASE_URL = "https://models.inference.ai.azure.com"


class GitHubModelsProvider(OpenAIProvider):
    """Provider implementation for GitHub Models (OpenAI-compatible API)."""

    def __init__(self):
        super().__init__()
        self._id = "github_models"
        self._label = "GitHub Models"
        self._default_model = "gpt-4o-mini"

    def _with_base_url(self, credentials: Dict[str, Any]) -> Dict[str, Any]:
        """Return credentials with the hardcoded GitHub Models base URL."""
        creds = dict(credentials)
        creds["base_url"] = GITHUB_MODELS_BASE_URL
        return creds

    def _get_client(self, credentials: Dict[str, Any]):
        return super()._get_client(self._with_base_url(credentials))

    def _list_models_raw(self, credentials: Dict[str, Any]):
        return super()._list_models_raw(self._with_base_url(credentials))

    def validate_credentials(self, credentials: Dict[str, Any]) -> bool:
        return super().validate_credentials(self._with_base_url(credentials))

    def validate_credentials_and_list_models(self, credentials: Dict[str, Any]) -> Dict[str, Any]:
        return super().validate_credentials_and_list_models(self._with_base_url(credentials))

    def list_models(self, credentials: Dict[str, Any]) -> List[ModelInfo]:
        return super().list_models(self._with_base_url(credentials))

    def chat(
        self,
        credentials: Dict[str, Any],
        model: str,
        messages: List[Message],
        **kwargs,
    ) -> ChatResponse:
        return super().chat(self._with_base_url(credentials), model, messages, **kwargs)
