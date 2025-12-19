<template>
  <div class="text-center text-black">Home {{ route.params.pageId }}</div>
</template>

<script setup>
    const route = useRoute();
    console.log(route.params.pageId)
</script>

<style>

</style>