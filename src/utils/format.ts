/**
 * 单位类型定义
 */
type UnitType = "B" | "KB" | "MB" | "GB" | "TB";

const BYTE_FACTOR = 1024;
const MB_FACTOR = BYTE_FACTOR * BYTE_FACTOR;
const GB_FACTOR = MB_FACTOR * BYTE_FACTOR;
const TB_FACTOR = GB_FACTOR * BYTE_FACTOR;

/**
 * 单位配置
 */
const UNITS: { [key in UnitType]: { factor: number; label: string } } = {
  B: { factor: 1, label: "B" },
  KB: { factor: BYTE_FACTOR, label: "KB" },
  MB: { factor: MB_FACTOR, label: "MB" },
  GB: { factor: GB_FACTOR, label: "GB" },
  TB: { factor: TB_FACTOR, label: "TB" },
};

/**
 * 通用格式化函数
 * @param value 数值
 * @param fromUnit 起始单位，意思是传入的值的是哪个单位，默认是B
 * @param precision 小数位数，默认1位，比如1.23MB，precision为1，则返回1.2MB
 * @returns 格式化后的字符串
 */
export const formatValue = (
  value: number,
  fromUnit: UnitType = "B",
  precision: number = 1
): string => {
  if (value === 0) return `0 ${fromUnit}`;

  const unitKeys: UnitType[] = ["B", "KB", "MB", "GB", "TB"];
  const startIndex = unitKeys.indexOf(fromUnit);

  // 将值转换为字节
  const bytes = value * UNITS[fromUnit].factor;

  // 从起始单位开始查找合适的单位
  for (let i = startIndex; i < unitKeys.length; i++) {
    const unit = unitKeys[i];
    const nextUnit = unitKeys[i + 1];

    // 如果没有下一个单位，或者下一个单位的值小于1，则使用当前单位
    if (!nextUnit || bytes / UNITS[nextUnit].factor < 1) {
      const convertedValue = bytes / UNITS[unit].factor;
      return `${convertedValue.toFixed(precision)} ${unit}`;
    }
  }

  // 兜底返回MB
  return `${(bytes / UNITS.MB.factor).toFixed(precision)} MB`;
};

/**
 * 格式化字节数为可读的字符串
 * @param bytes 字节数
 * @returns 格式化后的字符串，如 "1.2 GB", "512 MB", "256 KB"
 */
export const formatBytes = (bytes: number): string => {
  return formatValue(bytes, "B", 1);
};

/**
 * 格式化MB数为可读的字符串
 * @param mb MB数
 * @returns 格式化后的字符串，如 "1.2 GB", "512 MB"
 */
export const formatMB = (mb: number): string => {
  return formatValue(mb, "MB", 0);
};

/**
 * 格式化交换分区使用情况
 * @param swap 交换分区信息
 * @returns 格式化后的字符串，如 "584MB/2048MB"
 */
export const formatSwapUsage = (swap: any): string => {
  if (!swap || swap.total === 0) return "0MB/0MB";

  const used = Math.round(swap.used);
  const total = Math.round(swap.total);

  // 交换分区优先显示MB格式，除非超过1GB才显示GB
  const usedStr = used >= 1024 ? formatValue(used, "KB", 1) : `${used}MB`;
  const totalStr = total >= 1024 ? formatValue(total, "KB", 1) : `${total}MB`;

  return `${usedStr}/${totalStr}`;
};
