<script setup lang="ts">

</script>
     
<template>
    <section class="app-main">
        <router-view v-slot="{ Component }">
            <transition name="zoom-fade"
                        mode="out-in"
                        appear>
                <keep-alive>
                    <div>
                        <component :is="Component" />
                    </div>
                </keep-alive>
            </transition>
        </router-view>
    </section>
</template>
     
<style scoped lang="scss">

</style>
    