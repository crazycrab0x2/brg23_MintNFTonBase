<template>
  <main>
    <WalletComponent />
  </main>
</template>
<script setup lang="ts">
import WalletComponent from "@/components/dashboard/WalletComponent.vue";

defineOptions({
  name: 'WalletView'
});
</script>