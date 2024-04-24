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
    <el-row :gutter="20">
      <el-col :span="6">
        <div style="margin-top: 10px">
          <el-text size="small">{{ currentMusic }}</el-text>
        </div>
      </el-col>
      <el-col :span="2"></el-col>
      <el-col :span="8">
        <el-button link type="primary" class="no-shadow" @click="prevSong"
          ><el-icon><ArrowLeftBold /></el-icon
        ></el-button>
        <el-button
          link
          type="primary"
          class="no-shadow"
          v-if="!isPlaying"
          @click="recoveryAudio()"
          ><el-icon size="35px"><VideoPlay /></el-icon
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
      </el-col>
      <el-col :span="4"></el-col>
      <el-col :span="4">
        <div style="margin-top: 10px">
          <el-popover placement="top" trigger="hover">
            <template #reference>
              <el-button
                link
                type="primary"
                class="no-shadow"
                @click="toggleMute"
                ><el-icon v-if="!isMuted"><Bell /></el-icon>
                <el-icon v-else><MuteNotification /></el-icon>
              </el-button>
            </template>
            <div class="slider-block">
              <el-slider
                v-model="volume"
                :min="0"
                :max="100"
                :step="2"
                size="small"
                height="140px"
                @mouseup="changeVolume"
                @touchend="changeVolume"
              />
            </div>
          </el-popover>
        </div>
      </el-col>
    </el-row>
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
  VideoPlay,
  Bell,
  MuteNotification,
} from "@element-plus/icons-vue";

const musicHubPath = ref("E://music/"); // Storage directory
const isPlaying = ref(false);
const tableData = ref([]);
const currentMusic = ref("");
const isMuted = ref(false);
const volume = ref(50); // Initial volume value
let originalVolume: number | null = null; // Store the original volume in a non silent state for use during recovery

interface CustomEventPayload {
  action: "play" | "pause" | "recovery" | "volume" | "next";
  file_path?: string;
  volume?: number;
}

const playAudio = async (file_name: String) => {
  isPlaying.value = true;
  currentMusic.value = file_name;
  const file_path = musicHubPath.value + file_name;
  const event: CustomEventPayload = { action: "play", file_path: file_path };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    ElMessage.error(error);
  }
};

const pauseAudio = async () => {
  isPlaying.value = false;
  const event: CustomEventPayload = { action: "pause" };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    ElMessage.error(error);
  }
};

const recoveryAudio = async () => {
  if (currentMusic.value === "") {
    return;
  }
  isPlaying.value = true;
  const event: CustomEventPayload = { action: "recovery" };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    ElMessage.error(error);
  }
};

const toggleMute = () => {
  if (!isMuted.value) {
    // Save volume values before muting
    originalVolume = volume.value;
  }
  isMuted.value = !isMuted.value;
  // Update volume
  volume.value = isMuted.value ? 0 : originalVolume ?? volume.value;
  // 调用后台接口
  changeVolume();
};

const changeVolume = async () => {
  const event: CustomEventPayload = { action: "volume", volume: volume.value };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    ElMessage.error(error);
  }
};

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

const nextSong = async () => {
  const event: CustomEventPayload = { action: "next" };
  try {
    await invoke("handle_event", { event: JSON.stringify(event) });
  } catch (error) {
    ElMessage.error(error);
  }
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

.slider-block {
  max-width: 150px;
  display: flex;
  align-items: center;
}
.slider-block .el-slider {
  margin-top: 0;
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