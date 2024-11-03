<script  lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { confirm } from '@tauri-apps/api/dialog';
import Aside from "./components/Aside.vue";
export default {
  data() {
    return {
      unlisten: Function(),
    };
  },
  async mounted() {
    this.unlisten = await appWindow.onCloseRequested(async (event) => {
      const confirmed = await confirm('可能还有模拟任务在运行，确定要退出吗?', { title: '警告', type: 'info',okLabel: '确定', cancelLabel: '取消' });
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
    <div id="app" class="common-layout">
      <el-container>
        <el-aside style="height: 100vh;width: 15vw;">
          <Aside style="height: 100%" />
        </el-aside>
        <el-main style="height: 100vh;">
          <router-view  v-slot="{ Component }" >
            <keep-alive>
              <component :is="Component" />
            </keep-alive>
          </router-view>
        </el-main>
      </el-container>
    </div>
  

</template>

<style scoped>
.gradient {
  background: linear-gradient(to right, rgb(71, 153, 235), rgb(247, 252, 119), rgb(191, 248, 221));
  }

.left-bar {
  position: relative;
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
  z-index: 1000;
}

/* .bar::before{
  content: "";
  width: 200%;
  height: 200%;
  background-color: aliceblue;
  position: absolute;
  left: -50%;
  top: -50%;
} */

.router-view{
  width: 90%;
  height: 100%;
  /* align-items: center; */
}



</style>
