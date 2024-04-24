import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessage } from "element-plus";

const audioHubPath = ref("E://music/"); // Storage directory

export const isPlaying = ref(false);
export const tableData = ref([]);
export const currAudioName = ref("");
const currAudioId = ref(0);
export const isMuted = ref(false);
export const volume = ref(50); // Initial volume value
let originalVolume: number | null = null; // Store the original volume in a non silent state for use during recovery

interface CustomEventPayload {
    action: "play" | "pause" | "recovery" | "volume";
    file_path?: string;
    volume?: number;
}

export const getFileList = () => {
    invoke("scan_files_in_directory", {
        path: audioHubPath.value,
    }).then((res: any) => {
        tableData.value = res;
    });
};

export const playAudio = async (row: any) => {
    isPlaying.value = true;
    currAudioName.value = row.file_name;
    currAudioId.value = row.id;
    const file_path = audioHubPath.value + row.file_name;
    const event: CustomEventPayload = { action: "play", file_path: file_path };
    await invoke("handle_event", { event: JSON.stringify(event) }).catch((error) => ElMessage.error(error));

};

export const pauseAudio = async () => {
    isPlaying.value = false;
    const event: CustomEventPayload = { action: "pause" };
    await invoke("handle_event", { event: JSON.stringify(event) }).catch((error) => ElMessage.error(error));

};

export const recoveryAudio = async () => {
    if (currAudioName.value === "") {
        return;
    }
    isPlaying.value = true;
    const event: CustomEventPayload = { action: "recovery" };
    await invoke("handle_event", { event: JSON.stringify(event) }).catch((error) => ElMessage.error(error));

};

export const toggleMute = () => {
    if (!isMuted.value) {
        // Save volume values before muting
        originalVolume = volume.value;
    }
    isMuted.value = !isMuted.value;
    // Update volume
    volume.value = isMuted.value ? 0 : originalVolume ?? volume.value;
    changeVolume();
};

export const changeVolume = async () => {
    const event: CustomEventPayload = { action: "volume", volume: volume.value };
    await invoke("handle_event", { event: JSON.stringify(event) }).catch((error) => ElMessage.error(error));
};

export const preAudio = () => {
    if (currAudioId.value >= 2) {
        let row = tableData.value[currAudioId.value - 2];
        playAudio(row);
    }
};

export const nextAudio = async () => {
    let row = tableData.value[currAudioId.value];
    playAudio(row);
};

export const playControl = async () => {
    await invoke("is_sink_empty")
        .then((is_empty) => {
            if (is_empty) {
                isPlaying.value = false
                currAudioName.value = '';
                currAudioId.value = 0;
            }
        })
        .catch((error) => ElMessage.error(error))
};

