<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';
const count = ref(0)
const title = ref('测试')
const updateTitle = async () => {
    count.value++
    await invoke("update_title", { title: "测试" + count.value })
}
</script>
<template>
    <h1>main</h1>
    <button @click="updateTitle">更改标题:{{title}}:{{count}}</button>
</template>
