<template>
  <!-- 上侧导航栏 -->
  <div class="row " style="justify-content:space-around;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">
    
      <input type="text" placeholder='请选择目录' @change="input1" v-model="path"  class="border center" style="width: 60%;">
      <button class=" border center " @click="selectDir">选择目录</button>
      <button class=" center border" @click="confirm">确定</button> 
      <button class=" center border" @click="aftertreat">开始</button> 
    
  </div>

  <div class=" row  " style="justify-content:space-around;height: 5%;margin: 10px 0 10px 0;">

      <input type="text" placeholder='请选择宏文件' @change="input" v-model="macro"  class="border center" style="width: 60%;">
      <button class=" border center " @click="selectfile">选择宏文件</button>
      <input type="text" placeholder='版本'  v-model="version"  class="border center" style="width: 11%;">
      <button class="border center" @click="add">+</button>

  </div>

  

  <!-- 下侧功能区 -->
  <div class="scroll" style="height: calc(86% - 30px);">
  
    <div v-for="(item, index) in replace" :key="index"  class=" row" style="height: 8%;width: 100%;box-sizing: border-box;margin: 2px 0 2px 0;">
      <input type="text" :data-id="index" data-index="0" placeholder='字符串' :value="replace[index][0]" @change="input"   class="border center" >
      <input type="text" :data-id="index" data-index="1" placeholder='替换字符串' :value="replace[index][1]" @change="input"   class="border center" >
      <button class="border" :data-id="index" @click="reduce">-</button>


    </div>

    <div class="inppath " v-for="odbpath in odbPaths" :key="path">
      <div class="border">{{ odbpath }}</div>
    </div>
    
  </div>

  <div class="center" style="height: 3%;" v-if="odbPaths.length > 0">{{ odbPaths.length }}</div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import ProgressBar from './Progressbar.vue';
export default {
  components: {
    ProgressBar,
  },
  data() {
    return {
      path: ['G:\\Model\\Abaqus\\project'],
      macro:'G:\\desktop\\abaqus.py',
      version:'2022',
      odbPaths: [],
      replace: [["D:/temp/test0/test.odb","{pwd}\\{0}"]],
    };
  },
  methods: {
    input1(e){
      this.path = e.target.value.split(',');
    },
    input(e){
      // 用以实现宏文件的字符串替换功能
      console.log(e);
      this.replace[e.target.dataset.id][e.target.dataset.index] = e.target.value;
    },
    add(e){
      this.replace.push(['', '']);
      console.log(this.version);
      console.log(this.replace);
    },
    reduce(e){
      console.log(e.target.dataset.id);
      this.replace.splice(e.target.dataset.id, 1);
      console.log(this.replace);
    },
    selectDir:async function(){
      const selected = await open({
        directory: true,
        multiple: true,
        defaultPath: await appDir(),
      });
      if (Array.isArray(selected)) {
        // user selected multiple directories
        this.path = selected;
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        this.path = selected;
        // user selected a single directory
      }
      
      console.log(selected);
    },

    selectfile:async function(){
      const selected = await open({
        directory: false,
        multiple: false,
        defaultPath: await appDir(),
      });
      if (Array.isArray(selected)) {
        // user selected multiple directories
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        this.macro = selected;
        // user selected a single directory
      }
      
      console.log(selected);
    },

    confirm(){
      // 返回选择目录下所有的指定后缀的文件的路径
      console.log(this.path);
      invoke('confirm', {
        paths: this.path,
        suffix: 'odb',
      })
        .then((res) => {
          if (res) {
            console.log(res);
            this.odbPaths = res;
          } else {
            this.s = 'false';
          }
        })
        .catch((error) => {
          console.error(error);
        });
    },

    start(){
      // const invoke = window.__TAURI__.invoke
      invoke('start_aftertreat', {
        inppaths: this.inpPaths,
      })
        .then((res) => {
          if (res) {
            console.log(res);
            
          } else {
            this.s = 'false';
          }
        })
        .catch((error) => {
          console.error(error);
        });
    },

    aftertreat(){
      // const invoke = window.__TAURI__.invoke
      invoke('aftertreat', {
        macrofile: this.macro,
        version: this.version,
        odbpaths: this.odbPaths,
        replace: this.replace,
      })
        .then((res) => {
          if (res) {
            console.log(res);
            
          } else {
            console.log('false');
          }
        })
        .catch((error) => {
          console.error(error);
        });
    }

}
};

</script>

<style>
@import "../styles.css";
side{
  width: 50%;
  height: 100%;
}

button{
  width: auto;
  height: 100%;
  border-radius: 5px;
}

.scroll{
  display: flex;
  flex-direction: column;
  width: 100%;
  overflow: auto;
  margin: 0;
  box-sizing: border-box;
  align-items: center;
  overflow-x: hidden;
}


.inppath{
  display: flex;
  flex-direction: column;
  border-radius: 3px;
  height: auto;
  width: 95%;
  /* border: 1px solid rgb(6, 6, 6);
  box-shadow: 2px 2px 2px rgb(221, 148, 148); */
  box-sizing: border-box;
}

</style>
