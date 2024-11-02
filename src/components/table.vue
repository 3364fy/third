<template>
    <div class="mt-4 row" style=" justify-content: space-evenly;margin: 10px 0 10px 0;">
        <el-input
        style="max-width: 600px;height: 5vh;"
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
            <el-button  type="primary" @click="createinp">生成inp</el-button>
            <!-- <el-button type="primary" @click="createinp">
            生成inp<el-icon class="el-icon--right"><ArrowRight /></el-icon>
            </el-button> -->
            <el-button type="primary" @click="post">
            后处理<el-icon class="el-icon--right"><ArrowRight /></el-icon>
            </el-button>
        </el-button-group>

        
    </div>
<!-- 上侧单行参数输入框 -->
    <el-table :data="tabledata1" style="width: 100%">
        <el-table-column label="气化腔宽度(m)" width="180"  header-align="center">
        <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.gaplength"
              :min="5"
              :max="100"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGV" label="上覆地应力梯度(MPa/100m)"  width="180" header-align="center">
        <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.SIGV"
              :min="1.5"
              :max="3.0"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGh" label="水平最小地应力梯度(MPa/100m)" width="180"  header-align="center">
        <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.SIGh"
              :min="1.0"
              :max="2.5"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="SIGH" label="水平最大地应力梯度(MPa/100m)"  width="180" header-align="center">
        <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.SIGH"
              :min="1.0"
              :max="2.5"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="TEMP_INI" label="煤层初始温度(℃)"  width="180" header-align="center">
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.TEMP_INI"
              :min="20"
              :max="300"
              :controls="false"
            />
          </template>
        </el-table-column>
        
        <el-table-column prop="TEMP_GAS" label="气化温度(℃)" width="180"  header-align="center">
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.TEMP_GAS"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="TEMP_COL" label="冷却温度(℃)"  width="180" header-align="center">
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.TEMP_COL"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="DEPTH_CEN" label="煤层中部深度(m)"  width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.DEPTH_CEN"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="GAS_PRES" label="气化运行当量压力(MPa/100m)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.GAS_PRES"
              :min="0"
              :max="1.0"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column prop="GAS_TIME" label="气化运行时间(天 )" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.GAS_TIME"
              :controls="false"
            />
          </template>
        </el-table-column>

    </el-table>
<!-- 下侧多行参数输入框 -->
   
    <el-table :data="tabledata2" height="72vh" style="width: 100%;margin:10px 0 10px 0 ;"  >
        <el-table-column  prop="date" label="岩性" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.rock"
              :min="0"
              :max="1"
              :step="1"
              :step-strictly="true"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="厚度(m)" width="180"  header-align="center">
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.thick"
              :min="2"
              :max="300"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="弹性模量(GPa)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.E"
              :min="1"
              :max="100"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="泊松比" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.u"
              :min="0"
              :max="0.5"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column label="内摩擦角(°)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.angle"
              :min="0"
              :max="60"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="粘聚力(MPa)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.C"
              :min="0"
              :max="300"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="导热系数(W/(m·K))" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.thermal"
              :min="0"
              :max="30"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column  label="密度(kg/m³)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.density"
              :min="1000"
              :max="3500"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column label="比热容(J/(kg·K))" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.Q"
              :min="100"
              :max="3000"
              :controls="false"
            />
          </template>
        </el-table-column>

        <el-table-column label="热膨胀系数(10⁻⁵/K)" width="180" header-align="center" >
            <template #default="scope">
            <el-input-number
              input-style="text-align: center;"
              v-show="true"
              v-model="scope.row.heat"
              :min="0"
              :max="30"
              :controls="false"
            />
          </template>
        </el-table-column>



        <el-table-column fixed="right" min-width="70" align="center" header-align="center">
            <template #header>
                <el-button
                    type="primary"
                    size="large"
                    @click="onAddItem"
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
    
</template>

<script>
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
                    gaplength:20,
                    SIGV:2.6,
                    SIGh:1.7,
                    SIGH:2.4,
                    TEMP_INI:44.10,
                    TEMP_GAS:1200.0,
                    TEMP_COL:200.0,
                    DEPTH_CEN : 1330,
                    GAS_PRES:0.3,
                    GAS_TIME:45,
                },
            ],
            tabledata2:[
                
                {
                    rock:0,
                    thick:68,
                    E:26,
                    u:0.27,
                    angle:39.5,
                    C:12.6,
                    thermal:2.21,
                    density:2680,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:21,
                    E:21,
                    u:0.3,
                    angle:34,
                    C:6.1,
                    thermal:2.21,
                    density:2660,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:10,
                    E:26,
                    u:0.27,
                    angle:39.5,
                    C:12.6,
                    thermal:2.21,
                    density:2680,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:2,
                    E:21,
                    u:0.3,
                    angle:34,
                    C:6.1,
                    thermal:2.21,
                    density:2660,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:1,
                    thick:8,
                    E:3,
                    u:0.25,
                    angle:25,
                    C:3,
                    thermal:0.5,
                    density:1400,
                    Q:1670,
                    heat:0.9,
                },
                {
                    rock:0,
                    thick:3,
                    E:26,
                    u:0.27,
                    angle:39.5,
                    C:12.6,
                    thermal:2.21,
                    density:2680,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:10,
                    E:21,
                    u:0.3,
                    angle:34,
                    C:6.1,
                    thermal:2.21,
                    density:2660,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:10,
                    E:26,
                    u:0.27,
                    angle:39.5,
                    C:12.6,
                    thermal:2.21,
                    density:2680,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:16,
                    E:21,
                    u:0.3,
                    angle:34,
                    C:6.1,
                    thermal:2.21,
                    density:2660,
                    Q:1100,
                    heat:0.7,
                },
                {
                    rock:0,
                    thick:62,
                    E:26,
                    u:0.27,
                    angle:39.5,
                    C:12.6,
                    thermal:2.21,
                    density:2680,
                    Q:1100,
                    heat:0.7,
                },
            ]
        }
    },
    methods: {
      input1(e){
        pr
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
                this.path = selected;
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
          this.convertedArray = this.tabledata2.map(item => Object.values(item));
          console.log(this.convertedArray);
          confirm('确定生成输入文件吗?', 
            { title: '警告', type: 'info',okLabel: '确定', cancelLabel: '取消' }
          ).then((e)=>{
            console.log(e);
            if(e){
              console.log(this.tabledata1[0].gaplength)
              invoke('coal', {
                path: this.path,
                parameter: this.convertedArray,
                length: Number(this.length),
                gaplength: Number(this.tabledata1[0].gaplength)/2,
                sigv: Number(this.tabledata1[0].SIGV),
                sigh: Number(this.tabledata1[0].SIGh),
                sigH: Number(this.tabledata1[0].SIGH),
                tempini: Number(this.tabledata1[0].TEMP_INI),
                tempgas: Number(this.tabledata1[0].TEMP_GAS),
                tempcol: Number(this.tabledata1[0].TEMP_COL),
                depthcen: Number(this.tabledata1[0].DEPTH_CEN),
                gaspres: Number(this.tabledata1[0].GAS_PRES)*10000,
                gastime: Number(this.tabledata1[0].GAS_TIME)*3600*24,
              });
            }
          }).catch((err)=>{
                console.log(err);
             });
        },
        showimage(){
          invoke('showimage', {
            path: this.path
          })
        },
        post(){
          invoke('post', {
            path: this.path,
          }).then((res)=>{
            console.log(res);
          }).catch((err)=>{
            console.log(err);
          })
        },


    },

}

</script>

<style  scoped>

</style>