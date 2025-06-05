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

    <el-table :data="tabledata2" >
        <el-table-column label="&sigma;11(Pa)"   header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"  
              v-show="true"
              v-model="scope.row.sigma11"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="&sigma;22(Pa)"   header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.sigma22"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="&sigma;33(Pa)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.sigma33"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="弹性模量(Pa)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.E"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="泊松比"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.u"
              
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="裂缝面压力(Pa)"  header-align="center">
        <template #default="scope">
            <el-input-number
              style="width: 100%;"
              v-show="true"
              v-model="scope.row.F"
              
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
            tabledata2: [
                {
                    sigma11:-58000000.0,
                    sigma22:-65000000.0,
                    sigma33:-78000000.0,
                    E:15E9,
                    u:0.2,
                    F:70000000,
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

      createinp(){
        // this.convertedArray = this.tabledata2.map(item => Object.values(item));
        // console.log(this.convertedArray);
        // this.tabledata1=this.tabledata1.map(item=>Number(item));
        // this.tabledata2=this.tabledata2.map(item=>Number(item));
        const mergedData = { ...this.tabledata1[0], ...this.tabledata2[0] };
        // const mergedData=Object.values(merged).map(item => Number(item));
        console.log(mergedData);
        confirm('确定开始计算吗?', 
          { title: '警告', type: 'info',okLabel: '确定', cancelLabel: '取消' }
        ).then((e)=>{
          console.log(e);
          if(e){
            console.log(mergedData.length)
            invoke('stressdirect',{
              path:this.path[0],
              jsonData:JSON.stringify(mergedData) ,
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