import { defineStore } from "pinia";
import { ref } from "vue";
import { LinkItem, LinkStatus } from "./type";

export const useLinkStore = defineStore("link", () => {
  const list = ref<LinkItem[]>([
    {
      id: 1,
      name: "test1test1test1test1test1test1test1",
      ip: "192.168.1.1192.168.1.1192.168.1.1192.168.1.1",
      status: LinkStatus.INFO,
    },
    {
      id: 2,
      name: "test2",
      ip: "192.168.1.2",
      status: LinkStatus.SUCCESS,
    },
    {
      id: 3,
      name: "test3",
      ip: "192.168.1.3",
      status: LinkStatus.CONNECTING,
    },
    {
      id: 4,
      name: "test4",
      ip: "192.168.1.4",
      status: LinkStatus.INFO,
    },
    {
      id: 5,
      name: "test5",
      ip: "192.168.1.5",
      status: LinkStatus.SUCCESS,
    },
    {
      id: 6,
      name: "test6",
      ip: "192.168.1.6",
      status: LinkStatus.CONNECTING,
    },
    {
      id: 7,
      name: "test7",
      ip: "192.168.1.7",
      status: LinkStatus.INFO,
    },
    {
      id: 8,
      name: "test8",
      ip: "192.168.1.8",
      status: LinkStatus.SUCCESS,
    },
    {
      id: 9,
      name: "test9",
      ip: "192.168.1.9",
      status: LinkStatus.CONNECTING,
    },
    {
      id: 10,
      name: "test10",
      ip: "192.168.1.10",
      status: LinkStatus.INFO,
    },
  ]);
  // 选中的link
  const currentLinkItem = ref<LinkItem | undefined>();
  // 通过id设置选中的link
  const setCurrentLinkItemById = (id: string | number) => {
    currentLinkItem.value = list.value.find((item) => item.id === id);
  };
  // 通过linkItem设置选中的link
  const setCurrentLinkItem = (linkItem: LinkItem) => {
    currentLinkItem.value = linkItem;
  };
  // 清除选中的link
  const clearCurrentLinkItem = () => {
    currentLinkItem.value = undefined;
  };
  return {
    list,
    currentLinkItem,
    setCurrentLinkItemById,
    setCurrentLinkItem,
    clearCurrentLinkItem,
  };
});
