<template>
    <div class="px-20 py-10" v-if="isLogin">
        <div class="grid grid-cols-12 gap-4">
            <div class="col-span-4 flex flex-col">
                <q-file dense outlined v-model="model" @update:model-value="updatedFile" @clear="removeFile">
                    <template v-slot:prepend>
                        <q-icon name="attach_file" />
                    </template>
                    <template v-slot:append>
                        <q-icon name="close" @click="removeFile" class="cursor-pointer" />
                    </template>
                </q-file>
                <q-img :src="avatarLink" fit="contain" spinner-color="white"
                    class="w-full aspect-square my-3 border-[#1976D2] border-[2px] rounded-md" />
            </div>
            <div class="col-span-4">
                <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
                    <q-input v-model="detectedJsonData" outlined readonly autogrow
                        placeholder="Detected Receipt Data will appear here." class="bg-transparent" />
                </transition>
                <q-inner-loading :showing="visible" label="Detecting receipt data..." label-class="text-teal"
                    label-style="font-size: 1.1em" />
            </div>
            <div class="col-span-4">
                <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
                    <div>
                        <q-btn label="Upload Image" color="primary" no-caps class="!w-full mb-3" @click="onUploadImage"/>
                        <q-input v-model="uploadedImageURL" placeholder="Uploaded Image URL" outlined readonly autogrow/>
                        <q-btn label="Upload Receipt Data" color="primary" no-caps class="!w-full my-3" />
                        <q-input v-model="uploadedImageURL" placeholder="Uploaded Receipt URL" outlined readonly autogrow/>
                    </div>
                </transition>
                <q-inner-loading :showing="visible" label="Detecting text from image..." label-class="text-teal"
                    label-style="font-size: 1.1em" />
            </div>
        </div>
    </div>

  <q-page v-else class="flex justify-center items-center">
    <p class="text-[#1976D2] text-2xl font-semibold">Login first...</p>
  </q-page>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { convertToDataURL } from "@/utils/image.js";
import { recognizeText } from "@/utils/recognizeText";
import { resizeAndGrayscaleImage } from "@/utils/resizeAndGrayscaleImage"
import {useAuthStore} from "@/stores/auth";
import { storeToRefs } from "pinia";
import { NFTonBase_backend } from "declarations/NFTonBase_backend/index"
import { nanoid } from "nanoid";

const authStore = useAuthStore();
const {isLogin} = storeToRefs(authStore);


const model = ref(null);
const avatarLink = ref("");
const detectedText = ref("");
const detectedJsonData = ref("");
const visible = ref(false);
const uploadedImageURL = ref('')
const imageByteArray = ref();


const updatedFile = async (uploadFile: any) => {
    if (uploadFile) {

        const grayImage = await resizeAndGrayscaleImage(uploadFile);
        const reader = new FileReader();

        reader.onload = async (e) => {
            const arrayBuffer = await grayImage.arrayBuffer();
            const byteArray = new Uint8Array(arrayBuffer);
            imageByteArray.value = byteArray;
            console.log(imageByteArray.value)
            const chunks = [];
            chunks.push(new Uint8Array(byteArray));

            const blob = new Blob(chunks, { type: "image/jpeg" });

            let new_url = await convertToDataURL(blob);
            avatarLink.value = new_url;
        };

        reader.readAsArrayBuffer(grayImage);
        // visible.value = true;
        // detectedText.value = await recognizeText(uploadFile)
        // const detectedData= await tesseract_backend.get_gpt_data("convert this receipt data as json format\n" + detectedText.value);
        // detectedJsonData.value = JSON.parse(detectedData).choices[0].text; 
        // console.log(JSON.parse(detectedJsonData.value))
        visible.value = false;
    }
}

const removeFile = () => {
    detectedText.value = "";
    detectedJsonData.value = "";
    avatarLink.value = "";
    model.value = null;
}

const onUploadImage = async () => {
    const docKey = nanoid();
    uploadedImageURL.value = await NFTonBase_backend.upload_image(docKey, imageByteArray.value) as string;
}

</script>