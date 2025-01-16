<template>
    <div class="mt-4 row" style=" justify-content: space-evenly;margin: 10px 0 10px 0;">
        <el-input
        style="max-width: 55vw;height: 5vh;"
        placeholder="Please input"
        class="input-with-select"
        input-style="text-align: center;"
        @change="input1" 
        v-model="path"
        >
            <template #append>
                <!-- <el-button icon="Search" /> -->
                <el-icon @click="selectDir" style="cursor: pointer;"><Folder  /></el-icon>
            </template>
        </el-input>

        <el-button-group style="height: 5vh;" size="large">
            <el-button  type="primary" icon="ArrowLeft"  @click="showimage">效果图</el-button>
            <el-button  type="primary" @click="createinp">运行计算</el-button>
            <!-- <el-button type="primary" @click="createinp">
            生成inp<el-icon class="el-icon--right"><ArrowRight /></el-icon>
            </el-button> -->
            <el-button type="primary" @click="post">
            后处理<el-icon class="el-icon--right"><ArrowRight /></el-icon>
            </el-button>
        </el-button-group>

        
    </div>
<!-- 上侧单行参数输入框 -->
    <el-table :data="tabledata1" >
        <el-table-column label="长度(m)"   header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"  
              v-show="true"
              v-model="scope.row.length"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="宽度(m)"   header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.width"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="高度(m)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.height"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="簇间距(m)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.distance"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="裂缝高度(m)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.fracheight"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="网格大小(m)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.meshsize"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="CPU核数(个)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.cpu"
              
              :controls="false"
            />
          </template>
        </el-table-column>

    </el-table>

</template>

<script >
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
export default {
    data() {
        return {
            path: 'G:\\Model\\Abaqus\\project',
            length:120,
            tabledata1: [
                {
                    length:120,
                    width:50,
                    height:20,
                    distance:10,
                    fracheight:10,
                    meshsize:1,
                    cpu:12,
                },
            ],
        }
    },
    methods: {
      input1(e){
        console.log(e)
        this.path = e.target.value.split(',');
        this.$store.commit('changepath', this.path);
      },
      selectDir:async function(){
          const selected = await open({
              directory: true,
              multiple: false,
              defaultPath: await appDir(),
          });
          if (Array.isArray(selected)) {
              // user selected multiple directories
              this.path = selected;
              this.$store.commit('changepath',selected);
              this.base64code = 0;
          } else if (selected === null) {
              // user cancelled the selection
          } else {
              this.path = [selected];
              this.$store.commit('changepath', selected);
              this.base64code = 0;  
              console.log('88888888888888888888');
              console.log(this.$store.state.path);
              // user selected a single directory
          }
          
          // console.log(selected);
      },
      onAddItem() {
        console.log(this.tabledata2);
          this.tabledata2.push({
              rock:0,
              thick:0,
              E:0,
              u:0,
              angle:0,
              C:0,
              thermal:0,
              density:0,
              Q:0,
              heat:0,
          });
      },
      deleteRow(index) {
          this.tabledata2.splice(index, 1);
      },

      createinp(){
        // this.convertedArray = this.tabledata2.map(item => Object.values(item));
        console.log(this.convertedArray);
        // this.tabledata1=this.tabledata1.map(item=>Number(item));
        confirm('确定开始计算吗?', 
          { title: '警告', type: 'info',okLabel: '确定', cancelLabel: '取消' }
        ).then((e)=>{
          console.log(e);
          if(e){
            console.log(this.tabledata1[0].length)
            invoke('stressdirect',{
              path:this.path[0],
              jsonData:JSON.stringify(this.tabledata1[0]) ,
            });
          }
        }).catch((err)=>{
              console.log(err);
            });
      },
      showimage(){
        invoke('showimage', {
          path: this.path[0]
        })
      },
      post(){
        invoke('stressdirectpost', {
          path: this.path[0],
        }).then((res)=>{
          console.log(res);
        }).catch((err)=>{
          console.log(err);
        })
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

}

</script>

<style scoped>

</style>