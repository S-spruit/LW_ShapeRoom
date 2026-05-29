<script setup>
    import EventConsole from './EventConsole.vue';
</script>
<template>
    <div id="sb_sidebar">
        <div id="sb_header">
            sidebar
        </div>
        <EventConsole></EventConsole>
    </div>
</template>
<style scoped>
#sb_header {
    background: radial-gradient(#b07a43, #c28b52) ;
    width: 300px;
}
#sb_sidebar {
    background-color: #222428;
    height: calc(100vh);
}
</style>