<template>
  <div class="music-body">
    <el-table
      :data="tableData"
      style="width: 100%"
      size="small"
      :show-header="false"
    >
      <el-table-column prop="id" width="40px" />
      <el-table-column prop="file_name" />
      <el-table-column width="100">
        <template #default="{ row }">
          <el-button
            link
            type="primary"
            class="hover-visible-button no-shadow"
            @click="playAudio(row)"
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
          <el-text size="small">
            <el-icon><Headset /></el-icon> {{ currAudioName }}</el-text
          >
        </div>
      </el-col>
      <el-col :span="2"></el-col>
      <el-col :span="8">
        <el-button link type="primary" class="no-shadow" @click="preAudio"
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
        <el-button link type="primary" class="no-shadow" @click="nextAudio"
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
          <el-popover placement="top" trigger="hover">
            <template #reference>
              <el-button link type="primary" class="no-shadow">
                <span v-if="radio === 'L'">L</span>
                <span v-else>S</span>
              </el-button>
            </template>
            <div>
              <el-radio-group v-model="radio">
                <el-radio size="small" label="List loop" value="L" />
                <el-radio size="small" label="Single loop" value="S" />
              </el-radio-group>
            </div>
          </el-popover>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import * as Music from "./Music";
import { ref, onMounted, onUnmounted } from "vue";

import {
  CaretRight,
  ArrowLeftBold,
  ArrowRightBold,
  VideoPause,
  VideoPlay,
  Bell,
  MuteNotification,
  Headset,
} from "@element-plus/icons-vue";

const tableData = Music.tableData;
const isPlaying = Music.isPlaying;
const currAudioName = Music.currAudioName;
const isMuted = Music.isMuted;
const volume = Music.volume;
const radio = Music.radio;

const playAudio = Music.playAudio;
const pauseAudio = Music.pauseAudio;
const recoveryAudio = Music.recoveryAudio;
const toggleMute = Music.toggleMute;
const changeVolume = Music.changeVolume;
const preAudio = Music.preAudio;
const nextAudio = Music.nextAudio;

onMounted(() => {
  Music.getFileList();

  const intervalId = setInterval(Music.playControl, 1000);

  // clear
  onUnmounted(() => {
    clearInterval(intervalId);
  });
});
</script>

<style scoped>
@import "./Music.css";
</style>