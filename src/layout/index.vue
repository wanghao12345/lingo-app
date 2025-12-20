<template>
    <div class="w-full h-full">
        <div class="w-full h-50px bg-white box-border p-10px flex items-center">
            <div class="w-180px h-30px flex-shrink-0 mr-20px bg-[#eeeeee] cursor-pointer" @click="handleLogoClick"></div>
            <div class="flex-1 h-full tab-box">
                <a-tabs :editable="true" @tab-click="handleTabClick" @add="handleAdd" @delete="handleDelete"
                    show-add-button auto-switch>
                    <a-tab-pane v-for="(item) of data" :key="item.key" :title="item.title">
                    </a-tab-pane>
                </a-tabs>
            </div>
        </div>
        <div class="w-full page-box">
            <router-view></router-view>
        </div>

    </div>
</template>
<script setup>
const router = useRouter();
const data = ref([
    {
        key: '1',
        title: 'Tab 1',
        content: 'Content of Tab Panel 1'
    },
    {
        key: '2',
        title: 'Tab 2',
        content: 'Content of Tab Panel 2'
    },
    {
        key: '3',
        title: 'Tab 3',
        content: 'Content of Tab Panel 3'
    },
    {
        key: '4',
        title: 'Tab 4',
        content: 'Content of Tab Panel 4'
    }
]);
function handleAdd() {
    const number = data.value.length + 1;
    data.value = data.value.concat({
        key: `${number}`,
        title: `New Tab ${number}`,
        content: `Content of New Tab Panel ${number}`
    })
};
function handleDelete(key) {
    data.value = data.value.filter(item => item.key !== key)
};
function handleTabClick(key) {
    console.log(key)
    router.push(`/page/${key}/file-manage`)
}
function handleLogoClick() {
    router.push('/dashboard')
}
</script>
<style scoped lang="less">
.page-box {
    height: calc(100% - 50px);
}

.tab-box {
    width: calc(100% - 200px);

    :deep(.arco-tabs) {
        .arco-tabs-tab {
            width: 180px;
            box-sizing: border-box;
            height: 40px;
            margin: 0;
            box-sizing: border-box;
            padding: 0 10px;
            position: relative;

            &::before {
                content: '';
                position: absolute;
                top: 10px;
                left: 0;
                width: 1px;
                height: 20px;
                background: #eeeeee;
            }

            .arco-tabs-tab-close-btn {
                position: absolute;
                top: 50%;
                right: 10px;
                transform: translateY(-50%);
            }
        }

        .arco-tabs-nav-ink {
            display: none;
        }

        .arco-tabs-tab-active {
            border: 1px solid #eeeeee;
            border-radius: 10px 10px 0 0;
            background: #f2f3f5;

            &::before {
                display: none;
            }

            +.arco-tabs-tab {
                &::before {
                    display: none;
                }
            }
        }

        .arco-tabs-content {
            display: none;
        }

        .arco-tabs-nav::before {
            display: none;
        }

        .arco-tabs-nav-type-line .arco-tabs-tab-title::before {
            display: none;
        }
    }
}
</style>
