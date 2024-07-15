<template>
  <q-page v-if="isLogin">
    <div class="px-10 py-10">
      <div class="grid grid-cols-12 gap-3">
        <div class="xl:col-span-3 lg:col-span-4 col-span-6" v-for="nft in nftList" v-if="nftList.length > 0">
          <NftCard v-bind="{...nft, type:'All', from: ''}" />
        </div>
        <div v-else class="col-span-12">
          <p class="text-center text-primary text-xl text-bold">No NFTs...</p>
        </div>
      </div>
    </div>
  </q-page>
  <q-page v-else class="flex justify-center items-center">
      <p class="text-[#1976D2] text-2xl font-semibold">Login first...</p>
    </q-page>
</template>
<script setup lang="ts">
import { ref, onBeforeMount } from 'vue'
// import { ext_standard } from 'declarations/ext_standard'
import { useAuthStore } from '@/stores/auth'
import { storeToRefs } from 'pinia'
import { useQuasar } from 'quasar';
import NftCard from "@/components/dashboard/NFTCard.vue";

const $q = useQuasar()

interface NFT {
  token: number
  balance: number
  metadata: Uint8Array | number[]
}

const authStore = useAuthStore()
const { isLogin } = storeToRefs(authStore)
const nftList = ref<NFT[]>([])

onBeforeMount(async () => {
  // nftList.value = await ext_standard.get_all_nfts()
  console.log(typeof(nftList.value[0].metadata))
  console.log(nftList.value[0].metadata)
  
})
</script>
