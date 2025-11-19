<template>
  <n-layout has-sider class="layout-container">
    <n-layout-sider
      bordered
      show-trigger
      collapse-mode="width"
      :collapsed-width="siderConfig.collapsedWidth"
      :width="siderConfig.width"
      :native-scrollbar="false"
    >
      <HardwareInformation />
    </n-layout-sider>
    <n-layout-content>
      <div class="w-full h-full flex flex-col">
        <div class="flex-1 flex-shrink-0">
          <router-view></router-view>
        </div>
        <n-layout-footer class="flex-shrink-0">
          <div class="px-4 py-1 text-right">Footer Footer Footer</div>
        </n-layout-footer>
      </div>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { BookOutline, PersonOutline, WineOutline } from "@vicons/ionicons5";
import { renderIcon } from "../../utils/renderIcon";
import HardwareInformation from "../../components/HardwareInformation/index.vue";

const siderConfig = {
  width: "clamp(180px, 18vw, 240px)",
  collapsedWidth: 64,
};

const menuOptions = [
  {
    label: "且听风吟",
    key: "hear-the-wind-sing",
    icon: renderIcon(BookOutline),
  },
  {
    label: "1973年的弹珠玩具",
    key: "pinball-1973",
    icon: renderIcon(BookOutline),
    disabled: true,
    children: [
      {
        label: "鼠",
        key: "rat",
      },
    ],
  },
  {
    label: "寻羊冒险记",
    key: "a-wild-sheep-chase",
    disabled: true,
    icon: renderIcon(BookOutline),
  },
  {
    label: "舞，舞，舞",
    key: "dance-dance-dance",
    icon: renderIcon(BookOutline),
    children: [
      {
        type: "group",
        label: "人物",
        key: "people",
        children: [
          {
            label: "叙事者",
            key: "narrator",
            icon: renderIcon(PersonOutline),
          },
          {
            label: "羊男",
            key: "sheep-man",
            icon: renderIcon(PersonOutline),
          },
        ],
      },
      {
        label: "饮品",
        key: "beverage",
        icon: renderIcon(WineOutline),
        children: [
          {
            label: "威士忌",
            key: "whisky",
          },
        ],
      },
      {
        label: "食物",
        key: "food",
        children: [
          {
            label: "三明治",
            key: "sandwich",
          },
        ],
      },
      {
        label: "过去增多，未来减少",
        key: "the-past-increases-the-future-recedes",
      },
    ],
  },
];
</script>
<style scoped lang="scss">
.layout-container {
  width: 100%;
  height: 100%;
}
</style>
