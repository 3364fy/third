<template>
    <div class="progressbar">
      <div class="progressbarfill" :style="{ width: `${progress}%`}"></div>
      <div class="centered-content">第{{step}}步：{{ progress }}%</div> 
    </div>
</template>
  
<script>
import { type } from '@tauri-apps/api/os';

export default {
props: {
    progress: {
    type: Number,
    required: true,
    },
    step: {
      type:Number,
      required:true,
    }
},
};
</script>

<style>

.progressbar {
position: relative;
width: 100%;
background-color: #f3f3f3;
border-radius: 5px;
}

.progressbarfill {
height: 20px;
background-color: #4caf50;
border-radius: 5px;
transition: width 0.4s ease-in-out;
/* height: auto; */
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
  
}

.centered-content {
  position: absolute; /* 设置为 absolute，这样它就会脱离正常的文档流 */
  /* 将其定位到父元素的中心 */
  top: 50%; 
  left: 50%;
  transform: translate(-50%, -50%); /* 使用 transform 来确保元素的中心与父元素的中心对齐 */
  /* 你可以在这里添加你需要的样式 */
}

</style>