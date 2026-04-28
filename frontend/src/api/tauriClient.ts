import { invoke } from '@tauri-apps/api/core';

/**
 * Tauri API Client Bridge
 * 
 * This module replaces the traditional HTTP fetch requests to the Python backend
 * with native IPC calls to the Rust backend.
 */

export async function healthCheck(): Promise<string> {
  try {
    return await invoke<string>('health_check');
  } catch (error) {
    console.error('Failed to perform health check via Tauri:', error);
    throw error;
  }
}

// Additional IPC methods will be added here as the Rust backend is built out.
// Example:
// export async function searchItems(authors: string, titles: string, dates: string) {
//   return await invoke('search_items', { authors, titles, dates });
// }
