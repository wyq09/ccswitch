<template>
  <div class="app-container">
    <div class="content-wrapper">
      <!-- Sidebar with Glass Effect - Deep Dark -->
      <aside class="w-64 glass-effect-dark text-gray-100 flex flex-col">
        <!-- Logo -->
        <div class="p-6 pt-8 flex justify-center">
          <img
            :src="logoImage"
            alt="CCSwitch Logo"
            class="h-20 w-auto object-contain"
          />
        </div>

        <!-- Navigation -->
        <nav class="flex-1 px-4 space-y-1 py-4">
          <router-link
            to="/providers"
            class="flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-300 font-medium"
            :class="isProvidersRoute
              ? 'bg-orange-500 text-white shadow-md'
              : 'text-gray-300 hover:bg-white/10 hover:text-white'"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span>{{ t('nav.providers') }}</span>
          </router-link>
        </nav>

        <!-- Footer -->
        <div class="p-4 text-xs text-center border-t border-white/5">
          <a 
            href="https://github.com/wyq09/ccswitch" 
            target="_blank"
            rel="noopener noreferrer"
            class="text-gray-400 hover:text-orange-500 transition-colors duration-300 flex items-center justify-center gap-2 group"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
              <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
            </svg>
            <span class="group-hover:underline">@WYQ</span>
          </a>
        </div>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 overflow-auto">
        <router-view />
      </main>
    </div>

    <!-- Global Toast -->
    <Toast :visible="toastVisible" :message="toastMessage" :type="toastType" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import Toast from './components/Toast.vue'
import { useToast } from './composables/useToast'
import logoImage from '@/assets/logo.png'

const { t } = useI18n()
const route = useRoute()
const isProvidersRoute = computed(() => route.path.startsWith('/providers'))
const { visible: toastVisible, message: toastMessage, type: toastType } = useToast()
</script>

<style scoped>
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(to bottom right, #f9fafb, #f3f4f6);
}

.content-wrapper {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
