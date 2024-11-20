<!-- 引入src\styles.css -->

<template >
  <!-- 上侧导航栏 -->
  <div class="mt-4 row" style=" justify-content: space-evenly;margin: 10px 0 10px 0;">
        <el-input
        style="max-width: 60vw;height: 5vh;"
        placeholder="Please input"
        class="input-with-select"
        input-style="text-align: center;"
        @change="input" 
        v-model="path"
        >
            <template #append>
                <!-- <el-button icon="Search" /> -->
                <el-icon @click="selectDir" style="cursor: pointer;"><Folder  /></el-icon>
            </template>
        </el-input>

        <el-button-group style="height: 5vh;" size="large">
            <el-button  type="primary" icon="ArrowLeft" :disabled="confirm_disabled" @click="confirm">确定</el-button>
            <el-button  type="primary" >
              <el-icon  size="large"><component :is="currentIcon" /></el-icon>
            </el-button>
            <el-button type="primary" :disabled='start_disabled' @click="start">
            全部开始<el-icon class="el-icon--right" ><ArrowRight/></el-icon>
            </el-button>
        </el-button-group>   
  </div>

  <el-table :data="tabledata1" >
        <el-table-column label="Abaqus版本"  header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.version"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGV" label="CPU核数"   header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.cpunumber"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGh" label="时间"   header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.injecttime"
              @change.native="timeinput"
              :controls="false"
            />
          </template>
        </el-table-column>
    </el-table>


  <el-scrollbar  style="padding: 10px;height: 10vh;flex-grow: 1;boxShadow:--el-box-shadow-dark;">
      <el-card v-for="(inppath, index) in inpPaths" class="box-card" style="height:70px; margin: 2px 0 0 0;">
          <div >{{ inppath }}</div> 
          <ProgressBar :progress="progress[index][0]" :step="progress[index][1]" />
      </el-card>
    </el-scrollbar>

  <div class="center" style="height: 3%;" v-if="inpPaths.length > 0">{{ inpPaths.length }}</div>
</template>

<script >
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appDir, dirname } from '@tauri-apps/api/path';
import ProgressBar from './Progressbar.vue';
import { fs } from '@tauri-apps/api';
import { basename, extname } from '@tauri-apps/api/path';
import {
  VideoPlay,
  VideoPause,
} from '@element-plus/icons-vue'
export default {
  components: {
    ProgressBar,
  },
  data() {
    return {
      tabledata1: [
        {
          version: '2022',
          cpunumber: '1',
          injecttime: [1,432000,3888000,5184000],
        },
      ],
      confirm_disabled: false,
      start_disabled: true,
      path: ['G:\\Model\\Abaqus\\project'],
      inpPaths: [],
      allowsuspend:false,
      index: 0,
      progress: [],
      currentIcon:"VideoPlay",
    };
  },
  methods: {
    timeinput(e){
      // console.log(e)
      this.tabledata1[0].injecttime = e.target.value.split(',');
      // this.injecttime = e.target.value.split(',');
      // console.log(this.injecttime);
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
        this.path = [selected];
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
        version:this.tabledata1[0].version,
        cpunumber:this.tabledata1[0].cpunumber,
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
          // this.suspendimg = 开始图片;
          this.currentIcon="VideoPause";
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
                  console.log(totaltime / this.tabledata1[0].injecttime[step-1] * 100);
                  // this.$set(this.progress, index, [parseFloat((totaltime / this.injecttime[step-1] * 100).toFixed(1)), step]);
                  this.progress[index][0]=parseFloat((totaltime / this.tabledata1[0].injecttime[step-1] * 100).toFixed(1));
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
          this.currentIcon="VideoPause";
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

.scroll{
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 90%;
  overflow: auto;
  align-items: center;
}




</style>