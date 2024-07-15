<template>
  <q-page v-if="isLogin">
    <div class="px-10 py-10">
      <div class="grid grid-cols-12 gap-3">
        <div class="col-span-4 flex flex-col overflow-x-hidden relative">
          <q-btn color="primary" label="Add Account" no-caps @click="onAddAccount"></q-btn>
          <div
            v-for="account in accountList"
            @click="onSelectAccount(account)"
            :class="selectedAccount == account && 'border-[#1976D2] border-[2px]'"
            class="w-full border-blue border-[1px] rounded-md my-2 py-1 px-3 flex flex-col items-around cursor-pointer hover:border-[#1976D2] overflow-hidden"
          >
            <p class="m-0" :class="selectedAccount == account && 'text-primary text-bold'">
              {{ account.name }}
            </p>
            <p
              class="m-0 overflow-hidden w-full"
              :class="selectedAccount == account && 'text-primary text-bold'"
            >
              {{ account.address }}
            </p>
          </div>
        </div>
        <div class="col-span-8">
          <div class="px-5">
            <p class="text-primary text-bold text-xl m-0">{{ selectedAccount?.name }}</p>
            <p class="text-primary text-bold text-[18px]">{{ selectedAccount?.address }}</p>
          </div>
          <div class="grid grid-cols-12 gap-3" v-if="nftList.length > 0">
            <div class="xl:col-span-4 lg:col-span-6 col-span-12" v-for="nft in nftList">
              <NftCard v-bind="{...nft, type: 'User', from: selectedAccount?.address}" />
            </div>
            </div>
            <div v-else>
              <p class="text-center text-primary text-lg text-bold mt-5">No NFTs...</p>
            </div>
        </div>
      </div>
    </div>
  </q-page>
  <q-page v-else class="flex justify-center items-center">
    <p class="text-[#1976D2] text-2xl font-semibold">Login first...</p>
  </q-page>
</template>
<script setup lang="ts">
import { ref, onBeforeMount, watch } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { storeToRefs } from 'pinia'
// import { ext_standard } from 'declarations/ext_standard'
import NftCard from "@/components/dashboard/NFTCard.vue";

const authStore = useAuthStore()
const { isLogin, principal } = storeToRefs(authStore)

const accountList = ref<Account[]>([])
const selectedAccount = ref<Account>()
const nftList = ref<NFT[]>([]);

interface Account {
  name: string
  address: string
}

interface NFT {
  token: number
  balance: number
  metadata: Uint8Array | number[]
}

// onBeforeMount(async () => {
//   accountList.value.push({
//     name: 'Primary',
//     address: await ext_standard.get_account(
//       principal.value,
//       numberToU8Array32(0)
//     )
//   })
//   selectedAccount.value = accountList.value[0]
// })

const numberToU8Array32 = (number: number) => {
  let result = new Array(32).fill(0)
  for (let i = 31; number > 0 && i >= 0; i--) {
    result[i] = number & 0xff
    number >>= 8
  }
  return result
}

const onAddAccount = async () => {
  // accountList.value.push({
  //   name: `Account ${accountList.value.length}`,
  //   address: await ext_standard.get_account(
  //     principal.value,
  //     numberToU8Array32(accountList.value.length)
  //   )
  // })
}

// watch(()=>selectedAccount.value, async () => {
//   nftList.value = await ext_standard.get_user_nfts({owner: selectedAccount.value ? selectedAccount.value.address : ''})
//   console.log(nftList.value)
// })

const onSelectAccount = (account: Account) => {
  selectedAccount.value = account
}

</script>
