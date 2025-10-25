// 硬件信息相关类型定义

// 硬件信息基础接口
export interface HardwareInfo {
  cpu: CpuInfo;
  memory: MemoryInfo;
  storage: StorageInfo[];
  network: NetworkInfo;
  timestamp: string; // ISO 8601 格式的时间字符串
}

// CPU 信息
export interface CpuInfo {
  model: string; // 型号
  cores: number; // 核心数
  usage: number; // 使用率 0-100
  frequency?: number; // 频率 (MHz)
  temperature?: number; // 温度 (摄氏度)
}

// 内存信息
export interface MemoryInfo {
  total: number; // 总容量 (MB)
  used: number; // 已使用 (MB)
  free: number; // 空闲 (MB)
  usage: number; // 使用率 0-100
  swap?: SwapInfo; // 交换分区
}

// 交换分区信息
export interface SwapInfo {
  total: number; // 总容量 (MB)
  used: number; // 已使用 (MB)
  free: number; // 空闲 (MB)
  usage: number; // 使用率 0-100
}

// 硬盘信息
export interface StorageInfo {
  device: string; // 设备名 (如 /dev/sda1)
  mount_point: string; // 挂载点 (如 /, /home)
  filesystem: string; // 文件系统类型 (如 ext4, xfs)
  total: number; // 总容量 (MB)
  used: number; // 已使用 (MB)
  free: number; // 空闲 (MB)
  usage: number; // 使用率 0-100
  type: "ssd" | "hdd"; // 类型 (ssd, hdd)
}

// 网络信息
export interface NetworkInfo {
  interfaces: NetworkInterface[];
  total_rx: number; // 总接收字节数
  total_tx: number; // 总发送字节数
  rx_speed: number; // 接收速度 (MB/s)
  tx_speed: number; // 发送速度 (MB/s)
}

// 网络接口
export interface NetworkInterface {
  name: string; // 接口名
  status: string; // 状态 (up, down)
  rx: number; // 接收字节数
  tx: number; // 发送字节数
  rx_speed: number; // 接收速度 (MB/s)
  tx_speed: number; // 发送速度 (MB/s)
}

// 硬件监控状态
export interface HardwareMonitorState {
  hardwareInfo: HardwareInfo | null;
  loading: boolean;
  error: string | null;
  lastUpdate: number | null;
  autoRefresh: boolean;
  refreshInterval: number; // 刷新间隔 (毫秒)
}

// 图表配置选项
export interface ChartOptions {
  width?: number;
  height?: number;
  theme?: string;
  animation?: boolean;
  showLegend?: boolean;
  showTooltip?: boolean;
}

// 进度条图表数据
export interface ProgressChartData {
  value: number; // 当前值
  max: number; // 最大值
  label: string; // 标签
  unit: string; // 单位
  color?: string; // 颜色
}

// 环形图数据
export interface RingChartData {
  name: string; // 名称
  value: number; // 值
  color?: string; // 颜色
}

// 柱状图数据
export interface BarChartData {
  name: string; // 名称
  value: number; // 值
  color?: string; // 颜色
}
