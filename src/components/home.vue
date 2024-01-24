<!-- 引入src\styles.css -->

<template >
  <!-- 上侧导航栏 -->
  <div class="row " style="justify-content:left ;;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">
    <div class="side row  " style="justify-content:space-around;">
      <input type="text" placeholder='请选择目录' v-model="path" @change="input"  class=" border center" >
      <button class="border center"  @click="selectDir">选择目录</button>
    </div>

    <div class="side row" style="justify-content:space-around;">
      <button class=" center border" @click="confirm">确定</button>
      <button class=" center border" @click="start">全部开始</button> 
    </div>

  </div>

  <div class="row " style="justify-content:space-around ;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">
    <input style="width: 30%;" type="text" placeholder='版本' v-model="version"   class=" border center" >
    <input style="width: 30%;" type="text" placeholder='CPU核数' v-model="cpunumber"   class=" border center" >
    <input style="width: 30%;" type="text" placeholder='注液时间' v-model="injecttime"   class=" border center" >
  </div>

  <!-- 下侧功能区 -->
  <div class="scroll " style="height: calc(87% - 40px);">
    <div class="inppath " v-for="(inppath, index) in inpPaths" :key="index">
      <div class="border">{{ inppath }}</div>
      <ProgressBar class="border" style="margin: 10px 0 5px 0;border-radius: 5px;" :progress="progress[index]" />
    </div>
  </div>

  <div class="center" style="height: 3%;" v-if="inpPaths.length > 0">{{ inpPaths.length }}</div>
</template>

<script >
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import ProgressBar from './Progressbar.vue';
import { fs } from '@tauri-apps/api';
export default {
  components: {
    ProgressBar,
  },
  data() {
    return {
      path: ['G:\\Model\\Abaqus\\project'],
      inpPaths: [],
      version: '2022',
      cpunumber: '1',
      injecttime: 1200,
      progress: [],
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
      invoke('confirm', {
        paths: this.path,
        suffix: 'inp',
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
      console.log(this.path);
      // const invoke = window.__TAURI__.invoke
      invoke('start_simulate', {
        
        inppaths: this.inpPaths,
        selectedpath:this.path,
        version:this.version,
        cpunumber:this.cpunumber,
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
      
      let intervalId=setInterval(() => {
            this.progressbar();
          }, 10000);
    },
    
    progressbar(){
      console.log(this.path);
      invoke('read_file', {
        path:this.path+ "/log.ll",
      })
        .then(data => {
          console.log(data);
          let index = Number(data);
          console.log(this.inpPaths[index]);
          // let parts = this.inpPaths[index].split('\\');
          // let filename = parts[parts.length - 1]; // 获取最后一个部分，即文件名
          let name = this.inpPaths[index].split('.')[0]; // 去掉扩展名
          let stapath=name+".sta";
          console.log(stapath); // 输出：cross-layer
          invoke('read_file', {
            path:stapath,
          })
            .then(data => {
              console.log(data);
              //  THE ANALYSIS HAS COMPLETED SUCCESSFULLY
              if(data !==' THE ANALYSIS HAS COMPLETED SUCCESSFULLY'){
                var parts = data.split(/\s+/);
                console.log(parts);
                let totaltime=parts.length >= 7 ? parseFloat(parts[7]) : null; 
                if(totaltime){
                  console.log(totaltime);
                  this.progress[index]=parseFloat((totaltime / this.injecttime * 100).toFixed(1));
                }else{
                  console.log(0);
                  this.progress[index]=0;
                }
              }else{
                this.progress[index]=100;
              }
              
              
            })
            .catch(err => {
              console.error('sta'+err);
              this.progress[index]=0;
            });
        })
        .catch(err => {
          console.error('log.ll'+err);
        });
    },
    
  }


};

</script>

<style>
@import "../styles.css";

button{
  width: auto;
  height: 100%;
  border-radius: 5px;
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