<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar.vue";
import ViewPort from "./components/ViewPort.vue";

</script>

<template>
  <main class="container">
    <Sidebar />
    <ViewPort />
  </main>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>

</style>
