<template>
  <div>
    <template v-if="'trigger' in $slots">
      <div class="trigger-button" @click="showModal = true">
        <slot name="trigger"></slot>
      </div>
    </template>
    <n-modal
      v-model:show="showModal"
      preset="dialog"
      title="SSH链接配置"
      @after-leave="closeModal"
    >
      <n-form
        ref="formRef"
        :model="formData"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
      >
        <n-form-item label="链接名称" path="name">
          <n-input v-model:value="formData.name" placeholder="请输入链接名称" />
        </n-form-item>

        <n-form-item label="主机地址" path="host">
          <n-input
            v-model:value="formData.host"
            placeholder="请输入主机IP或域名"
          />
        </n-form-item>

        <n-form-item label="端口" path="port">
          <n-input-number
            v-model:value="formData.port"
            :min="1"
            :max="65535"
            placeholder="22"
          />
        </n-form-item>

        <n-form-item label="用户名" path="username">
          <n-input
            v-model:value="formData.username"
            placeholder="请输入用户名"
          />
        </n-form-item>

        <n-form-item label="认证方式" path="auth_method">
          <n-select
            v-model:value="formData.auth_method"
            :options="authOptions"
          />
        </n-form-item>

        <n-form-item
          v-if="
            formData.auth_method === 'Password' ||
            formData.auth_method === 'Both'
          "
          label="密码"
          path="password"
        >
          <n-input
            v-model:value="formData.password"
            type="password"
            placeholder="请输入密码"
            show-password-on="click"
          />
        </n-form-item>

        <n-form-item
          v-if="
            formData.auth_method === 'PrivateKey' ||
            formData.auth_method === 'Both'
          "
          label="私钥路径"
          path="private_key_path"
        >
          <n-input
            v-model:value="formData.private_key_path"
            placeholder="请输入私钥文件路径"
          />
        </n-form-item>
      </n-form>

      <template #action>
        <div class="flex items-center gap-2">
          <n-button size="small" @click="showModal = false">取消</n-button>
          <!-- update 链接 -->
          <n-button
            v-if="isEdit"
            type="primary"
            size="small"
            @click="handleSave"
          >
            保存
          </n-button>
          <n-button
            :type="getTestButtonType"
            @click="handleTest"
            :loading="testing"
            :disabled="connecting"
            size="small"
          >
            <template #icon v-if="isTested">
              <n-icon>
                <CheckCircleOutlined v-if="testStatus === 'success'" />
                <CloseCircleOutlined v-else-if="testStatus === 'error'" />
              </n-icon>
            </template>
            {{ getTestButtonText }}
          </n-button>
          <n-button
            type="info"
            size="small"
            @click="handleTestCommand"
            :loading="executingCommand"
            :disabled="isExecutingDisabled"
          >
            测试命令
          </n-button>
          <template v-if="!isEdit">
            <n-button
              type="success"
              size="small"
              @click="handleConnect"
              :loading="connecting"
              :disabled="testing || isExecutingDisabled"
            >
              连接服务器
            </n-button>
          </template>
        </div>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import type { FormInst, FormRules } from "naive-ui";
import { useMessage } from "naive-ui";
import { CheckCircleOutlined, CloseCircleOutlined } from "@vicons/antd";
import { Type } from "naive-ui/es/button/src/interface";
import type {
  AuthMethod,
  ConnectionForm,
  ConnectionConfig,
} from "@/stores/connection";
import { useConnectionStore } from "@/stores/connection";

// Props
interface Props {
  show?: boolean;
  connection?: ConnectionConfig;
}

// 获取 props
const props = defineProps<Props>();

// Emits
const emit = defineEmits<{
  connected: [connectionId: string];
  tested: [result: string];
  updated: [connection: ConnectionConfig];
  saved: [connection: ConnectionConfig];
}>();

// 使用 Pinia store
const connectionStore = useConnectionStore();
const message = useMessage();

// 响应式数据
const showModal = defineModel<boolean>("show");
const formRef = ref<FormInst | null>(null);
const testStatus = ref<"idle" | "success" | "error">("idle");
const testing = ref(false);
const connecting = ref(false);
const executingCommand = ref(false);

// 计算属性
const isTested = computed(() => testStatus.value !== "idle");
const isEdit = computed(() => !!props.connection?.id);
const isExecutingDisabled = computed(
  () => testing.value || connecting.value || executingCommand.value
);
// 表单数据
const baseFormData: ConnectionForm = {
  name: "wzd",
  host: "47.109.195.0",
  port: 22,
  username: "root",
  password: "Aioreturn@123",
  private_key_path: "",
  auth_method: "Password" as AuthMethod,
};

// 表单数据类型：编辑时使用 ConnectionConfig，新建时使用 ConnectionForm
type FormData = ConnectionForm | ConnectionConfig;
const formData = ref<FormData>(structuredClone(baseFormData));

// 认证方式选项
const authOptions = [
  { label: "密码认证", value: "Password" },
  { label: "密钥认证", value: "PrivateKey" },
  { label: "混合认证", value: "Both" },
];

// 表单验证规则
const rules: FormRules = {
  name: {
    required: true,
    message: "请输入连接名称",
    trigger: ["input", "blur"],
  },
  host: {
    required: true,
    message: "请输入主机地址",
    trigger: ["input", "blur"],
  },
  port: {
    required: true,
    type: "number",
    message: "请输入有效端口号",
    trigger: ["input", "blur"],
  },
  username: {
    required: true,
    message: "请输入用户名",
    trigger: ["input", "blur"],
  },
  password: {
    required: true,
    message: "请输入密码",
    trigger: ["input", "blur"],
    validator: (_, value) => {
      if (
        (formData.value.auth_method === "Password" ||
          formData.value.auth_method === "Both") &&
        !value
      ) {
        return new Error("密码认证需要提供密码");
      }
      return true;
    },
  },
  private_key_path: {
    required: true,
    message: "请输入私钥路径",
    trigger: ["input", "blur"],
    validator: (_, value) => {
      if (
        (formData.value.auth_method === "PrivateKey" ||
          formData.value.auth_method === "Both") &&
        !value
      ) {
        return new Error("密钥认证需要提供私钥路径");
      }
      return true;
    },
  },
};

// 监听表单数据变化，重置测试状态
watch(
  formData,
  () => {
    if (testStatus.value !== "idle") {
      testStatus.value = "idle";
    }
    testing.value = false;
    connecting.value = false;
    executingCommand.value = false;
  },
  { deep: true }
);

// 获取测试按钮类型
const getTestButtonType = computed(() => {
  if (testing.value) return "info" as Type;
  if (testStatus.value === "success") return "success" as Type;
  if (testStatus.value === "error") return "error" as Type;
  return "primary" as Type;
});

// 获取测试按钮文本
const getTestButtonText = computed(() => {
  if (testing.value) return "测试中...";
  if (testStatus.value === "success") return "测试成功";
  if (testStatus.value === "error") return "测试失败";
  return "测试连接";
});

// 生成链接配置
const generateConfig = async (): Promise<ConnectionConfig> => {
  return {
    id: (await connectionStore.generateUuid()) as string,
    name: formData.value.name,
    host: formData.value.host,
    port: formData.value.port,
    username: formData.value.username,
    password: formData.value.password || null,
    private_key_path: formData.value.private_key_path || null,
    auth_method: formData.value.auth_method,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    connected: false,
    active: false,
  };
};

// 测试连接
const handleTest = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    testing.value = true;
    testStatus.value = "idle";

    const config = await generateConfig();
    const result = await connectionStore.testConnection(config);

    emit("tested", result.message);
    console.log("连接测试成功:", result.message);
    testStatus.value = result.success ? "success" : "error";
  } catch (error) {
    console.error("连接测试失败:", error);
    const errorMsg = `测试失败: ${error}`;
    emit("tested", errorMsg);
    testStatus.value = "error";
  } finally {
    testing.value = false;
  }
};

// 测试命令
const handleTestCommand = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    executingCommand.value = true;

    const config = await generateConfig();

    // 先建立连接
    const connectResult = await connectionStore.connect(config);
    if (!connectResult.success) {
      throw new Error(connectResult.message);
    }

    // 执行测试命令
    const result = await connectionStore.executeCommand(
      config.id,
      "echo 'Hello from SSH!'"
    );

    // 执行完命令后断开连接
    try {
      await connectionStore.disconnect(config.id);
    } catch (disconnectError) {
      console.warn("断开测试连接失败:", disconnectError);
    }

    message.success(`命令执行成功: ${result}`);
    console.log("测试命令执行成功:", result);
  } catch (error) {
    console.error("测试命令执行失败:", error);
    message.error(`测试命令执行失败: ${error}`);
  } finally {
    executingCommand.value = false;
  }
};

// 建立连接
const handleConnect = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    connecting.value = true;

    const config = await generateConfig();
    const result = await connectionStore.connect(config);

    if (result.success) {
      // 连接成功后自动保存
      try {
        await connectionStore.saveConnection(config);
        console.log("链接配置已自动保存");
        message.success("连接建立成功并已保存");
      } catch (saveError) {
        console.warn("自动保存失败:", saveError);
        message.warning(`连接成功，但保存失败: ${saveError}`);
      }

      emit("connected", result.connectionId!);
      showModal.value = false;
      console.log("连接建立成功:", result.connectionId);
    } else {
      message.error(result.message);
    }
  } catch (error) {
    console.error("连接建立失败:", error);
    message.error(`连接建立失败: ${error}`);
  } finally {
    connecting.value = false;
  }
};

// 保存链接
const handleSave = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();

    if (isEdit.value && props.connection) {
      // 编辑模式：更新现有链接
      const updatedConfig: ConnectionConfig = {
        ...props.connection,
        name: formData.value.name,
        host: formData.value.host,
        port: formData.value.port,
        username: formData.value.username,
        password: formData.value.password || null,
        private_key_path: formData.value.private_key_path || null,
        auth_method: formData.value.auth_method,
        updated_at: new Date().toISOString(),
      };

      await connectionStore.updateConnection(updatedConfig);
      message.success("链接更新成功");
      emit("updated", updatedConfig);
      showModal.value = false;
    } else {
      // 新建模式：创建新链接
      const config = await generateConfig();
      await connectionStore.saveConnection(config);
      message.success("链接保存成功");
      emit("saved", config);
      showModal.value = false;
    }
  } catch (error) {
    console.error("保存链接失败:", error);
    message.error(`保存链接失败: ${error}`);
  }
};

// 关闭模态框
const closeModal = () => {
  showModal.value = false;
  resetForm();
  testStatus.value = "idle";
  testing.value = false;
  connecting.value = false;
  executingCommand.value = false;
};

// 重置表单
const resetForm = () => {
  formData.value = structuredClone(baseFormData);
};

// 监听模态框显示状态，处理编辑模式
watch(
  () => showModal.value,
  (newValue) => {
    if (newValue && props.connection) {
      // 编辑模式：填充现有连接数据
      formData.value = {
        id: props.connection.id,
        name: props.connection.name,
        host: props.connection.host,
        port: props.connection.port,
        username: props.connection.username,
        password: props.connection.password || "",
        private_key_path: props.connection.private_key_path || "",
        auth_method: props.connection.auth_method,
        created_at: props.connection.created_at,
        updated_at: props.connection.updated_at,
        connected: props.connection.connected,
        active: props.connection.active,
      } as ConnectionConfig;
    } else if (newValue) {
      // 新建模式：重置表单
      resetForm();
    }
  }
);

// 暴露方法
defineExpose({
  resetForm,
});
</script>

<style scoped>
.n-form {
  padding: 16px 0;
}

/* 测试按钮状态样式 */
.test-success {
  background-color: #18a058 !important;
  border-color: #18a058 !important;
}

.test-success:hover {
  background-color: #36ad6a !important;
  border-color: #36ad6a !important;
}

.test-error {
  background-color: #d03050 !important;
  border-color: #d03050 !important;
}

.test-error:hover {
  background-color: #de576d !important;
  border-color: #de576d !important;
}
</style>
