import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useAuthStore = defineStore({
  id: 'auth',
  state: () => ({
    isLogin: "",
    principal: ""
  }),
  actions: {
      setIsLogin(value) {
          this.isLogin = value 
      },
      setPrincipal(value) {
          this.principal = value 
      }
  },
  persist: {
      enabled: true,
      strategies: [
          { 
              key: 'my_store',
              storage: localStorage
          }
      ]
  }
})