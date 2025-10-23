<template>
  <!-- 状态圆点模式 -->
  <div
    v-if="statusOnly"
    :class="[
      'rounded-full transition-colors',
      statusOnlySizeClasses[size],
      loading ? 'bg-blue-500 animate-pulse' : connected ? 'bg-green-500' : 'bg-gray-400',
    ]"
    :title="loading ? '连接中...' : connected ? '已连接' : '未连接'"
  ></div>

  <!-- 完整状态标签模式 -->
  <div
    v-else
    :class="[
      'inline-flex items-center gap-1.5 px-2 py-1 rounded-full text-xs font-medium transition-colors',
      sizeClasses[size],
      loading
        ? 'bg-blue-100 text-blue-800 border border-blue-200 animate-pulse'
        : connected
        ? 'bg-green-100 text-green-800 border border-green-200'
        : 'bg-gray-100 text-gray-600 border border-gray-200',
    ]"
  >
    <CheckCircleOutlined
      v-if="connected && !loading"
      :class="iconSizeClasses[size]"
      class="text-green-600"
    />
    <InfoCircleOutlined
      v-else-if="!loading"
      :class="iconSizeClasses[size]"
      class="text-gray-500"
    />
    <div
      v-else
      :class="[iconSizeClasses[size], 'animate-spin']"
      class="text-blue-600"
    >
      ⏳
    </div>
    <span v-if="showText">{{ loading ? "连接中..." : connected ? "已连接" : "未连接" }}</span>
  </div>
</template>

<script setup lang="ts">
import { CheckCircleOutlined, InfoCircleOutlined } from "@vicons/antd";

interface Props {
  connected: boolean | undefined;
  loading?: boolean;
  size?: "small" | "medium" | "large" | "tiny";
  showText?: boolean;
  statusOnly?: boolean;
}

withDefaults(defineProps<Props>(), {
  size: "small",
  showText: true,
  statusOnly: false,
});

// 尺寸样式映射
const sizeClasses = {
  tiny: "px-.5 py-0.5 text-xs",
  small: "px-1 py-1 text-xs",
  medium: "px-1.5 py-.5 text-sm",
  large: "px-2 py-1.5 text-sm",
};

// 图标尺寸映射
const iconSizeClasses = {
  tiny: "w-3 h-3",
  small: "w-3.5 h-3.5",
  medium: "w-4 h-4",
  large: "w-4.5 h-4.5",
};

// 状态圆点尺寸映射
const statusOnlySizeClasses = {
  tiny: "w-1.5 h-1.5",
  small: "w-2 h-2",
  medium: "w-2.5 h-2.5",
  large: "w-3 h-3",
};
</script>
