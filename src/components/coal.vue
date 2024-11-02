<template >
  
<div class="row " style="justify-content:space-around;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">
  <div class="row border" style="width: 70%;justify-content: space-between;border-radius: 0%;">
    <input type="text" placeholder='请选择目录' @change="input1" v-model="path"  class=" center" style="width: 95%;border-radius: 0%;">
    <!-- <button class=" border center " @click="selectDir">选择目录</button> -->
    <img src="../assets/文件夹.png" @click="selectDir" style="width: auto;height: 100%;">
  </div>
  
  <button class=" center border" @click="confirm1">生成文件</button> 
  <button class=" center border"  @click="showImageViewer">效果图</button>
  <button class=" center border" @click="post">后处理</button>

</div>

<div class="row" style="height: 8%;margin: 3px 0 3px 0;justify-content:space-around;">
  <div class="border center " style="width: 10%;">气化腔宽度(m)</div>
  <div class="border center " style="width: 10%;">上覆地应力梯度(MPa/100m)</div>
  <div class="border center " style="width: 10%;">水平最小地应力梯度(MPa/100m)</div>
  <div class="border center " style="width: 10%;">水平最大地应力梯度(MPa/100m)</div>
  <div class="border center " style="width: 10%;">煤层初始温度(℃)</div>
  <div class="border center " style="width: 10%;">气化温度(℃)</div>
  <div class="border center " style="width: 10%;">冷却温度(℃)</div>
  <div class="border center " style="width: 10%;">煤层中部深度(m)</div>
  <div class="border center " style="width: 10%;">气化运行当量压力(MPa/100m)</div>
  <div class="border center " style="width: 10%;">气化运行时间(天 )</div>
</div>


<div class="row " style="justify-content:space-around;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">

  <!-- <input type="text" placeholder='模型长度'  v-model="length"  class="border center" style="width: 10%;margin: 0%;"> -->
  <input type="text" placeholder='气化腔宽度' data-index="10"  v-model="gaplength" @change="input2" class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGV' data-index="11" v-model="SIGV" @change="input2" class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGh' data-index="12" v-model="SIGh" @change="input2" class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGH' data-index="13" v-model="SIGH" @change="input2" class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_INI' data-index="14"  v-model="TEMP_INI" @change="input2" class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_GAS'  v-model="TEMP_GAS"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_COL'  v-model="TEMP_COL"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='DEPTH_CEN'  v-model="DEPTH_CEN"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='DEPTH_CEN' data-index="15" @change="input2" v-model="GAS_PRES"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='DEPTH_CEN'  v-model="GAS_TIME"  class="border center" style="width: 10%;margin: 0%;">
</div>

<div class="row" style="height: 10%;margin: 3px 0 3px 0;">
    <div class="border center ">岩性</div>
    <div class="border center ">厚度(m)</div>
    <div class="border center ">弹性模量(GPa)</div>
    <div class="border center ">泊松比</div>
    <div class="border center ">内摩擦角(°)</div>
    <div class="border center ">粘聚力(MPa)</div>
    <div class="border center ">导热系数(W/(m·K))</div>
    <div class="border center ">密度(kg/m³)</div>
    <div class="border center ">比热容(J/(kg·K))</div>
    <div class="border center ">热膨胀系数(10⁻⁵/K)</div>
    <button class="border center"  @click="add">+</button>
</div>


<div class="scroll" style="height: 64%">
  <div v-for="(item, index) in replace" :key="index"  class=" row" style="height: 10%;width: 100%;box-sizing: border-box;margin: 1px 0 1px 0;">
    <!-- id表示第几行，index表示第几列 -->
    <input type="text" :data-id="index" data-index="0"  :value="replace[index][0]" @change="input"   class="border center "  >
    <input type="text" :data-id="index" data-index="1"  :value="replace[index][1]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="2"  :value="replace[index][2]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="3"  :value="replace[index][3]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="4"  :value="replace[index][4]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="5"  :value="replace[index][5]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="6"  :value="replace[index][6]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="7"  :value="replace[index][7]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="8"  :value="replace[index][8]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="9"  :value="replace[index][9]" @change="input"   class="border center " >
    <button class="border" :data-id="index"  @click="reduce">-</button>
  </div>

</div>

<!-- 弹窗 -->
<div class="back_box"  ref="back_box" :style="`z-index: ${PictureZIndex};`">
  <div v-if="imageViewerVisible" class="drag_box border1" draggable="true" @dragstart="dragstart($event)"
        @drag="drag($event)"
        @dragend="dragend($event)"
        :style="`left:${elLeft}px;top:${elTop}px`">
        <button class="close-button center" @click="closeImageViewer">X</button>
        <img  
        style="max-width: auto;height:100%;"
        class=""
        :src="'data:image/*;base64,'+base64code" 
        alt="找不到图片"    
        >
  </div>
</div>





  
</template>
<script >
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import ProgressBar from './Progressbar.vue';
import { getCurrentInstance } from 'vue';
import { ask } from '@tauri-apps/api/dialog';
import { message } from '@tauri-apps/api/dialog';
import { ref } from 'vue'
import { Search } from '@element-plus/icons-vue'
const input1 = ref('')
const input2 = ref('')
const input3 = ref('')
const select = ref('')
export default {
  components: {
    ProgressBar,
  },
  data() {
    return {
      path: 'G:\\Model\\Abaqus\\project',
      version:'2022',
      odbPaths: [],
      replace: [
        [0, 68, 26.0, 0.27, 39.5, 12.6 , 2.21, 2680 , 1100, 0.70],
        [0, 21, 21.0, 0.3,  34.0, 6.1,   2.21, 2660,  1100, 0.70],
        [0, 10, 26.0, 0.27, 39.5, 12.6 , 2.21, 2680 , 1100, 0.70],
        [0, 2,  21.0, 0.3,  34.0, 6.1,   2.21, 2660,  1100, 0.70],
        [1, 8,  3.0 , 0.25, 25.0, 3.0 ,  0.5,  1400 , 1670, 0.90],
        [0, 3.0,26.0, 0.27, 39.5, 12.6 , 2.21, 2680 , 1100, 0.70],
        [0, 10, 21.0, 0.3,  34.0, 6.1,   2.21, 2660,  1100, 0.70],
        [0, 10, 26.0, 0.27, 39.5, 12.6 , 2.21, 2680 , 1100, 0.70],
        [0, 16, 21.0, 0.3,  34.0, 6.1,   2.21, 2660,  1100, 0.70],
        [0, 62, 26.0, 0.27, 39.5, 12.6 , 2.21, 2680 , 1100, 0.70],
      ],
      limit:[
        ['岩性',[0,1],],
        ['厚度',[2,300],],
        ['弹性模量',[1,100],],
        ['泊松比',[0,0.5],],
        ['内摩擦角',[0,60],],
        ['粘聚力',[0,300],],
        ['导热系数',[0,30],],
        ['密度',[1000,3500],],
        ['比热容',[100,3000],],
        ['热膨胀系数',[0,30],],
        ['气化腔宽度',[5,100]],
        ['上覆地应力梯度',[1.5,3.0]],
        ['水平最小地应力梯度',[1.0,2.5]],
        ['水平最大地应力梯度',[1.0,2.5]],
        ['煤层初始温度',[20,300]],
        ['气化运行当量压力',[0,1.0]]
      ],

      length:120,
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
      imageViewerVisible: false,
      elLeft: 0, // 元素的左偏移量
      elTop: 0, // 元素的右偏移量
      base64code:0,
      PictureZIndex:-999
    };
  },
  methods: {
    input1(e){
      this.path = e.target.value.split(',');
      this.$store.commit('changepath', this.path);
      this.base64code = 0;
    },
    input2(e){
      if(e.target.value<this.limit[e.target.dataset.index][1][0]||e.target.value>this.limit[e.target.dataset.index][1][1]){
        message(`输入数据超出范围,${this.limit[e.target.dataset.index][0]}的取值范围为${this.limit[e.target.dataset.index][1]}`, { title: '错误', type: 'error' });
        throw new Error(`输入数据超出范围,${this.limit[e.target.dataset.index][0]}的取值范围为${this.limit[e.target.dataset.index][1]}`);
      }
    },
    input(e){
      console.log(e);
      console.log(this.replace);
      if (e.target.dataset.index==0&&(e.target.value!=0&&e.target.value!=1)){
        
        message('岩性只能为0或1', { title: '错误', type: 'error' });
        throw new Error('岩性只能为0或1');
      }
      else{
        this.replace[e.target.dataset.id][e.target.dataset.index] =Number(e.target.value);
      }

      if(e.target.dataset.index!=0&&(
        e.target.value<this.limit[e.target.dataset.index][1][0]||e.target.value>this.limit[e.target.dataset.index][1][1]
      )){
        message(`输入数据超出范围,${this.limit[e.target.dataset.index][0]}的取值范围为${this.limit[e.target.dataset.index][1]}`, { title: '错误', type: 'error' });
        throw new Error(`输入数据超出范围,${this.limit[e.target.dataset.index][0]}的取值范围为${this.limit[e.target.dataset.index][1]}`);
      }else{
        this.replace[e.target.dataset.id][e.target.dataset.index] =Number(e.target.value);
      }
    },
    add(e){
      this.replace.push([0,0,0,0,0,0,0,0,0,0,0]);
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


    confirm1(){
      // console.log(this.replace);
      confirm('确定生成输入文件吗?', 
        { title: '警告', type: 'info',okLabel: '确定', cancelLabel: '取消' }
      ).then((e)=>{
        if (e) {
          for(var i in this.replace){
            for(var j in this.replace[i]){
              // console.log(this.replace[i][j]);
              if(j==0){
                console.log(this.replace[i][j]);
                if(this.replace[i][j]!=0&&this.replace[i][j]!=1){
                  message('岩性只能为0或1', { title: '错误', type: 'error' });
                  throw new Error('岩性只能为0或1');
                }
              }
              else if(this.replace[i][j]<this.limit[j][1][0]||this.replace[i][j]>this.limit[j][1][1]){
                console.log(this.replace[i][j]);
                message(`输入数据超出范围,${this.limit[j][0]}的取值范围为${this.limit[j][1]}`, { title: '错误', type: 'error' });
                throw new Error(`输入数据超出范围,${this.limit[j][0]}的取值范围为${this.limit[j][1]}`);
              }
            }
          }
        }
      }).then((res)=>{
        console.log('pppppppppppppp');
        console.log(res);
        invoke('coal', {
          path: this.path,
          parameter: this.replace,
          length: Number(this.length),
          gaplength: Number(this.gaplength)/2,
          sigv: Number(this.SIGV),
          sigh: Number(this.SIGh),
          sigH: Number(this.SIGH),
          tempini: Number(this.TEMP_INI),
          tempgas: Number(this.TEMP_GAS),
          tempcol: Number(this.TEMP_COL),
          depthcen: Number(this.DEPTH_CEN),
          gaspres: Number(this.GAS_PRES)*10000,
          gastime: Number(this.GAS_TIME)*3600*24,
        });
        console.log(res);
              
      }).catch((err)=>{
        console.log(err);
      })
   },

    showImageViewer() {
      
      // 点击按钮时显示弹窗
      if(this.base64code!=0){
        this.imageViewerVisible = true; 
        this.PictureZIndex=998
      }
      else{
        invoke('base',{
          path:this.path+'\\1.png'
        }).then((res)=>{
          this.base64code = res;
          this.imageViewerVisible = true; 
          this.PictureZIndex=998
        }).catch((err)=>{
          console.log(err);
          // ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', type: 'warning' });
          message('没有找到指定文件', { title: '错误', type: 'error' });
        })
        // console.log('fffffffffffffffff');
        // console.log(base64code);
      }

    },
    closeImageViewer() {
      this.imageViewerVisible = false; // 关闭弹窗
      this.PictureZIndex=-998
    },
    initBodySize() {
      this.initWidth =this.$refs.back_box.clientWidth//获取背景盒子的宽度
    //   this.initHeight = this.initWidth*((1080*0.88)/(1920-1080*0.02))
    this.initHeight = this.initWidth * (1080 / 1920);
    },
    dragstart(e) {
      console.log('dragStart', e)
      this.startclintX = e.clientX
      this.startclintY = e.clientY
    },

    drag(e) {
      console.log('drag', e)
      let x= e.clientX - this.startclintX
      let y= e.clientY - this.startclintY
      this.elLeft += x
      this.elTop += y
    },

    dragend(e) {
        console.log('dragEnd', e)
        let x= e.clientX - this.startclintX
        let y= e.clientY - this.startclintY
        this.elLeft += x
        this.elTop += y
      
    },
    
    post(){
      invoke('post', {
        path: this.path,
      }).then((res)=>{
        console.log(res);
      }).catch((err)=>{
        console.log(err);
      })
    }

  },

  beforeRouteEnter(to, from, next) {
    console.log('beforeRouteEnter');
    next(vm => {
    // 访问组件实例 `vm`
      console.log(vm.$store.state.path);
      console.log('================');
      if (vm.$store.state.path!='E:/Office'){
        vm.path = vm.$store.state.path;
      }
      }
    );
  },
  mounted() {
      this.initBodySize()
    },
  computed: {
    imageSrc() {
      return require(`${this.path}\\1.png`);
    },
  },
};

</script>

<style scoped>

.input-with-select .el-input-group__prepend {
  background-color: var(--el-fill-color-blank);
}
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
  box-shadow: 2px 2px 2px rgb(221, 1410, 1410); */
  box-sizing: border-box;
}

.scroll div input{
  width: 8%;
  border-radius: 5px;
  margin: 0 3px 0 3px;
}

.row div{
  width: 8%;
  border-radius: 5px;
  margin: 0 3px 0 3px;
}


.back_box {
  background: rgba(0, 0, 0, 0);
  width: 98vw;
  height: 100vh;
  position: fixed;
  top: 0;
  left: 0;
  transform: translate(-10%, -10%);
  /* filter: blur(1px); */
}

.drag_box {
  /* max-width: 80vw;
  max-height: 80vh; */
  position: absolute;
  /* background: rgb(243, 131, 46); */
  background: linear-gradient(to bottom, rgb(23, 23, 23,0.5), rgb(255, 255, 255,1));
  backdrop-filter: blur(100px);
  /* 不可选中,为了拖拽时不让文字高亮 */
  user-select: none; 
  
  z-index: 999;
}

.border1{
  border: 0px solid rgb(12, 1, 21);
  box-shadow: 20px 20px 20px rgb(10, 10, 10);
  box-sizing: border-box;
  border-radius: 10px;
}

.close-button {
  position: absolute;
  height: 5px;
  top: 0;
  right: 0;
  background: transparent;
  border: none;
  font-size: 1.5em;
  cursor: pointer;
}

</style>