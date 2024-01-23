<template>
    <div class="progressbar">
      <div class="progressbarfill" :style="{ width: `${progress}%` }">{{ progress }}%</div>
    </div>
</template>
  
<script>
export default {
props: {
    progress: {
    type: Number,
    required: true,
    },
},
};
</script>

<style>

.progressbar {
width: 100%;
background-color: #f3f3f3;
border-radius: 5px;
}

.progressbarfill {
height: 20px;
background-color: #4caf50;
border-radius: 5px;
transition: width 0.4s ease-in-out;
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
  
}

</style>