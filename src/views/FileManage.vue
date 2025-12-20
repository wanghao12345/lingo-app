<template>
    <div class="w-full box-border p-4">
        <div>
            <a-breadcrumb :routes="routes">
                <template #item-render="{ route, paths }">
                    <a-link :href="paths.join('/')">
                        {{ route.label }}
                    </a-link>
                </template>
            </a-breadcrumb>
        </div>
        <div class="w-full flex flex-wrap box-border border border-solid border-[#ccc] py-4">
            <div class="w-[100px] flex flex-col justify-center items-center" v-for="item in state.list"
                :key="item.name">
                <div class="w-full h-[60px] flex items-center justify-center">
                    
                    <img 
                    :src="item.type === 'DIR' ? '/icons/dir-icon.png' : '/icons/file-icon.png'" alt="icon"
                        class="w-50px h-50px">
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