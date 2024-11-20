<template>
<!-- 上侧导航栏 -->
  <el-row style="margin-bottom: 10px;">
    <el-col :span="19">
        <el-input
        style="max-width: 60vw;height: 5vh;"
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
      </el-col>

      <el-col :span="5" >
        <el-button-group  style="height: 5vh;" size="large">
            <el-button  type="primary" icon="ArrowLeft" :disabled="confirm_disabled" @click="confirm">确定</el-button>
            
            <el-button type="primary" :disabled='start_disabled' @click="aftertreat">
            全部开始<el-icon class="el-icon--right" ><ArrowRight/></el-icon>
            </el-button>
        </el-button-group>
      </el-col>   
  </el-row>

<!-- 选择宏-->
  <el-row style="margin-bottom: 10px;">
    <el-col :span="19">
      <el-input
        style="max-width: 60vw;height: 5vh;"
        placeholder="Please input"
        class="input-with-select"
        input-style="text-align: center;"
        @change="input" 
        v-model="macro"
        >
            <template #append>
                <!-- <el-button icon="Search" /> -->
                <el-icon @click="selectfile" style="cursor: pointer;"><Folder  /></el-icon>
            </template>
        </el-input>
    </el-col>
    <el-col :span="5">
      <el-input
        v-model="version"
        style="max-width: 600px;"
        placeholder="Please input"
        input-style="text-align: center;"
        size = "large"
      >
        <template #prepend>Abaqus版本号：</template>
      </el-input>
    </el-col>
    <!-- <el-col :span="1">
      <el-button size="large" type="primary" @click="add">+</el-button>
    </el-col> -->
  </el-row>


  

<!-- 下侧功能区 -->
    <el-table :data="tabledata"  max-height="25vh" style="margin-bottom: 10px;">
      <el-table-column label="字符串"  header-align="center">
        <template #default="scope">
            <el-input
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.str"
            />
          </template>
        </el-table-column>
        <el-table-column label="替换字符串"  header-align="center">
          <template #default="scope">
              <el-input
                input-style="text-align: center;"
                v-show="true"
                v-model="scope.row.replacestr"
              />
            </template>
        </el-table-column>
        <el-table-column fixed="right" min-width="10" align="center" header-align="center">
            <template #header>
                <el-button
                    type="primary"
                    size="large"
                    @click="add"
                >
                    +
                </el-button>
            </template>

            <template #default="scope">
                <el-button
                link
                type="primary"
                size="small"
                @click.prevent="deleteRow(scope.$index)"
                >
                删除
                </el-button>
            </template>
        </el-table-column>
    </el-table>

    <el-scrollbar  style="padding: 10px;height: 50vh;flex-grow: 1;boxShadow:--el-box-shadow-dark;">
      <el-card v-for="odbpath in odbPaths" class="box-card" style="height:30px;margin: 2px 0 0 0;display: flex; text-align: center;align-items: center;">
        
          <div >{{ odbpath }}</div> 
       
      </el-card>
    </el-scrollbar>
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
      replace: [],
      tabledata: [
        {
          str:"",
          replacestr:"{pwd}\\{0}",
        },
        {
          str:"",
          replacestr:"{odb}",
        },
      ],
    };
  },
  methods: {
    input1(e){
      this.path = e.target.value.split(',');
      this.$store.commit('changepath', this.path);
    },
    input(e){
      // 用以实现宏文件的字符串替换功能
      console.log(e);
      this.replace[e.target.dataset.id][e.target.dataset.index] = e.target.value;
    },
    add(e){
      this.tabledata.push(
        {
          str:"",
          replacestr:"",
        }
      );
      console.log(this.version);
      console.log(this.replace);
    },
    deleteRow(index) {
            this.tabledata.splice(index, 1);
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
        this.$store.commit('changepath', selected);
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

    aftertreat(){
      this.replace = this.tabledata.map(item => Object.values(item));
      console.log(this.replace);
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

  },
  beforeRouteEnter(to, from, next) {
    console.log('beforeRouteEnter');
    next(vm => {
    // 访问组件实例 `vm`
      console.log(vm.$store.state.path);
      console.log('================');
      if (vm.$store.state.path!='E:/Office'){
        vm.path = [vm.$store.state.path];
      }
      }
    );
  },
};

</script>

<style>
@import "../styles.css";


</style>
