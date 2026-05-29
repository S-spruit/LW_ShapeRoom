<script setup>
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from "@tauri-apps/api/core";


const logs = ref([]);
// const messageText = ref("Hello from the Frontend!");

// async function sendToConsole() {
//   try {
//     // This matches the function name in your Rust lib.rs
//     await invoke('cmd_send_message', { message: messageText.value });
//   } catch (error) {
//     console.error("Failed to send message to Rust:", error);
//   }
// }
function clearLogs() {
  logs.value = [];
}


onMounted(async () => {
  // Listen for the 'c-log' event from Rust
  await listen('c-log', (event) => {
    logs.value.push({
      id: Date.now(),
      text: event.payload,
      time: new Date().toLocaleTimeString()
    });
    // Keep only the last 50 logs so it doesn't get laggy
    if (logs.value.length > 50) logs.value.shift();
  });
});
</script>

<template>
  <div id="c_container">
    <div id="c_header">
      Console
      <button class="btn_transparent" @click="clearLogs">X</button>
      <!-- <div class="test_controls">
        <input v-model="messageText" placeholder="Type a log message..." />
        <button @click="sendToConsole">Send to Console</button>
      </div> -->
    </div>
    <div class="console_box">
      <div v-for="log in logs" :key="log.id" class="log_entry">
        <span class="timestamp">[{{ log.time }}]</span> {{ log.text }}
      </div>
    </div>
  </div>

</template>

<style scoped>
#c_header {
  background: radial-gradient(#b07a43, #c28b52);
  width: 270px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0px 15px 0px 15px;
}
#c_container {
  position: absolute;
  bottom: 0px;
}

.console_box {
  background: #000000;
  color: #00ff00;
  /* Classic hacker green */
  font-family: 'Courier New', monospace;
  font-size: 12px;
  overflow-y: auto;
  height: 200px;
  padding: 10px;
  
}
.test_controls {
  display: flex;
  flex-direction: row;
}

.timestamp {
  color: #888;
  margin-right: 5px;
}
</style>