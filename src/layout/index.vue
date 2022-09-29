<script setup lang="ts">
import { ref, reactive } from 'vue';
import AppMain from './components/AppMain.vue';
import SideTitle from './components/SideTitle.vue';
import SideBar from './components/SideBar.vue';
import NavBar from './components/NavBar.vue';
import Profile from './components/Profile.vue';
</script>
     
<template>
    <div class="app">
        <div class="left">
            <div class="left-top">
                <side-title></side-title>
            </div>
            <div class="left-body">
                <side-bar></side-bar>
            </div>
        </div>
        <div class="right">
            <div class="right-top">
                <div class="right-top-left">
                    <nav-bar></nav-bar>
                </div>
                <div class="right-top-right">
                    <profile></profile>
                </div>
            </div>
            <div class="right-body">
                <app-main></app-main>
            </div>
        </div>
    </div>
</template>
     
<style scoped lang="scss">
.app {
    display: flex;

    .left {
        width: 200px;
        display: flex;
        flex-direction: column;
    }

    .right {
        width: 100%;
        display: flex;
        flex-direction: column;

        .right-top {
            display: flex;

            .right-top-left {
                width: 100%;
            }

            .right-top-right {
                width: 150px;
            }
        }
    }
}
</style>
    