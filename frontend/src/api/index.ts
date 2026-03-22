import { request } from "./client";

export type IndexStatus = {
  status: string;
  progress?: {
    processed_items?: number;
    total_items?: number;
    elapsed_seconds?: number;
    eta_seconds?: number | null;
    skipped_items?: number;
    skip_reasons?: string[];
    mode?: 'incremental' | 'full';
    error?: string | null;
  };
};

export async function indexLibrary(): Promise<{ msg?: string }>{
  return request("/api/index_library", { method: "POST" });
}

export async function getIndexStatus(): Promise<IndexStatus> {
  return request("/api/index_status");
}

export async function cancelIndexing(): Promise<{ msg?: string }>{
  return request("/api/index_cancel", { method: "POST" });
}
