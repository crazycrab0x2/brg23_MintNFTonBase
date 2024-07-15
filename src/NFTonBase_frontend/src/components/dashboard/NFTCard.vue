<template>
  <q-card class="my-card">
    <q-img :src="imageLink" class="aspect-square" fit="contain"> </q-img>
    <q-card-actions class="flex flex-col items-center mx-10">
      <div class="m-0 text-primary text-md">Token Identifier: {{ token }}</div>
      <div class="m-0 text-primary text-md my-1">
        {{ type == 'All' ? 'Totoal Supply:' : 'Amount:' }} {{ balance }}
      </div>
      <q-btn v-if="type == 'All'" color="primary" @click="onViewDetails" class="w-full"
        >Details</q-btn
      >
      <q-btn v-else color="primary" @click="onTransferModalView" class="w-full">transfer</q-btn>
    </q-card-actions>
  </q-card>
  <q-dialog v-model="showDetail">
    <q-card>
      <q-card-section>
        <div class="text-h6">Token Identifier: {{ token }}</div>
        <div class="text-h6">Total Supply: {{ balance }}</div>
      </q-card-section>

      <q-card-section class="q-pt-none">
        <div class="" v-for="holder in tokenHolders">
          <p class="text-primary text-bold">{{ holder.amount }} : {{ holder.holder }}</p>
        </div>
      </q-card-section>
    </q-card>
  </q-dialog>
  <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
    <q-dialog v-model="transferModalView" persistent>
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6">Transfer NFTs</div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          <p class="text-md my-1 text-[#1976D2]">Input receiver address</p>
          <q-input v-model="receiverAddress" dense outlined class="mb-1" />
          <p class="text-md my-1 text-[#1976D2]">Input amount</p>
          <q-input v-model="amount" dense outlined type="number" class="mb-3" />
        </q-card-section>

        <q-card-actions align="right" class="text-primary">
          <q-btn flat label="Cancel" no-caps v-close-popup />
          <q-btn flat label="Transfer" no-caps v-close-popup @click="onTransfer" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </transition>
  <q-inner-loading
    :showing="visibleText"
    label="Transferring NFTs..."
    label-class="text-teal"
    label-style="font-size: 1.1em"
  />
</template>
<script setup lang="ts">
const props = defineProps(['token', 'balance', 'metadata', 'type', 'from'])
import { ref, onMounted } from 'vue'
import { convertToDataURL } from '@/utils/image.js'
// import { ext_standard } from 'declarations/ext_standard'
import { useQuasar } from 'quasar'

const $q = useQuasar()
const showDetail = ref(false)
const transferModalView = ref(false)
const imageLink = ref('https://cdn.quasar.dev/img/parallax2.jpg')
const tokenHolders = ref<TokenHolder[]>([])
const amount = ref()
const receiverAddress = ref('')
const visibleText = ref(false)

interface TokenHolder {
  holder: string
  amount: number
}

onMounted(async () => {
  const byteArray = new Uint8Array(props.metadata)
  const chunks = []
  chunks.push(new Uint8Array(byteArray))

  const blob = new Blob(chunks, { type: 'image/jpeg' })
  imageLink.value = await convertToDataURL(blob)
})

const onViewDetails = async () => {
  // tokenHolders.value = await ext_standard.number_of_token_holders(props.token)
  showDetail.value = true
}
const onTransferModalView = () => {
  transferModalView.value = true
}
const onTransfer = async () => {
  visibleText.value = true
  // const restBalane = await ext_standard.transfer({
  //   to: receiverAddress.value,
  //   from: props.from,
  //   amount: Number(amount.value),
  //   token: props.token
  // })
  // console.log(restBalane)
  visibleText.value = false
  $q.notify({
    color: 'primary',
    textColor: 'white',
    icon: 'information',
    message: 'NFT transferred successfully',
    position: 'bottom-right',
    timeout: 3000
  })
}
</script>
