<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
  <div id="app" style="display: flex;flex-direction: row;">
    <!-- 左侧导航栏 -->
    <div class="left-bar border" >

      <div class="bar" style="background-color: rgb(207, 218, 218);">
        <router-link style="color: rgb(16, 12, 12);" to="/home">模拟</router-link>
      </div>

      <div class="bar" style="background-color: rgb(207, 218, 218);">
        <router-link style="color: rgb(16, 12, 12);" to="/index">后处理</router-link>
      </div>

    </div>

      <!-- 右侧功能区 -->
    <div class="router-view border" >
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </router-view>

    </div>  
  </div>
  

</template>

<style scoped>


.left-bar {
  width: auto;
  height: 100%;
  display: flex; 
  flex-direction: column; 
  align-items: center;
  justify-content: space-evenly;

}
.bar {
  width: 10vw;
  height: 8vh;
  border-radius: 5px;
  display: flex;
  justify-content: center;
  align-items: center;
  border: 1px solid rgb(12, 1, 21);
  box-shadow: 5px 5px 5px black;
  box-sizing: border-box;
  margin: 2px;
}

.router-view{
  width: 90%;
  height: 100%;
  /* align-items: center; */
}

</style>
