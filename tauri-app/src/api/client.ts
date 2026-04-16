import { invoke } from "@tauri-apps/api/core";

// Tauri doesn't need a BASE_URL for IPC, but we keep it for compatibility if needed elsewhere
const BASE_URL = "";

async function request<T>(path: string, options: any = {}): Promise<T> {
  // Map path to Rust command
  // Legacy paths from the Python backend are converted to camelCase commands
  const commandMap: Record<string, string> = {
    "/api/chat": "chat",
    "/api/index_library": "index_library",
    "/api/index_status": "index_status",
    "/api/search_items": "get_zotero_items",
    "/api/health": "health_check", // placeholder
  };

  const command = commandMap[path] || path.replace("/api/", "").replace(/\//g, "_");
  
  console.log(`[Tauri API] Invoking command: ${command}`);

  // In Tauri 2.0, invoke takes an object as second argument.
  // We parse the legacy 'body' if it exists.
  let args = {};
  if (options.body) {
    try {
      args = typeof options.body === 'string' ? JSON.parse(options.body) : options.body;
    } catch (e) {
      console.warn("Failed to parse body as JSON", e);
    }
  } else if (options.params) {
    args = options.params;
  }

  try {
    const data = await invoke(command, args);
    
    // Compatibility check for backend error responses
    if (data && typeof data === 'object' && 'error' in (data as any)) {
      throw new Error((data as any).error || 'An error occurred');
    }

    return data as T;
  } catch (err: any) {
    console.error(`[Tauri API] Command ${command} failed:`, err);
    throw new Error(err.toString());
  }
}

/**
 * Compatibility fetch-like function for Tauri
 */
async function apiFetch(path: string, options?: any): Promise<any> {
  const data = await request(path, options);
  // Returns a fake Response object for things that expect one
  return {
    ok: true,
    json: async () => data,
    text: async () => JSON.stringify(data),
  };
}

export { BASE_URL, request, apiFetch };
