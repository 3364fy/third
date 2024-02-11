<!-- 引入src\styles.css -->

<template >
    <!-- 上侧导航栏 -->
    <div class="row " style="justify-content:space-between ;box-sizing: border-box;height: 5%; margin: 10px 0 10px 0;">
        <input type="text" placeholder='请选择目录' v-model="path" @change="input"  class=" border center" style="width: 70%;">
        <button class="border center"  @click="selectDir">选择目录</button>
        <button class=" center border" @click="confirm" :disabled="confirm_disabled">确定</button>
    </div>
  
    <div class="row " style="justify-content:space-between ;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">
      <input style="width: 25%;" type="text" placeholder='版本' v-model="version"   class=" border center" >
      <input style="width: 25%;" type="text" placeholder='维度' v-model="dimensions"   class=" border center" >      
      <input style="width: 25%;" type="text" placeholder='线程数' v-model="cpunumber"   class=" border center" >
      <button class=" center border" @click="start" :disabled='start_disabled'>全部开始</button>
    </div>
  
    <!-- 下侧功能区 -->
    <div class="scroll " style="height: calc(87% - 40px);">
      <div class="inppath " v-for="(inppath, index) in inpPaths" :key="index">
        <div class="border">{{ inppath }}</div>
        <!-- <ProgressBar class="border" style="margin: 10px 0 5px 0;border-radius: 5px;" :progress="progress[index]" /> -->
      </div>
    </div>
  
    <div class="center" style="height: 3%;" v-if="inpPaths.length > 0">{{ inpPaths.length }}</div>
  </template>
  
  <script >
  import { invoke } from '@tauri-apps/api/tauri';
  import { open } from '@tauri-apps/api/dialog';
  import { appDir, dirname } from '@tauri-apps/api/path';
  import ProgressBar from './Progressbar.vue';
  import { fs } from '@tauri-apps/api';
  import 开始图片 from '../assets/运行.png';
  import 暂停图片 from '../assets/暂停.png';
  import { basename, extname } from '@tauri-apps/api/path';
  export default {
    components: {
      ProgressBar,
    },
    data() {
      return {
        confirm_disabled: false,
        start_disabled: true,
        path: ['G:\\Model\\ANSYS\\project'],
        inpPaths: [],
        version: '2022',
        cpunumber: '3',
        dimensions:'2',
        progress: [],
        
        index: 0,
        
      };
    },
    methods: {
      input(e){
        this.path = e.target.value.split(',');
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
  
      confirm(){
        this.start_disabled = false;
        invoke('confirm', {
          paths: this.path,
          suffix: 'jou',
        })
          .then((res) => {
            if (res) {
              console.log(res);
              this.inpPaths = res;
              this.progress = new Array(res.length).fill(0);
            } else {
            }
          })
          .catch((error) => {
            console.error(error);
          });
      },
  
      
      start(){
          invoke('startfluent',{
            selectedpath: this.path[0],
            joupaths: this.inpPaths,
            version: this.version,
            cpunumber: this.cpunumber,
            dimensions: this.dimensions,
          })
          .then((res)=>{
              console.log(res)
          })
          .catch((error) => {
              console.error(error);
          });
        }
        
      
  
  
    },
  };
  
  </script>
  
  <style>
  @import "../styles.css";
  
  button{
    width: auto;
    height: 100%;
    border-radius: 5px;
  }
  
  button:disabled {
    /* 禁用状态下的颜色 */
    background-color: #CCCCCC;
    color: #666666;
  }
  
  .scroll{
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 90%;
    overflow: auto;
    align-items: center;
  }
  
  
  .inppath{
    display: flex;
    flex-direction: column;
    border-radius: 3px;
    height: auto;
    width: 95%;
    margin: 5px;
    /* border: 1px solid rgb(6, 6, 6);
    box-shadow: 2px 2px 2px rgb(221, 148, 148); */
    box-sizing: border-box;
  }
  
  </style>