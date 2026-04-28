import { invoke } from '@tauri-apps/api/core';

// Log API client configuration for debugging
console.log('[API Client] Initializing Native Tauri Bridge');

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  // Strip leading slash
  const route = path.startsWith("/") ? path.substring(1) : path;
  
  // Try to parse body if it exists
  let payload = {};
  if (options.body && typeof options.body === 'string') {
    try {
      payload = JSON.parse(options.body);
    } catch (e) {
      console.warn("Failed to parse request body as JSON:", e);
    }
  }

  // Map REST paths to Tauri Commands
  // Example: "api/chat" -> "chat_command"
  const commandMap: Record<string, string> = {
    "api/chat": "chat_command",
    "api/health": "health_check",
    "api/db_health": "health_check",
    "api/settings": "get_settings",
    "api/search_items": "search_items"
  };

  const command = commandMap[route];

  if (!command) {
    console.warn(`[Tauri IPC] Unmapped route: ${route}`);
    // Fallback or throw error depending on strictness
    throw new Error(`Unmapped API route for native IPC: ${route}`);
  }

  try {
    console.log(`[Tauri IPC] Invoking ${command}`, payload);
    const result = await invoke<T>(command, payload);
    return result;
  } catch (error) {
    console.error(`[Tauri IPC] Error invoking ${command}:`, error);
    throw new Error(typeof error === 'string' ? error : (error as any).message || 'Unknown IPC error');
  }
}

/**
 * Fallback fetch function for raw URLs (e.g., fetching a PDF)
 */
function apiFetch(path: string, options?: RequestInit): Promise<Response> {
  // In a fully native app, we shouldn't use fetch for local resources.
  // We should use tauri's convertFileSrc for PDFs, or an IPC command that returns bytes.
  console.warn(`[Tauri IPC] apiFetch called for ${path}. This should be refactored to native FS access.`);
  return fetch(path, options);
}

export { request, apiFetch };
