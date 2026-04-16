// src/backendClient.ts

// Thin compatibility layer so older imports from `src/backendClient.ts`
// keep working while the codebase uses `src/api/*` modules.
import { chat as postChat } from "./api/chat";
import { indexLibrary as apiIndexLibrary, getIndexStatus } from "./api/index";
export { getIndexStatus };

export type ChatResponse = import("./types/api").ChatResponse;

export async function indexLibrary(): Promise<any> {
  return apiIndexLibrary() as any;
}

/**
 * Compatibility wrapper: accept (question, itemIds?) to match older usage.
 * Delegates to the POST `/chat` API which accepts JSON body.
 */
export async function chat(question: string, itemIds?: string[]): Promise<ChatResponse> {
  // @ts-ignore
  const z = window.Zotero;
  const body: Record<string, unknown> = { query: question };
  if (itemIds && itemIds.length) body.item_ids = itemIds;
  return postChat(body as { query: string; item_ids?: string[] });
}
