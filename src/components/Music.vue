<template>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column label="序号" width="100">
      <template #default="{ $index }">
        {{ $index + 1 }}
      </template>
    </el-table-column>
    <el-table-column prop="file_name" label="歌曲名" width="500" />
    <el-table-column label="操作" width="100">
      <template #default="{ scope }">
        <el-button @click="play(scope.row)">播放</el-button>
      </template>
    </el-table-column>
  </el-table>

  <div class="music-toolbar">
    <!-- 上一首歌按钮 -->
    <el-button @click="prevSong">上一首</el-button>
    <!-- 播放/暂停按钮 -->
    <el-button v-if="!isPlaying" @click="play">开始</el-button>
    <el-button v-else @click="pause">暂停</el-button>
    <!-- 下一首歌按钮 -->
    <el-button @click="nextSong">下一首</el-button>
    <!-- 歌曲进度条 -->
    <el-slider
      v-model="currentProgress"
      :min="0"
      :max="100"
      :show-input="false"
      @change="changeProgress"
    ></el-slider>
  </div>
</template>


<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const isPlaying = ref(false); // 是否正在播放
const currentProgress = ref(0); // 当前歌曲进度
const tableData = ref([]);

const musicHubPath = ref("E://music/"); // 音乐存放目录
const currentMusic = ref("");

interface CustomEventPayload {
  action: "play" | "pause";
  file_path?: string;
}

const prevSong = () => {
  // 上一首歌逻辑
};

const nextSong = () => {
  // 下一首歌逻辑
};

async function playAudio(row: any) {
  isPlaying.value = true;
  currentMusic.value = row.file_name;
  const file_path = musicHubPath.value + row.file_name;
  const event: CustomEventPayload = { action: "play", file_path };
  try {
    await invoke("tauri://custom-event", { event });
  } catch (error) {
    console.error(error);
  }
}

async function pause() {
  isPlaying.value = false;
  const event: CustomEventPayload = { action: "pause" };
  try {
    await invoke("tauri://custom-event", { event });
  } catch (error) {
    console.error(error);
  }
}

const changeProgress = (value: number) => {
  // 改变歌曲进度逻辑
  currentProgress.value = value;
};

const getFileList = () => {
  invoke("scan_files_in_directory", {
    path: musicHubPath.value,
  }).then((res: any) => {
    console.log(res);
    tableData.value = res;
  });
};

onMounted(() => {
  getFileList();
});
</script>