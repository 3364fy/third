<template>
<div class="row " style="justify-content:space-around;box-sizing: border-box;height: 5%;margin: 10px 0 10px 0;">

  <input type="text" placeholder='请选择目录' @change="input1" v-model="path"  class="border center" style="width: 60%;">
  <button class=" border center " @click="selectDir">选择目录</button>
  <button class="border center" @click="add">+</button>
  <button class=" center border" @click="confirm">确定</button> 

</div>

<div class="scroll" style="height: 90%;">
  <div v-for="(item, index) in replace" :key="index"  class=" row" style="height: 8%;width: 100%;box-sizing: border-box;margin: 1px 0 1px 0;">
    <input type="text" :data-id="index" data-index="0" placeholder='煤层？' :value="replace[index][0]" @change="input"   class="border center aa"  >
    <input type="text" :data-id="index" data-index="1" placeholder='厚度' :value="replace[index][1]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="2" placeholder='弹性模量' :value="replace[index][2]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="3" placeholder='泊松比' :value="replace[index][3]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="4" placeholder='？' :value="replace[index][4]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="5" placeholder='？' :value="replace[index][5]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="6" placeholder='？' :value="replace[index][6]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="7" placeholder='导热性' :value="replace[index][7]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="8" placeholder='密度' :value="replace[index][8]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="9" placeholder='比热容' :value="replace[index][9]" @change="input"   class="border center aa" >
    <input type="text" :data-id="index" data-index="10" placeholder='热膨胀性' :value="replace[index][10]" @change="input"   class="border center aa" >
    <button class="border" :data-id="index" @click="reduce">-</button>
  </div>

</div>
  
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
      path: 'G:\\Model\\Abaqus\\project',
      macro:'G:\\desktop\\abaqusV.py',
      version:'2022',
      odbPaths: [],
      replace: [
        [0,3,21.0E9,0.30,34,34,6.1E6,2.21,2660,1100,0.70E-5],
        [0,1,26.0E9,0.27,39.5,39.5,12.6E6,2.21,2680,1100,0.70E-5],
        [0,2,21.0E9,0.30,34,34,6.1E6,2.21,2660,1100,0.70E-5],
        [0,1,26.0E9,0.27,39.5,39.5,12.6E6,2.21,2680,1100,0.70E-5],
        [0,5,21.0E9,0.30,34,34,6.1E6,2.21,2660,1100,0.70E-5],
        [1,3,3.0E9,0.25,25.0,25.0,3.0E6,0.5,1400,1670,0.90E-5],
        [0,2,26.0E9,0.27,39.5,39.5,12.6E6,2.21,2680,1100,0.70E-5],
        [0,3,21.0E9,0.30,34,34,6.1E6,2.21,2660,1100,0.70E-5],
        [0,2,26.0E9,0.27,39.5,39.5,12.6E6,2.21,2680,1100,0.70E-5],
    
      ],
    };
  },
  methods: {
    input1(e){
      this.path = e.target.value.split(',');
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
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        this.path = selected;
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


    confirm:async function(){
      console.log(this.replace);
      const res = await invoke('coal', {
        path: this.path,
        parameter: this.replace,
      });
      console.log(res);
    },

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
  box-shadow: 2px 2px 2px rgb(221, 148, 148); */
  box-sizing: border-box;
}

.scroll div input{
  width: 9%;
  border-radius: 0;
  margin: 0;
}



</style>