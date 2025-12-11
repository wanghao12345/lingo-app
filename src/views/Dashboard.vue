<template>
  <div>
    <h1>Welcome to the Dashboard</h1>
    <button @click="connect">Connect SSH</button>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
async function connect() {
  const greeting = await invoke("greet", { name: "Tauri" });
  console.log(greeting);
}


</script>

<style></style>