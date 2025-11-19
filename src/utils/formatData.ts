/**
 * 格式化数据量（字节）
 * @param bytes 字节数
 * @param decimals 小数位数，默认2位
 * @returns 格式化后的字符串，如 "1.5 GB"
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return "0 B";
  
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return (
    Math.round((bytes / Math.pow(k, i)) * Math.pow(10, decimals)) /
    Math.pow(10, decimals)
  ).toFixed(decimals) + " " + sizes[i];
}

/**
 * 格式化传输速度（字节/秒）
 * @param bytesPerSec 每秒字节数
 * @returns 格式化后的字符串，如 "1.5 MB/s"
 */
export function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec < 1024) {
    return bytesPerSec.toFixed(0) + " B/s";
  }
  if (bytesPerSec < 1024 * 1024) {
    return (bytesPerSec / 1024).toFixed(1) + " KB/s";
  }
  if (bytesPerSec < 1024 * 1024 * 1024) {
    return (bytesPerSec / (1024 * 1024)).toFixed(2) + " MB/s";
  }
  return (bytesPerSec / (1024 * 1024 * 1024)).toFixed(2) + " GB/s";
}

