<template >
<div class="row " style="justify-content:space-around;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">

  <div class="row border" style="width: 70%;justify-content: space-between;border-radius: 0%;">
    <input type="text" placeholder='请选择目录' @change="input1" v-model="path"  class=" center" style="width: 95%;border-radius: 0%;">
    <!-- <button class=" border center " @click="selectDir">选择目录</button> -->
    <img src="../assets/文件夹.png" @click="selectDir" style="width: auto;height: 100%;">
  </div>
  
  <button class=" center border" @click="confirm">生成文件</button> 
  <button class=" center border"  @click="showImageViewer">效果图</button>
  <button class=" center border">后处理</button>

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
  <input type="text" placeholder='缺口长度'  v-model="gaplength"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGV'  v-model="SIGV"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGh'  v-model="SIGh"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='SIGH'  v-model="SIGH"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_INI'  v-model="TEMP_INI"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_GAS'  v-model="TEMP_GAS"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='TEMP_COL'  v-model="TEMP_COL"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='DEPTH_CEN'  v-model="DEPTH_CEN"  class="border center" style="width: 10%;margin: 0%;">
  <input type="text" placeholder='DEPTH_CEN'  v-model="GAS_PRES"  class="border center" style="width: 10%;margin: 0%;">
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
    <input type="text" :data-id="index" data-index="0"  :value="replace[index][0]" @change="input"   class="border center "  >
    <input type="text" :data-id="index" data-index="1"  :value="replace[index][1]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="2"  :value="replace[index][2]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="3"  :value="replace[index][3]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="5"  :value="replace[index][4]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="6"  :value="replace[index][5]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="7"  :value="replace[index][6]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="10"  :value="replace[index][7]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="9"  :value="replace[index][8]" @change="input"   class="border center " >
    <input type="text" :data-id="index" data-index="10"  :value="replace[index][9]" @change="input"   class="border center " >
    <button class="border" :data-id="index"  @click="reduce">-</button>
  </div>

</div>

<!-- 弹窗 -->
<div class="back_box"  ref="back_box">
  <div v-if="imageViewerVisible" class="image-viewer " >
        <img class="drag_box border1" 
        @click="closeImageViewer" 
        src="../assets/1.png" 
        draggable="true"
        @dragstart="dragstart($event)"
        @drag="drag($event)"
        @dragend="dragend($event)"
        :style="`left:${elLeft}px;top:${elTop}px`"
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
export default {
  components: {
    ProgressBar,
  },
  data() {
    return {
      path: 'G:\\Model\\Abaqus\\project',
      macro:'G:\\desktop\\abaqusV.py',
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
      elLeft: 100, // 元素的左偏移量
      elTop: 100, // 元素的右偏移量
    };
  },
  methods: {
    input1(e){
      this.path = e.target.value.split(',');
      this.$store.commit('changepath', this.path);
    },
    input(e){
      console.log(e);
      this.replace[e.target.dataset.id][e.target.dataset.index] =Number(e.target.value);
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
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        this.path = selected;
        this.$store.commit('changepath', selected);
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


    confirm:async function(){
      console.log(this.replace);
      const res = await invoke('coal', {
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
    },

    showImageViewer() {
      this.imageViewerVisible = true; // 点击按钮时显示弹窗
    },
    closeImageViewer() {
      this.imageViewerVisible = false; // 关闭弹窗
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
    }
};

</script>

<style scoped>
@import "../styles.css";
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

.image-viewer {
  position: absolute;
  top: 0;
  left: 0;
  /* width: 50%;
  height: 50%; */
  /* background-color: rgba(0, 0, 0, 0.8); */
  display: flex;
  justify-content: center;
  align-items: center;
}

.image-viewer img {
  max-width: 100%;
  max-height: 80vh;
}

.back_box {
  background: rgba(0, 0, 0, 0);
  width: 50vw;
  height: 50vh;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -30%);
}

.drag_box {
  max-width: 100vw;
  max-height: 80vh;
  position: absolute;
  /* width: 100px;
  height: 100px; */
  background: skyblue;
  user-select: none; /* 不可选中,为了拖拽时不让文字高亮 */
  z-index: 999;
}

.border1{
  border: 0px solid rgb(12, 1, 21);
  box-shadow: 20px 20px 20px rgb(10, 10, 10);
  box-sizing: border-box;
  border-radius: 5px;
}



</style>