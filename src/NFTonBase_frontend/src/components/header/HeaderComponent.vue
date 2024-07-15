<template>
  <q-toolbar class="h-[60px]">
    <q-btn flat dense round icon="menu" @click="toggleLeftDrawer" />
    <!-- <q-Img :src=""></q-Img> -->
    <q-toolbar-title>
      <p class="m-0 text-2xl font-semibold">{{title}}</p>
    </q-toolbar-title>
    <div>

      <transition appear enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
        <div
          class="flex items-center bg-[#3f92e6] px-2 py-[2px] rounded-lg cursor-pointer duration-75 hover:px-3 hover:py-1"
          v-if="!isLogin" 
          @click="onLogin"
        >
          <InternetIdentityLogo class="w-[35px]" />
          <p class="m-0 ml-1 text-[16px] font-semibold">LogIn</p>
        </div>
      </transition>
      <p v-if="isLogin" class="mb-0 text-md font-semibold">{{ principal }}</p>

      <!-- <q-inner-loading :showing="isLoading" label-class="text-teal" label-style="font-size: 1.1em" /> -->
    </div>
  </q-toolbar>
</template>
<script setup lang="ts">
import InternetIdentityLogo from "./InternetIdentity.vue";
import { useAuthStore } from "@/stores/auth";
import { useLayoutStore } from "@/stores/layout";
import { storeToRefs } from "pinia";
import { AuthClient } from "@dfinity/auth-client";
import { Actor } from "@dfinity/agent"; 
import { ref , watch, onBeforeMount} from "vue";
import {useRoute} from "vue-router";

const route = useRoute();

const authStore = useAuthStore();
const layoutStore = useLayoutStore();

const { setIsLogin , setPrincipal} = authStore;
const { setDrawerOpen } = layoutStore;
const { isLogin , principal} = storeToRefs(authStore);
const isLoading = ref(false);
const title = ref("Mint NFT");

watch(()=> route.path, ()=> {
  if(route.path == '/') title.value = "Mint NFT"
  if(route.path == '/mint-nfts') title.value = "Mint NFTs"
  if(route.path == '/all-nfts') title.value = "All NFTs"
  if(route.path == '/wallet') title.value = "Wallet"
})

onBeforeMount(()=>{
  if(route.path == '/') title.value = "Mint NFT"
  if(route.path == '/mint-nfts') title.value = "Mint NFTs"
  if(route.path == '/all-nfts') title.value = "All NFTs"
  if(route.path == '/wallet') title.value = "Wallet"
})

const toggleLeftDrawer = () => {
  setDrawerOpen();
}

const onLogin = async () => {
  isLoading.value = true;
  let authClient = await AuthClient.create();
  const APP_NAME = "NFID example";
  const APP_LOGO = "https://nfid.one/icons/favicon-96x96.png";
  const CONFIG_QUERY = `?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`;

  const identityProvider = `https://nfid.one/authenticate${CONFIG_QUERY}`;

  authClient.login({
    identityProvider,
    onSuccess: async () => {
      const principal: string = authClient.getIdentity().getPrincipal().toText();
      // Actor.agentOf(quest_icp_backend)?.replaceIdentity(authClient.getIdentity());
      // let authentication = await tuneBook_backend.authentication(identity);
      // if (authentication.length == 0) {
      //   isRegisterModal.value = true;
      // }
      // else {
      //   setUserProfile(authentication[0]);

      // }
      setPrincipal(principal);
      setIsLogin(true);
      isLoading.value = false;
    },
    windowOpenerFeatures: `
      left=${window.screen.width / 2 - 525 / 2},
      top=${window.screen.height / 2 - 705 / 2},
      toolbar=0,location=0,menubar=0,width=525,height=705
      `,
  });
    // await new Promise((resolve) => {
    //   authClient.login({
    //     identityProvider: "https://identity.ic0.app",
    //     onSuccess: resolve,
    //   });
    // });

    // const identity = authClient.getIdentity();

    // Actor.agentOf(quest_icp_backend).replaceIdentity(identity);

    // setPrincipal(identity.getPrincipal().toText());
    // setIsLogin(true);
    // isLoading.value = false;
}
</script>