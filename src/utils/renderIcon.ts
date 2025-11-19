import { Component, h } from "vue";
import { NIcon } from "naive-ui";
import type { IconProps } from "naive-ui";

export function renderIcon(icon: Component, props: IconProps = {}) {
  return () => h(NIcon, { ...props }, { default: () => h(icon) });
}
