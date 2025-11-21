<template>
  <div class="link-list">
    <div class="link-header">
      <div class="header-content">
        <div class="header-title-wrapper">
          <n-text>链接列表</n-text>
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
        v-for="item in linkStore.list"
        :key="item.id"
        :name="item.name"
        :ip="item.ip"
        :status="item.status"
        :id="item.id"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
import { AddOutline } from "@vicons/ionicons5";
import LinkItem from "./components/LinkItem.vue";
import { useLinkStore } from "../../store/modules/LinkStore";
const linkStore = useLinkStore();
const AddIcon = AddOutline;
</script>
<style scoped lang="scss">
.link-list {
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  padding-top: 8px;
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
  }

  .link-content {
    display: flex;
    flex-direction: column;
    flex: 1 0;
    overflow: auto;
  }
  border-bottom: 1px solid var(--n-border-color);
}
</style>
