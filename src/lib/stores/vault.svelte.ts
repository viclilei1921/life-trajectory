import type { VaultStatus } from '$lib/bridge';
import { changeVaultPassword, getVaultStatus, lockVault, setupVaultPassword, unlockVault } from '$lib/bridge';
import { formatAppError } from '$lib/utils/errors';

/**
 * @description Vault 存储
 */
class VaultStore {
  /** @description Vault 状态 */
  status = $state<VaultStatus | null>(null);
  /** @description 加载状态 */
  loading = $state(false);
  /** @description 错误信息 */
  error = $state<string | null>(null);

  /** @description 是否需要解锁 */
  needsUnlock = $derived(this.status?.mode === 'password' && !this.status?.isUnlocked);

  /**
   * @description 刷新 Vault 状态
   */
  async refresh() {
    this.loading = true;
    this.error = null;
    try {
      this.status = await getVaultStatus();
    } catch (e) {
      this.error = formatAppError(e);
    } finally {
      this.loading = false;
    }
  }

  /**
   * @description 解锁 Vault
   * @param password - 密码
   */
  async unlock(password: string) {
    this.loading = true;
    this.error = null;
    try {
      this.status = await unlockVault(password);
    } catch (e) {
      this.error = formatAppError(e);
      throw e;
    } finally {
      this.loading = false;
    }
  }

  /**
   * @description 锁定 Vault
   */
  async lock() {
    this.loading = true;
    this.error = null;
    try {
      this.status = await lockVault();
    } catch (e) {
      this.error = formatAppError(e);
      throw e;
    } finally {
      this.loading = false;
    }
  }

  /**
   * @description 设置 Vault 密码
   * @param password - 密码
   */
  async setupPassword(password: string) {
    this.loading = true;
    this.error = null;
    try {
      this.status = await setupVaultPassword(password);
    } catch (e) {
      this.error = formatAppError(e);
      throw e;
    } finally {
      this.loading = false;
    }
  }

  /**
   * @description 修改 Vault 密码
   * @param oldPassword - 旧密码
   * @param newPassword - 新密码
   */
  async changePassword(oldPassword: string, newPassword: string) {
    this.loading = true;
    this.error = null;
    try {
      await changeVaultPassword(oldPassword, newPassword);
    } catch (e) {
      this.error = formatAppError(e);
      throw e;
    } finally {
      this.loading = false;
    }
  }
}

/** @description Vault 存储实例 */
export const vaultStore = new VaultStore();
