<template>
  <!-- <el-row class="tac" > -->
    
    <!-- <el-col :span="12"> -->
      <el-menu
        active-text-color="#ffd04b"
        background-color="#545c64"
        class="el-menu-vertical-demo"
        default-active="2"
        text-color="#fff"
        @open="handleOpen"
        @close="handleClose"
      >
        <el-sub-menu  index="1">
          <template #title>
            <el-icon><location /></el-icon>
            <span>算例</span>
          </template>
              <router-link to="/table">
                <el-menu-item index="1-1">煤层燃烧</el-menu-item>
              </router-link> -->

              <router-link to="/fracDirect">
                <el-menu-item index="1-2">应力转向</el-menu-item>
              </router-link>

              <!-- <router-link to="/coal">
                <el-menu-item index="1-3">表格</el-menu-item>
              </router-link> -->
            
            
          <!-- </el-menu-item-group> -->

          <!-- <el-menu-item-group title="Abaqus">
            <el-menu-item index="1-3">item three</el-menu-item>
          </el-menu-item-group>

          <el-sub-menu index="1-4">
            <template #title>item four</template>
            <el-menu-item index="1-4-1">item one</el-menu-item>
          </el-sub-menu> -->

        </el-sub-menu>

        
        <!-- <el-sub-menu index="2">
          <template #title>
            <el-icon><icon-menu /></el-icon>
            <span>Abaqus</span>
          </template>

          <router-link to="/home">
            <el-menu-item index="2-1">模拟</el-menu-item>
          </router-link>

          <router-link to="/index">
            <el-menu-item index="2-2">后处理</el-menu-item>
          </router-link>
          
          
  
        </el-sub-menu>
        
        <el-sub-menu index="3">
          <template #title>
            <el-icon><setting /></el-icon>
            <span>Fluent</span>
          </template>
          <router-link to="/fluent">
            <el-menu-item index="3-1">模拟</el-menu-item>
          </router-link>

          
  
        </el-sub-menu> -->

      </el-menu>
    <!-- </el-col> -->
  <!-- </el-row> -->
</template>

<script lang="ts" setup>
import {
  // Menu as IconMenu,
  Location,
  // Setting,
} from '@element-plus/icons-vue'
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>
