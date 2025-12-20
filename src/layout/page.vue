<template>
    <a-layout class="layout-demo">
        <a-layout-sider collapsible breakpoint="xl">
            <a-menu :selected-keys="[route.path]" :style="{ width: '100%' }"
                @menu-item-click="onClickMenuItem">
                <a-menu-item :key="`/page/${route.params.pageId}/file-manage`">
                    <icon-folder :size="20" />
                    文件管理器
                </a-menu-item>
                <a-menu-item :key="`/page/${route.params.pageId}/command-line`">
                    <icon-code-square :size="20" />
                    命令行工具
                </a-menu-item>
            </a-menu>
            <!-- trigger -->
            <template #trigger="{ collapsed }">
                <IconCaretRight v-if="collapsed"></IconCaretRight>
                <IconCaretLeft v-else></IconCaretLeft>
            </template>
        </a-layout-sider>
        <a-layout>
            <a-layout style="padding: 10px;">
                <a-layout-content>
                    <router-view></router-view>
                </a-layout-content>
            </a-layout>
        </a-layout>
    </a-layout>
</template>
<script setup>
import { Message } from '@arco-design/web-vue';
import {
    IconCaretRight,
    IconCaretLeft,
    IconFolder,
    IconCodeSquare,
} from '@arco-design/web-vue/es/icon';

const route = useRoute();
const router = useRouter();

function onClickMenuItem(key) {
    Message.info({ content: `You select ${key}`, showIcon: true });
    router.push(key)
    console.log(route)
}

</script>
<style lang="less" scoped>
.layout-demo {
    height: 100%;
    background: var(--color-fill-2);
    // border: 1px solid var(--color-border);
    box-sizing: border-box;
    :deep(.arco-menu) {
        .arco-menu-item {
            display: flex;
            align-items: center;
            .arco-icon {
                margin-right: 8px;
            }
        }
    }
}

.layout-demo :deep(.arco-layout-sider) .logo {
    height: 32px;
    margin: 12px 8px;
    background: rgba(255, 255, 255, 0.2);
}

.layout-demo :deep(.arco-layout-sider-light) .logo {
    background: var(--color-fill-2);
}

.layout-demo :deep(.arco-layout-header) {
    height: 64px;
    line-height: 64px;
    background: var(--color-bg-3);
}

.layout-demo :deep(.arco-layout-footer) {
    height: 48px;
    color: var(--color-text-2);
    font-weight: 400;
    font-size: 14px;
    line-height: 48px;
}

.layout-demo :deep(.arco-layout-content) {
    color: var(--color-text-2);
    font-weight: 400;
    font-size: 14px;
    background: var(--color-bg-3);
}
</style>
