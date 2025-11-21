<template>
  <div
    class="link-item b-l-4 b-l-solid b-l-transparent"
    :class="{
      '!bg-green-50 !b-l-green-500': linkStore.currentLinkItem?.id === id,
    }"
    @click="linkStore.setCurrentLinkItemById(id)"
  >
    <div class="link-item-content">
      <div class="link-icon-wrapper">
        <StatusDot :status="status" />
      </div>
      <div class="link-info">
        <div class="link-name">
          <n-ellipsis>{{ name }}</n-ellipsis>
        </div>
        <div class="link-url">
          <n-ellipsis>{{ ip }}</n-ellipsis>
        </div>
      </div>
    </div>
    <div class="link-operation">
      <!-- 断开 -->
      <n-popover>
        <template #trigger>
          <n-button size="small" text>
            <template #icon>
              <n-icon :size="14">
                <component :is="DisconnectOutlined" />
              </n-icon>
            </template>
          </n-button>
        </template>
        <span>断开连接</span>
      </n-popover>
      <!-- 链接设置 -->
      <n-popover>
        <template #trigger>
          <n-button size="small" text>
            <template #icon>
              <n-icon :size="14">
                <component :is="ControlOutlined" />
              </n-icon>
            </template>
          </n-button>
        </template>
        <span>链接设置</span>
      </n-popover>
      <!-- 删除 -->
      <n-popover>
        <template #trigger>
          <n-button size="small" text>
            <template #icon>
              <n-icon :size="14">
                <component :is="DeleteRowOutlined" />
              </n-icon>
            </template>
          </n-button>
        </template>
        <span>删除</span>
      </n-popover>
    </div>
  </div>
</template>

<script setup lang="ts">
import StatusDot from "./StatusDot.vue";
import {
  ControlOutlined,
  DisconnectOutlined,
  DeleteRowOutlined,
} from "@vicons/antd";
import { useLinkStore } from "@/store/modules/LinkStore/index";

interface Props {
  name: string;
  ip: string;
  id: string | number;
  status?: "info" | "success" | "connecting";
}

const { status = "info", ip, id } = defineProps<Props>();
const linkStore = useLinkStore();
</script>

<style scoped lang="scss">
.link-item {
  line-height: normal;
  padding: 6px 4px 6px 8px;
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
    .link-operation {
      width: auto;
    }
  }
  &:last-child {
    border-bottom: none;
  }
}

.link-item-content {
  flex: 1 0;
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  overflow: hidden;
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
    line-height: 1.2;
    transition: color 0.2s ease;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
    max-width: 100%;
  }

  .link-url {
    font-size: 12px;
    color: rgb(107, 114, 128);
    line-height: 1.2;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
      "Liberation Mono", monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
    max-width: 100%;
  }
}

.link-operation {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  min-width: 0;
  width: 0;
  overflow-x: hidden;
  transition: width 0.2s ease;
}
</style>
