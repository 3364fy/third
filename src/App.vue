<script  lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { confirm } from '@tauri-apps/api/dialog';
export default {
  data() {
    return {
      unlisten: Function(),
    };
  },
  async mounted() {
    this.unlisten = await appWindow.onCloseRequested(async (event) => {
      const confirmed = await confirm('可能还有模拟任务在运行，确定要退出吗?');
      if (!confirmed) {
        // user did not confirm closing the window; let's prevent it
        event.preventDefault();
      }
    });
  },

  beforeUnmount(){
    this.unlisten();
  },
  
}
</script>

<template>
  <div id="app" style="display: flex;flex-direction: row;">
    <!-- 左侧导航栏 -->
    <div class="left-bar border" >

      <div class="bar" style="">
        <router-link style="color: rgb(16, 12, 12);" to="/coal">生成输入文件</router-link>
      </div>

      <div class="bar" style="">
        <router-link style="color: rgb(16, 12, 12);" to="/home">Abaqus模拟</router-link>
      </div>

      <div class="bar" >
        <router-link style="color: rgb(16, 12, 12);" to="/index">Abaqus后处理</router-link>
      </div>

      <!-- <div class="bar" >
        <router-link style="color: rgb(16, 12, 12);" to="/fluent">Fluent模拟</router-link>
      </div> -->

    </div>

      <!-- 右侧功能区 -->
    <div class="router-view border"  style="backdrop-filter: blur(10px);">
      <router-view  v-slot="{ Component }" >
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </router-view>

    </div>  
  </div>
  

</template>

<style scoped>
.gradient {
  background: linear-gradient(to right, rgb(71, 153, 235), rgb(247, 252, 119), rgb(191, 248, 221));
  }

.left-bar {
  width: 10%;
  height: 100%;
  display: flex; 
  flex-direction: column; 
  align-items: center;
  justify-content: space-evenly;

}
.bar {
  width: 80%;
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
