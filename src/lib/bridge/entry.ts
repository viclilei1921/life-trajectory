import { invoke } from '@tauri-apps/api/core';
import type { CreateEntryArgs, EntryResponse, EntrySummary } from './type';

/**
 * @description 创建 Entry
 * @param args - 创建 Entry 参数
 * @returns Entry 响应
 */
export async function createEntry(args: CreateEntryArgs) {
  return await invoke<EntryResponse>('create_entry', {
    title: args.title,
    content: args.content,
    happenedAt: args.happenedAt
  });
}

/**
 * @description 获取 Entry
 * @param id - Entry ID
 * @returns Entry 响应
 */
export async function getEntry(id: string) {
  return await invoke<EntryResponse>('get_entry', { id });
}

/**
 * @description 列出条目摘要（UI 先行：尚无 list IPC，返回空数组；后续替换为 invoke）
 */
export async function listEntries(): Promise<EntrySummary[]> {
  // TODO: replace with invoke<EntrySummary[]>('list_entries') when backend is ready
  return [];
}
