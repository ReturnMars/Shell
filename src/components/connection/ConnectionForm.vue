<template>
  <n-modal
    v-model:show="showModal"
    preset="dialog"
    title="SSH连接配置"
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
      <n-form-item label="连接名称" path="name">
        <n-input v-model:value="formData.name" placeholder="请输入连接名称" />
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
        <n-input v-model:value="formData.username" placeholder="请输入用户名" />
      </n-form-item>

      <n-form-item label="认证方式" path="auth_method">
        <n-select v-model:value="formData.auth_method" :options="authOptions" />
      </n-form-item>

      <n-form-item
        v-if="
          formData.auth_method === 'Password' || formData.auth_method === 'Both'
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
      <n-space>
        <n-button size="small" @click="showModal = false">取消</n-button>
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
          type="success"
          size="small"
          @click="handleConnect"
          :loading="connecting"
          :disabled="testing || testStatus !== 'success'"
        >
          连接
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { FormInst, FormRules } from "naive-ui";
import { useMessage } from "naive-ui";
import { CheckCircleOutlined, CloseCircleOutlined } from "@vicons/antd";
import { Type } from "naive-ui/es/button/src/interface";
import { Connection, ConnectionForm } from "./type";

// Props
interface Props {
  show: boolean;
}

defineProps<Props>();

// Emits
const emit = defineEmits<{
  connected: [connectionId: string];
  tested: [result: string];
}>();

// 响应式数据
const showModal = defineModel<boolean>("show");
const formRef = ref<FormInst | null>(null);
const testing = ref(false);
const isTested = ref(false);
const connecting = ref(false);
const testStatus = ref<"idle" | "success" | "error">("idle");

// 消息提示
const message = useMessage();

const baseFormData: ConnectionForm = {
  name: "wzd",
  host: "47.109.195.0",
  port: 22,
  username: "root",
  password: "Aioreturn@123",
  private_key_path: "",
  auth_method: "Password" as "Password" | "PrivateKey" | "Both",
};
// 表单数据
const formData = ref<typeof baseFormData>(structuredClone(baseFormData));

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
    isTested.value = false;
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

// 生成连接配置
const generateConfig = async () => {
  return {
    id: (await invoke("generate_uuid")) as string,
    name: formData.value.name,
    host: formData.value.host,
    port: formData.value.port,
    username: formData.value.username,
    password: formData.value.password,
    private_key_path: formData.value.private_key_path,
    auth_method: formData.value.auth_method,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
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
    const result = await invoke("test_connection", { config });

    emit("tested", result as string);
    console.log("连接测试成功:", result);
    testStatus.value = "success";
  } catch (error) {
    console.error("连接测试失败:", error);
    const errorMsg = `测试失败: ${error}`;
    emit("tested", errorMsg);
    testStatus.value = "error";
  } finally {
    testing.value = false;
    isTested.value = true;
  }
};

// 建立连接
const handleConnect = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    connecting.value = true;

    const config = await generateConfig();
    const connectionId = await invoke("connect_ssh", { config });

    // 连接成功后自动保存
    try {
      await invoke("save_connection", { config });
      console.log("连接配置已自动保存");
      message.success("连接建立成功并已保存");
    } catch (saveError) {
      console.warn("自动保存失败:", saveError);
      message.warning(`连接成功，但保存失败: ${saveError}`);
    }

    emit("connected", connectionId as string);
    showModal.value = false;
    console.log("连接建立成功:", connectionId);
  } catch (error) {
    console.error("连接建立失败:", error);
    message.error(`连接建立失败: ${error}`);
  } finally {
    connecting.value = false;
  }
};

// 关闭模态框
const closeModal = () => {
  showModal.value = false;
  resetForm();
  testStatus.value = "idle";
  testing.value = false;
  connecting.value = false;
  isTested.value = false;
};

// 重置表单
const resetForm = () => {
  formData.value = structuredClone(baseFormData);
};

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
