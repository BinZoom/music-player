<template>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column prop="id" label="序号" width="180" />
    <el-table-column prop="name" label="歌曲名" />
  </el-table>

  <div class="music-toolbar">
    <!-- 上一首歌按钮 -->
    <el-button @click="prevSong">上一首</el-button>
    <!-- 播放/暂停按钮 -->
    <el-button v-if="!isPlaying" @click="play">开始</el-button>
    <el-button v-else @click="pause">暂停</el-button>
    <!-- 下一首歌按钮 -->
    <el-button  @click="nextSong">下一首</el-button>
    <!-- 歌曲进度条 -->
    <el-slider v-model="currentProgress" :min="0" :max="100" :show-input="false" @change="changeProgress"></el-slider>
  </div>

  <!-- <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p> -->
</template>


<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {VideoPlay } from '@element-plus/icons-vue'

const greetMsg = ref("");
const name = ref("");

const isPlaying = ref(false); // 是否正在播放
const currentProgress = ref(0); // 当前歌曲进度


const prevSong = () => {
      // 上一首歌逻辑
    };

const nextSong = () => {
  // 下一首歌逻辑
};

const play = () => {
  isPlaying.value = true
  invoke("play");
  // 播放/暂停逻辑
};

const pause = () => {
  isPlaying.value = false
  // 播放/暂停逻辑
};

const changeProgress = (value: number) => {
  // 改变歌曲进度逻辑
  currentProgress.value = value;
};

const tableData = [
  {
    id: '1',
    name: '希望你被这个世界爱着',
  },
  {
    id: '2',
    name: '盛夏的果实',
  }
];

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>