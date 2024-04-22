<template>
  <div class="music-body">
    <el-table
      :data="tableData"
      style="width: 100%"
      size="small"
      :show-header="false"
    >
      <el-table-column width="40px">
        <template #default="{ $index }">
          {{ $index + 1 }}
        </template>
      </el-table-column>
      <el-table-column prop="file_name" />
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
  <div class="music-footer">
    <!-- <el-slider
      size='mini'
      v-model="currentProgress"
      :min="0"
      :max="100"
      :show-input="false"
      @change="changeProgress"
    ></el-slider> -->
    <!-- previous audio -->
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
    <!-- next audio -->
    <el-button link type="primary" class="no-shadow" @click="nextSong"
      ><el-icon><ArrowRightBold /></el-icon
    ></el-button>
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
.music-body {
  flex: 1; /* 设置 body 填充剩余空间 */
  overflow-y: auto; /* 当内容溢出时，显示滚动条 */
}

.music-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 50px;
  background-color: white;
}

.no-shadow {
  box-shadow: none !important;
}

::v-deep .el-table--border th.el-table__cell,
::v-deep .el-table td.el-table__cell {
  border-bottom: none !important;
}
::v-deep .el-table--border .el-table__cell {
  border-right:none !important;
}
::v-deep .el-table--group, .el-table--border{
  border: none !important;
}
</style>