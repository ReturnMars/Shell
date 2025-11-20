<template>
  <div class="link-list">
    <div class="link-header">
      <div class="header-content">
        <div class="header-title-wrapper">
          <span class="header-title">链接列表</span>
        </div>
        <n-button size="small" text circle>
          <template #icon>
            <n-icon :size="16">
              <component :is="AddIcon" />
            </n-icon>
          </template>
        </n-button>
      </div>
    </div>
      <div class="link-content">
        <LinkItem
          v-for="item in linkList"
          :key="item.name"
          :name="item.name"
          :link="item.link"
          :status="item.status"
        />
      </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { AddOutline } from "@vicons/ionicons5";
import LinkItem from "./components/LinkItem.vue";

const AddIcon = AddOutline;

const linkList = ref<any[]>([
  {
    name: "test1",
    link: "192.168.1.1",
    status: "info" as const,
  },
  {
    name: "test2",
    link: "192.168.1.2",
    status: "success" as const,
  },
  {
    name: "test3",
    link: "192.168.1.3",
    status: "connecting" as const,
  },
  {
    name: "test4",
    link: "192.168.1.4",
    status: "info" as const,
  },
  {
    name: "test5",
    link: "192.168.1.5",
    status: "success" as const,
  },
  {
    name: "test6",
    link: "192.168.1.6",
    status: "connecting" as const,
  },
  {
    name: "test7",
    link: "192.168.1.7",
    status: "info" as const,
  },
  {
    name: "test8",
    link: "192.168.1.8",
    status: "success" as const,
  },
  {
    name: "test9",
    link: "192.168.1.9",
    status: "connecting" as const,
  },
  {
    name: "test10",
    link: "192.168.1.10",
    status: "info" as const,
  },
]);
</script>
<style scoped lang="scss">
.link-list {
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  padding: 8px 0;
  overflow: auto;
  display: flex;
  flex-direction: column;
  .link-header {
    padding: 0 8px 0 8px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--n-border-color);
    .header-content {
      display: flex;
      justify-content: space-between;
      align-items: center;
      overflow: auto;
    }
    .header-title-wrapper {
      display: flex;
      align-items: center;
      gap: 6px;
    }
    .header-icon {
      color: rgb(107, 114, 128);
    }

    .header-title {
      font-size: 14px;
      font-weight: 500;
      color: rgb(75, 85, 99);
    }
  }

  .link-content {
    display: flex;
    flex-direction: column;
    flex: 1 0;
    overflow: auto;
  }
}
</style>
