<!-- 引入src\styles.css -->

<template >
  <!-- 上侧导航栏 -->
  <div class="row " style="justify-content:space-between;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;padding: 0 10px 0 10px;">
      <!-- <input type="text" placeholder='请选择目录' v-model="path" @change="input"  class=" border center" style="width: 50%;">
      <button class="border center"  @click="selectDir">选择目录</button> -->
      <div class="row border" style="width: 70%;justify-content: space-between;border-radius: 0%;">
        <input type="text" placeholder='请选择目录' @change="input" v-model="path"  class=" center" style="width: 95%;border-radius: 0%;">
        <!-- <button class=" border center " @click="selectDir">选择目录</button> -->
        <img src="../assets/文件夹.png" @click="selectDir" style="width: auto;height: 100%;">
      </div>
    
      <button class=" center border" @click="confirm" :disabled="confirm_disabled">确定</button>
      <button class=" center border" @click="start" :disabled='start_disabled'>全部开始</button>
      
  </div>


  
  <div class="row " style="justify-content:space-between ;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;padding: 0 10px 0 10px;">
    <div class="row border"  style="width: 30%;">
      <div class="center" style="width: auto;">Abaqus版本：</div>
      <input  type="text" placeholder='版本' v-model="version" style="border-radius: 0%;width: 50%;">
    </div>

    <div class="row border"  style="width: 30%;">
      <div class="center" style="width: auto;">CPU核数：</div>
      <input style="border-radius: 0%;width: 60%;" type="text" placeholder='CPU核数' v-model="cpunumber">
    </div>

    <div class="row border"  style="width: 30%;">
      <div class="center" style="width: auto">时间：</div>
      <input style="border-radius: 0%;width: 60%;" @change="timeinput" type="text" v-model="injecttime" placeholder='时间' >
    </div>
    
    <div class="row border center" style="width:auto;border-radius: 10px;justify-content: space-evenly;user-select: none;width: 10%;" @click="suspend">
        <img  :src="suspendimg" style="width: auto;height: 30px;">
        <div class="center"  style="width: auto;">{{ status }}</div>
    </div>
  </div>

  <!-- 下侧功能区 -->
  <div class="scroll " style="height: calc(87% - 40px);">
    <div class="inppath " v-for="(inppath, index) in inpPaths" :key="index">
      <div class="border">{{ inppath }}</div>
      <ProgressBar class="border"  style="margin: 10px 0 5px 0;border-radius: 5px;" :progress="progress[index][0]" :step="progress[index][1]" />
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
      path: ['G:\\Model\\Abaqus\\project'],
      inpPaths: [],
      version: '2022',
      cpunumber: '1',
      injecttime: [1,432000,3888000,5184000],
      allowsuspend:false,
      index: 0,
      status:'已暂停',
      suspendimg: 暂停图片,
      progress: [],
    };
  },
  methods: {
    timeinput(e){
      this.injecttime = e.target.value.split(',');
      console.log(this.injecttime);
    },

    input(e){
      this.path = e.target.value.split(',');
      this.$store.commit('changepath', this.path);
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
        this.$store.commit('changepath', selected);
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        this.path = selected;
        this.$store.commit('changepath', [selected]);
        // user selected a single directory
      }
      
      console.log(selected);
    },

    confirm(){
      this.start_disabled = false;
      invoke('confirm', {
        paths: this.path,
        suffix: 'inp',
      })
        .then((res) => {
          if (res) {
            console.log(res);
            this.inpPaths = res;
            this.progress = Array.from({length: res.length}, () => [0, 1]);
            console.log(this.progress)
          } else {
          }
        })
        .catch((error) => {
          console.error(error);
        });
    },

    start(){
      this.confirm_disabled = true;
      this.start_disabled = true;
      console.log(this.path);
      // const invoke = window.__TAURI__.invoke
      invoke('start_simulate', {
        
        inppaths: this.inpPaths,
        selectedpath:this.path[0],
        version:this.version,
        cpunumber:this.cpunumber,
      })
        .then((res) => {
          if (res) {
            console.log(res);
            
          } else {
            
          }
        })
        .catch((error) => {
          console.error(error);
        });
      
      // setTimeout(() => {
      //   invoke('read_file', {
      //   path:this.path[0]+ "/log.ll",
      // }).then(data => {
      //   console.log(`调用延时函数成功，当前运log.ll内容为：${data}`);
      //   let index = Number(data);
      //   if(index!==0){
      //       for(let i=0;i<index;i++){
      //         // this.progress[i]=100;
      //         invoke('read_file', {
      //           path:this.inpPaths[i].split('.')[0]+".sta",
      //         })
      //           .then(data => {
      //             if(data !==' THE ANALYSIS HAS COMPLETED SUCCESSFULLY'){
      //               var parts = data.split(/\s+/);
      //               console.log(parts);
      //               let totaltime=parts.length >= 9 ? parseFloat(parts[8]) : null; 
      //               let step=parseInt(parts[1],10)
      //               if(totaltime){
      //                 console.log(`当前第${step}步已经模拟的时间为：${totaltime}秒`)
      //                 this.progress[i]=parseFloat((totaltime / this.injecttime[step-1] * 100).toFixed(1));
      //               }else{
      //                 console.log(0);
      //                 this.progress[i]=0;
      //               }
      //             }else{
      //               this.progress[i]=100;
      //             }
      //           })
      //           .catch(err => {
      //             console.error(err);
      //           });
      //       }
      //     }
      // })
      // }, 10000);

      
      
      let interval=setInterval(() => {
        if(this.allowsuspend){
          this.status = '已开始';
          this.suspendimg = 开始图片;
          clearInterval(interval);
        }else{
          console.log('index+++++++++++++++++:'+this.index);
        }
      }, 1000); 
      
      let intervalId=setInterval(() => {
        this.progressbar();
      }, 10000); 
    },
    
    progressbar(){
      console.log(this.path);
      invoke('read_file', {
        path:this.path[0]+ "/log.ll",
      })
        .then(data => {
          console.log(`当前运行的模拟的索引为：${data}`);
          let index = Number(data);
      
          console.log(`data中的index:${this.index}`);
          this.index = index;
          // this.allowsuspend=true;
          console.log(`index:${index}`);
          console.log(`data中的index:${this.index}`);
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
              this.allowsuspend=true;
              //  THE ANALYSIS HAS COMPLETED SUCCESSFULLY
              if(data !==' THE ANALYSIS HAS COMPLETED SUCCESSFULLY'){
                var parts = data.split(/\s+/);
                console.log(parts);
                var totaltime=parts.length >= 9 ? parseFloat(parts[8]) : null; 
                var step=parseInt(parts[1],10)
                if(totaltime){
                  console.log(`当前第${step}步已经模拟的时间为：${totaltime}秒`)
                  console.log(totaltime / this.injecttime[step-1] * 100);
                  // this.$set(this.progress, index, [parseFloat((totaltime / this.injecttime[step-1] * 100).toFixed(1)), step]);
                  this.progress[index][0]=parseFloat((totaltime / this.injecttime[step-1] * 100).toFixed(1));
                  this.progress[index][1]=step;
                  console.log(`更改完后的进度为：${this.progress[index]}`);
                }else{
                  console.log(0);
                  this.progress[index][0]=0;
                  this.progress[index][1]=1;
                }
              }else{
                this.progress[index][0]=100;
                this.progress[index][1]=1;
              }
              
              
            })
            .catch(err => {
              console.error('sta：'+err);
              this.progress[index][0]=0;
            });
        })
        .catch(err => {
          console.error('log.ll'+err);
        });
    },
    async suspend() {
      if(this.allowsuspend){

        let filePath = this.inpPaths[this.index]; 
        let fileNameWithExt = await basename(filePath); // "example.txt"
        let ext = await extname(filePath); // ".txt"

        if (typeof fileNameWithExt !== 'string') {
          throw new Error(`Expected a string but got ${typeof fileNameWithExt}`);
        }

        let fileName = fileNameWithExt.replace(ext, ''); // "example"
        if (fileName.endsWith('.')) {
          fileName = fileName.slice(0, -1);
        }
        console.log(fileName);
        let jobname = fileName
        let dirpath= await dirname(this.inpPaths[this.index]);
        console.log(`当前模拟的路径为：${dirpath}`)
        if (this.status === '已暂停') {
        console.log(this.index);
        console.log(this.inpPaths[this.index])
        console.log(`abaqus resume job=${jobname}`);
        
        invoke('suspendswitch',{
          dirpath:dirpath,
          command:`abaqus resume job=${jobname}`
        }).then((res) => {
          console.log(`res:${res}`);
          this.status = '已开始';
          this.suspendimg = 开始图片;
        })
        
      } else {
        console.log(this.index);
        console.log(this.inpPaths[this.index])
        console.log(`abaqus suspend job=${jobname}`);
        invoke('suspendswitch',{
          dirpath:dirpath,
          command:`abaqus suspend job=${jobname}`
        }).then((res) => {
          console.log(`res:${res}`);
          this.status = '已暂停';
          this.suspendimg = 暂停图片;
        })
        
      }
      }
      
    },


  },
  beforeRouteEnter(to, from, next) {
    console.log('home--beforeRouteEnter');
    next(vm => {
    // 访问组件实例 `vm`
      console.log(vm.$store.state.path);
      console.log('================');
      if (vm.$store.state.path!='E:/Office'){
        console.log(vm.$store.state.path);
        vm.path = [vm.$store.state.path];
      }
      }
    );
  },
};

</script>

<style scoped>
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