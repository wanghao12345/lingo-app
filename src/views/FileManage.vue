<template>
    <div class="w-full h-full flex flex-col box-border p-4">
        <div class="w-full h-50px bg-[#eeeeee] mb-4 rounded-5px flex items-center justify-between box-border px-2">
            <a-breadcrumb :routes="routes">
                <template #item-render="{ route, paths }">
                    <a-link :href="paths.join('/')">
                        {{ route.label }}
                    </a-link>
                </template>
            </a-breadcrumb>

            <a-space>
                <a-button type="outline">
                    <template #icon>
                        <icon-plus />
                    </template>
                    新建
                </a-button>
            </a-space>
        </div>
        <div class="w-full flex-1 flex flex-wrap box-border border border-solid border-[#eeeeee] rounded-5px">
            <div class="w-[100px] h-[100px] flex flex-col justify-center items-center" v-for="item in state.list"
                :key="item.name" @click="state.selected = item.name">
                <div :class="{ 'bg-[#eeeeee]': item.name === state.selected }"
                    class="w-[60px] h-[60px] rounded-5px flex items-center justify-center">
                    <img v-if="item.type === 'DIR'" src="/icons/dir-icon.png" alt="icon" class="w-50px h-50px" />
                    <img v-else src="/icons/file-icon.png" alt="icon" class="w-45px h-40px" />
                </div>

                {{ item.name }}
            </div>
        </div>
    </div>
</template>

<script setup>
const routes = [
    {
        path: '/',
        label: 'Home'
    },
    {
        path: '/channel',
        label: 'Channel',
    },
    {
        path: '/news',
        label: 'News'
    },
]
const state = reactive({
    selected: null,
    list: [
        {
            type: "DIR",
            name: "opt",
        },
        {
            type: "DIR",
            name: "channel",
        },
        {
            type: "FILE",
            name: "index.html",
        },
        {
            type: "FILE",
            name: "news.html",
        },
    ]
})
</script>

<style></style>