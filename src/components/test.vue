<template>
    <div class="drag">
    <div class="back_box"  ref="back_box">这是一个背景
        <div class="drag_box" 
        draggable="true"
        @dragstart="dragstart($event)"
        @dragend="dragend($event)"
        :style="`left:${elLeft}px;top:${elTop}px`"
        >
            这是一个蓝色可拖拽元素
        </div>
    </div>
  </div>
</template>

<style scoped>
.back_box {
  background: #ccc;
  width: 50vw;
  height: 50vh;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -30%);
}

.drag_box {
  position: absolute;
  width: 100px;
  height: 100px;
  background: skyblue;
  user-select: none; /* 不可选中,为了拖拽时不让文字高亮 */
  z-index: 999;
}
</style>

<script>
export default {
  data() {
    return{
        elLeft: 100, // 元素的左偏移量
        elTop: 100, // 元素的右偏移量
    }
  },
  methods: {
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
    dragend(e) {
        console.log('dragEnd', e)
        let x= e.clientX - this.startclintX
        let y= e.clientY - this.startclintY
        this.elLeft += x
        this.elTop += y
      
    }
  },
    mounted() {
      this.initBodySize()
    }
}

</script>