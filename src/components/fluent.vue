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
            <el-button type="primary"  @click="start">
            全部开始<el-icon class="el-icon--right" ><ArrowRight/></el-icon>
            </el-button>
        </el-button-group>   
  </div>
  <el-table :data="tabledata" >
        <el-table-column label="Fluent版本"  header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.version"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGV" label="维度"   header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.dimensions"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGh" label="线程数"   header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.cpunumber"
             
              :controls="false"
            />
          </template>
        </el-table-column>
    </el-table>
  
    <!-- 下侧功能区 -->

    <el-scrollbar  style="padding: 10px;height: 50vh;flex-grow: 1;boxShadow:--el-box-shadow-dark;">
      <el-card v-for="inppath in inpPaths" class="box-card" style="height:30px; margin: 2px 0 0 0;display: flex; text-align: center;align-items: center;">
          <div >{{ inppath }}</div> 
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
  // import 开始图片 from '../assets/运行.png';
  // import 暂停图片 from '../assets/暂停.png';
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
        tabledata: [
          {
            version: '2022',
            dimensions: '2',
            cpunumber: '3',
          },
        ],
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
          this.path = [selected];
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