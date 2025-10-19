<template>
  <div class="bg-white border-b border-gray-200 flex items-center">
    <!-- 标签页列表 -->
    <div class="flex-1 flex overflow-x-auto">
      <div 
        v-for="tab in tabs" 
        :key="tab.id"
        :class="{ 'tab-active': tab.active }"
        class="tab-item"
        @click="selectTab(tab)"
      >
        <!-- 连接图标 -->
        <n-icon size="14" class="mr-1.5">
          <CodeOutlined v-if="tab.type === 'terminal'" />
          <FileTextOutlined v-else-if="tab.type === 'file'" />
        </n-icon>
        
        <!-- 标签页标题 -->
        <span class="text-sm font-medium mr-1.5 whitespace-nowrap overflow-hidden text-ellipsis">
          {{ tab.title }}
        </span>
        
        <!-- 连接状态指示器 -->
        <div class="mr-1.5">
          <ConnectionStatus :connected="tab.connected" size="tiny" />
        </div>
        
        <!-- 关闭按钮 -->
        <n-button 
          quaternary 
          circle 
          size="tiny"
          @click.stop="closeTab(tab)"
        >
          <template #icon>
            <n-icon><CloseOutlined /></n-icon>
          </template>
        </n-button>
      </div>
    </div>
    
    <!-- 标签页操作按钮 -->
    <div class="flex items-center px-2 border-l border-gray-200">
      <!-- 新建连接按钮 -->
      <n-button 
        quaternary 
        circle 
        size="small"
        @click="showNewConnection"
        class="mr-1"
      >
        <template #icon>
          <n-icon><PlusOutlined /></n-icon>
        </template>
      </n-button>
      
      <!-- 文件管理器按钮 -->
      <n-button 
        quaternary 
        circle 
        size="small"
        @click="showFileManager"
        class="mr-1"
      >
        <template #icon>
          <n-icon><FolderOutlined /></n-icon>
        </template>
      </n-button>
      
      <!-- 标签页菜单 -->
      <n-dropdown :options="tabMenuOptions" @select="handleTabMenuSelect">
        <n-button quaternary circle size="small">
          <template #icon>
            <n-icon><MoreOutlined /></n-icon>
          </template>
        </n-button>
      </n-dropdown>
    </div>
  </div>
</template>

<script setup lang="ts">
import { h, ref } from 'vue'
import { 
  CodeOutlined, 
  FileTextOutlined, 
  CloseOutlined, 
  PlusOutlined, 
  FolderOutlined, 
  MoreOutlined,
  SettingOutlined 
} from '@vicons/antd'
import ConnectionStatus from './ConnectionStatus.vue'

// 标签页数据接口
interface Tab {
  id: string
  title: string
  type: 'terminal' | 'file'
  connected: boolean
  active: boolean
  connectionId?: string
}

// 响应式数据
const tabs = ref<Tab[]>([
  {
    id: '1',
    title: '开发服务器',
    type: 'terminal',
    connected: true,
    active: true,
    connectionId: '2'
  },
  {
    id: '2',
    title: '生产服务器',
    type: 'terminal',
    connected: false,
    active: false,
    connectionId: '1'
  }
])

// 标签页菜单选项
const tabMenuOptions = [
  {
    label: '关闭所有标签页',
    key: 'closeAll',
    icon: () => h('n-icon', null, { default: () => h(CloseOutlined) })
  },
  {
    label: '关闭其他标签页',
    key: 'closeOthers',
    icon: () => h('n-icon', null, { default: () => h(CloseOutlined) })
  },
  {
    label: '复制当前标签页',
    key: 'duplicate',
    icon: () => h('n-icon', null, { default: () => h(PlusOutlined) })
  },
  {
    type: 'divider'
  },
  {
    label: '标签页设置',
    key: 'settings',
    icon: () => h('n-icon', null, { default: () => h(SettingOutlined) })
  }
]

// 方法
const selectTab = (tab: Tab) => {
  // 取消其他标签页的激活状态
  tabs.value.forEach(t => {
    t.active = t.id === tab.id
  })
  console.log('选择标签页:', tab.title)
}

const closeTab = (tab: Tab) => {
  const index = tabs.value.findIndex(t => t.id === tab.id)
  if (index > -1) {
    const wasActive = tab.active
    tabs.value.splice(index, 1)
    
    // 如果关闭的是当前激活的标签页，激活下一个标签页
    if (wasActive && tabs.value.length > 0) {
      const nextIndex = Math.min(index, tabs.value.length - 1)
      tabs.value[nextIndex].active = true
    }
  }
  console.log('关闭标签页:', tab.title)
}

const showNewConnection = () => {
  console.log('显示新建连接对话框')
}

const showFileManager = () => {
  // 添加文件管理器标签页
  const newTab: Tab = {
    id: Date.now().toString(),
    title: '文件管理器',
    type: 'file',
    connected: false,
    active: true
  }
  
  // 取消其他标签页的激活状态
  tabs.value.forEach(tab => {
    tab.active = false
  })
  
  tabs.value.push(newTab)
  console.log('打开文件管理器')
}

const handleTabMenuSelect = (key: string) => {
  switch (key) {
    case 'closeAll':
      if (confirm('确定要关闭所有标签页吗？')) {
        tabs.value = []
      }
      break
    case 'closeOthers':
      const activeTab = tabs.value.find(tab => tab.active)
      if (activeTab) {
        tabs.value = [activeTab]
      }
      break
    case 'duplicate':
      const currentTab = tabs.value.find(tab => tab.active)
      if (currentTab) {
        const newTab: Tab = {
          id: Date.now().toString(),
          title: currentTab.title + ' (副本)',
          type: currentTab.type,
          connected: false,
          active: true,
          connectionId: currentTab.connectionId
        }
        
        // 取消其他标签页的激活状态
        tabs.value.forEach(tab => {
          tab.active = false
        })
        
        tabs.value.push(newTab)
      }
      break
    case 'settings':
      console.log('显示标签页设置')
      break
  }
}
</script>

<style scoped>
@import "tailwindcss";

/* 标签页项样式 */
.tab-item {
  @apply flex items-center px-3 py-2 border-r border-gray-200 cursor-pointer transition-all duration-200 min-w-0 flex-shrink-0;
}

/* 悬停效果 */
.tab-item:hover {
  @apply bg-gray-100;
}

/* 激活状态 */
.tab-active {
  @apply bg-blue-50 border-b-2 border-green-500;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  height: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>