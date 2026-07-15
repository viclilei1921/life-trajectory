import { invoke } from '@tauri-apps/api/core';
import type { VaultStatus } from './type';

/**
 * @description 获取 Vault 状态
 * @returns Vault 状态
 */
export async function getVaultStatus() {
  return await invoke<VaultStatus>('vault_get_status');
}

/**
 * @description 解锁 Vault
 * @param password - Vault 密码
 * @returns Vault 状态
 */
export async function unlockVault(password: string) {
  return await invoke<VaultStatus>('vault_unlock', { password });
}

/**
 * @description 锁定 Vault
 * @returns Vault 状态
 */
export async function lockVault() {
  return await invoke<VaultStatus>('vault_lock');
}

/**
 * @description 设置 Vault 密码
 * @param password - Vault 密码
 * @returns Vault 状态
 */
export async function setupVaultPassword(password: string) {
  return await invoke<VaultStatus>('vault_setup_password', { password });
}

/**
 * @description 修改 Vault 密码
 * @param oldPassword - 旧密码
 * @param newPassword - 新密码
 */
export async function changeVaultPassword(oldPassword: string, newPassword: string) {
  return await invoke<void>('vault_change_password', { old: oldPassword, new: newPassword });
}
