/**
 * Unix 秒时间戳 → 本地 datetime-local 输入值
 */
export function toDatetimeLocal(unixSeconds: number): string {
  const date = new Date(unixSeconds * 1000);
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/**
 * datetime-local 输入值 → Unix 秒时间戳
 */
export function fromDatetimeLocal(value: string): number {
  return Math.floor(new Date(value).getTime() / 1000);
}

/**
 * Unix 秒时间戳 → 本地可读字符串
 */
export function formatHappenedAt(unixSeconds: number): string {
  return new Date(unixSeconds * 1000).toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

/**
 * 当前时间的 Unix 秒时间戳
 */
export function nowUnix(): number {
  return Math.floor(Date.now() / 1000);
}
