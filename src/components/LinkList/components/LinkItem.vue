<template>
  <div
    class="link-item b-r-4 b-r-solid b-r-transparent"
    :class="{
      '!bg-green-50 !b-r-green-500': isActive,
    }"
    @click="handleClick"
  >
    <div class="link-item-content">
      <div class="link-icon-wrapper">
        <StatusDot :status="status" />
      </div>
      <div class="link-info">
        <div class="link-name">{{ name }}</div>
        <div class="link-url">{{ link }}</div>
      </div>
    </div>
    <div class="link-operation">
      <!-- 连接\断开 -->
      <n-button size="small" text>
        <template #icon>
          <n-icon :size="14">
            <component :is="LinkOutlined" />
          </n-icon>
        </template>
      </n-button>
      <n-button size="small" text>
        <template #icon>
          <n-icon :size="14">
            <component :is="SettingOutlined" />
          </n-icon>
        </template>
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import StatusDot from "./StatusDot.vue";
import { SettingOutlined, LinkOutlined } from "@vicons/antd";

interface Props {
  name: string;
  link: string;
  status?: "info" | "success" | "connecting";
  isActive?: boolean;
}

// const props = withDefaults(defineProps<Props>(), {
//   status: "info",
//   isActive: false,
// });
const { status = "info", isActive = false, link } = defineProps<Props>();
const handleClick = () => {
  const url = link.startsWith("http") ? link : `http://${link}`;
  window.open(url, "_blank");
};
</script>

<style scoped lang="scss">
.link-item {
  line-height: normal;
  padding: 6px 4px 6px 12px;
  background: var(--n-card-color);
  cursor: pointer;
  transition: all 0.2s ease;
  width: 100%;
  box-sizing: border-box;
  border-bottom: 1px solid var(--n-border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  &:hover {
    background: rgba(0, 123, 255, 0.08);

    .link-name {
      color: rgb(55, 65, 81);
    }
  }
}

.link-item-content {
  flex: 1 0;
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  .link-icon-wrapper {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .link-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .link-name {
    font-size: 14px;
    font-weight: 500;
    color: rgb(55, 65, 81);
    line-height: 1.4;
    transition: color 0.2s ease;
  }

  .link-url {
    font-size: 12px;
    color: rgb(107, 114, 128);
    line-height: 1.4;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
      "Liberation Mono", monospace;
  }
}

.link-operation {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}
</style>
