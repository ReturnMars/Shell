// 硬件信息相关类型定义

// 硬件信息基础接口
export interface HardwareInfo {
  cpu: CpuInfo;
  memory: MemoryInfo;
  storage: StorageInfo[];
  network?: NetworkInfo;
  timestamp: number;
}

// CPU 信息
export interface CpuInfo {
  usage: number;        // 使用率 0-100
  cores: number;        // 核心数
  model: string;        // 型号
  temperature?: number; // 温度 (摄氏度)
  frequency?: number;   // 频率 (MHz)
}

// 内存信息
export interface MemoryInfo {
  total: number;        // 总容量 (MB)
  used: number;         // 已使用 (MB)
  free: number;         // 空闲 (MB)
  usage: number;        // 使用率 0-100
  swap?: SwapInfo;      // 交换分区
}

// 交换分区信息
export interface SwapInfo {
  total: number;        // 总容量 (MB)
  used: number;         // 已使用 (MB)
  free: number;         // 空闲 (MB)
  usage: number;        // 使用率 0-100
}

// 硬盘信息
export interface StorageInfo {
  name: string;         // 设备名
  total: number;        // 总容量 (GB)
  used: number;         // 已使用 (GB)
  free: number;         // 空闲 (GB)
  usage: number;        // 使用率 0-100
  type: 'ssd' | 'hdd';  // 类型
  mountPoint: string;   // 挂载点
  readSpeed?: number;   // 读取速度 (MB/s)
  writeSpeed?: number;  // 写入速度 (MB/s)
}

// 网络信息
export interface NetworkInfo {
  interfaces: NetworkInterface[];
  totalRx: number;      // 总接收 (MB)
  totalTx: number;      // 总发送 (MB)
  rxSpeed: number;      // 接收速度 (MB/s)
  txSpeed: number;      // 发送速度 (MB/s)
}

// 网络接口
export interface NetworkInterface {
  name: string;         // 接口名
  rx: number;           // 接收 (MB)
  tx: number;           // 发送 (MB)
  rxSpeed: number;      // 接收速度 (MB/s)
  txSpeed: number;      // 发送速度 (MB/s)
  status: 'up' | 'down'; // 状态
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
  value: number;        // 当前值
  max: number;          // 最大值
  label: string;        // 标签
  unit: string;         // 单位
  color?: string;       // 颜色
}

// 环形图数据
export interface RingChartData {
  name: string;         // 名称
  value: number;        // 值
  color?: string;       // 颜色
}

// 柱状图数据
export interface BarChartData {
  name: string;         // 名称
  value: number;        // 值
  color?: string;       // 颜色
}
