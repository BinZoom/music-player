<template>
  <div class="content">
    <div class="music-container">
      <el-table :data="tableData" style="width: 100%" size="small">
        <el-table-column label="No" width="40px">
          <template #default="{ $index }">
            {{ $index + 1 }}
          </template>
        </el-table-column>
        <el-table-column prop="file_name" label="Name" />
        <el-table-column width="100">
          <template #default="{ row }">
            <el-button
              link
              type="primary"
              class="no-shadow"
              @click="playAudio(row.file_name)"
            >
              <el-icon size="20px"><CaretRight /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    <div class="music-toolbar">
      <!-- 上一首歌按钮 -->
      <el-button link type="primary" class="no-shadow" @click="prevSong"
        ><el-icon><ArrowLeftBold /></el-icon
      ></el-button>
      <!-- 播放/暂停按钮 -->
      <el-button
        link
        type="primary"
        class="no-shadow"
        v-if="!isPlaying"
        @click="play"
        ><el-icon size="35px"><CaretRight /></el-icon
      ></el-button>
      <el-button v-else @click="pause"></el-button>
      <!-- 下一首歌按钮 -->
      <el-button link type="primary" class="no-shadow" @click="nextSong"
        ><el-icon><ArrowRightBold /></el-icon
      ></el-button>
      <!-- 歌曲进度条 -->
      <!-- <el-slider
      v-model="currentProgress"
      :min="0"
      :max="100"
      :show-input="false"
      @change="changeProgress"
    ></el-slider> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {
  CaretRight,
  ArrowLeftBold,
  ArrowRightBold,
} from "@element-plus/icons-vue";

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

async function playAudio(file_name: String) {
  isPlaying.value = true;
  // currentMusic.value = file_name;
  const file_path = musicHubPath.value + file_name;
  const event: CustomEventPayload = { action: "play", file_path };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    console.error(error);
  }
}

async function pauseAudio() {
  isPlaying.value = false;
  const event: CustomEventPayload = { action: "pause" };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
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

<style scoped>
.container {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  width: 100%;
}

.music-container {
  margin-bottom: 70px; /* 为底部固定的 div 留出空间 */
  background-color: #ccc; /* 假设第二个 div 的背景颜色为灰色 */
}

.music-toolbar {
  position: fixed;
  left: 0;
  bottom: 0;
  width: 100%; /* 让 div 充满整个页面宽度 */
  background-color: white; /* 设置背景颜色，可根据需要修改 */
  height: 70px;
  padding: 18px; /* 设置内边距，可根据需要修改 */
  box-sizing: border-box; /* 确保 padding 不会影响到 div 的宽度 */
}

.no-shadow {
  box-shadow: none !important;
}
</style>