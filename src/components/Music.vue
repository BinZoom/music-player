<template>
  <div class="music-body">
    Current Music: {{ currentMusic }}
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
            class="hover-visible-button no-shadow"
            @click="playAudio(row.file_name)"
          >
            <el-icon size="20px"><CaretRight /></el-icon>
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
  <div class="music-footer">
    <el-button link type="primary" class="no-shadow" @click="prevSong"
      ><el-icon><ArrowLeftBold /></el-icon
    ></el-button>
    <!-- 播放/暂停按钮 -->
    <el-button
      link
      type="primary"
      class="no-shadow"
      v-if="!isPlaying"
      @click="recoveryAudio()"
      ><el-icon size="35px"><CaretRight /></el-icon
    ></el-button>
    <el-button
      v-else
      link
      type="primary"
      class="no-shadow"
      @click="pauseAudio()"
      ><el-icon size="35px"><VideoPause /></el-icon>
    </el-button>
    <!-- next audio -->
    <el-button link type="primary" class="no-shadow" @click="nextSong"
      ><el-icon><ArrowRightBold /></el-icon
    ></el-button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessage } from "element-plus";
import {
  CaretRight,
  ArrowLeftBold,
  ArrowRightBold,
  VideoPause,
} from "@element-plus/icons-vue";

const isPlaying = ref(false); // 是否正在播放
const tableData = ref([]);

const musicHubPath = ref("E://music/"); // 音乐存放目录
const currentMusic = ref("");
const currentIndex = ref(null);

interface CustomEventPayload {
  action: "play" | "pause" | "recovery";
  file_path?: string;
}

const prevSong = () => {
  // 上一首歌逻辑
};

const nextSong = () => {
  // 下一首歌逻辑
};

const handleMouseEnter = (index: number) => {
  // 鼠标移入事件
  currentIndex.value = index;
  console.log(index);
};

const handleMouseLeave = () => {
  currentIndex.value = null;
};

async function playAudio(file_name: String) {
  isPlaying.value = true;
  currentMusic.value = file_name;
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

async function recoveryAudio() {
  if (currentMusic.value === "") {
    return;
  }
  isPlaying.value = true;
  const event: CustomEventPayload = { action: "recovery" };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    console.error(error);
  }
}

const getFileList = () => {
  invoke("scan_files_in_directory", {
    path: musicHubPath.value,
  }).then((res: any) => {
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

.el-table tr:hover .hover-visible-button {
  display: inline-block; /* 或者其他您希望的显示样式，如 'block' */
}

/* 默认状态下，隐藏按钮 */
.hover-visible-button {
  display: none;
}

::v-deep .el-table--border th.el-table__cell,
::v-deep .el-table td.el-table__cell {
  border-bottom: none !important;
}
::v-deep .el-table--border .el-table__cell {
  border-right: none !important;
}
::v-deep .el-table--group,
.el-table--border {
  border: none !important;
}
</style>