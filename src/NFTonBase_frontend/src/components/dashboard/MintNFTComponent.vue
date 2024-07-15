<template>
  <q-page v-if="isLogin">
    <div class="py-10">
      <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
        <div class="grid grid-cols-12 gap-4">
          <div class="col-span-4 col-start-3 flex flex-col">
            <q-file
              dense
              outlined
              v-model="model"
              placeholder="Select Image"
              class="my-3"
              @update:model-value="updatedFile"
              @clear="removeFile"
            >
              <template v-slot:prepend>
                <q-icon name="attach_file" />
              </template>
              <template v-slot:append>
                <q-icon name="close" @click="removeFile" class="cursor-pointer" />
              </template>
            </q-file>
            <q-img
              :src="avatarLink"
              fit="contain"
              spinner-color="white"
              class="w-full aspect-square border-[#1976D2] border-[1px] rounded-md"
            />
          </div>
          <div class="col-span-4 flex flex-col">
            <p class="text-md mt-5 mb-1 text-[#1976D2]">Input token supply</p>
            <q-input v-model="mintAmount" dense type="number" outlined class="mb-3" />
            <p class="text-md mt-5 mb-1 text-[#1976D2]">Input Owner account address</p>
            <q-input v-model="ownerAddress" dense outlined class="mb-3" />
            <q-btn label="Mint NFTs" no-caps color="primary" class="my-3" @click="mintNFTs" />
          </div>
        </div>
      </transition>
      <q-inner-loading
        :showing="visibleText"
        label="Minting NFTs..."
        label-class="text-teal"
        label-style="font-size: 1.1em"
      />
    </div>
  </q-page>
  <q-page v-else class="flex justify-center items-center">
    <p class="text-[#1976D2] text-2xl font-semibold">Login first...</p>
  </q-page>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { convertToDataURL } from '@/utils/image.js'
// import { ext_standard } from 'declarations/ext_standard'
import { useAuthStore } from '@/stores/auth'
import { storeToRefs } from 'pinia'
import { useQuasar } from 'quasar'

const $q = useQuasar()

const authStore = useAuthStore()
const { isLogin } = storeToRefs(authStore)
const model = ref(null)
const avatarLink = ref('')
const detectedText = ref('')
const detectedJsonData = ref('')
const mintAmount = ref(0)
const ownerAddress = ref('')
const avatarByteArray = ref()
const visibleText = ref(false)

const updatedFile = async (uploadFile: any) => {
  if (uploadFile) {
    const reader = new FileReader()

    reader.onload = async (e) => {
      const arrayBuffer = await uploadFile.arrayBuffer()
      const byteArray = new Uint8Array(arrayBuffer)
      avatarByteArray.value = byteArray
      const chunks = []
      chunks.push(new Uint8Array(byteArray))

      const blob = new Blob(chunks, { type: 'image/jpeg' })

      let new_url = await convertToDataURL(blob)
      avatarLink.value = new_url
    }

    reader.readAsArrayBuffer(uploadFile)
  }
}

const mintNFTs = async () => {
  visibleText.value = true
  // const tokenIdetifier = await ext_standard.register_token({
  //   owner: ownerAddress.value,
  //   metadata: avatarByteArray.value,
  //   supply: Number(mintAmount.value)
  // })
  // console.log("tokenIdetifier", tokenIdetifier)
  visibleText.value = false
  $q.notify({
    color: 'primary',
    textColor: 'white',
    icon: 'information',
    message: 'NFT minted successfully',
    position: 'bottom-right',
    timeout: 3000
  })
}

const removeFile = () => {
  detectedText.value = ''
  detectedJsonData.value = ''
  avatarLink.value = ''
  model.value = null
}
</script>
