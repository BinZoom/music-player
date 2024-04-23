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

const musicHubPath = ref("E://music/"); // Storage directory
const isPlaying = ref(false);
const tableData = ref([]);
const currentMusic = ref("");

interface CustomEventPayload {
  action: "play" | "pause" | "recovery";
  file_path?: string;
}

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

const prevSong = () => {
  //TODO
};

const nextSong = () => {
  //TODO
};

onMounted(() => {
  getFileList();
});
</script>

<style scoped>
.music-body {
  flex: 1; /*  fill remaining space */
  overflow-y: auto;
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

/* list button dynamic display */
.el-table tr:hover .hover-visible-button {
  display: inline-block;
}
.hover-visible-button {
  display: none;
}

/* Hide table borders */
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