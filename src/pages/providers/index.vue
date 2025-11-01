<template>
  <div class="h-full flex flex-col" style="background-color: #273B3E;">
    <!-- 顶部标题栏 -->
    <header class="bg-white shadow-sm px-6 py-4 flex justify-between items-center">
      <h2 class="text-xl font-semibold text-gray-800">{{ t('providers.title') }}</h2>
      <div class="flex space-x-2">
        <LanguageSwitcher />
        <button 
          @click="handleAdd"
          class="px-4 py-2 text-white rounded-md transition-colors flex items-center"
          style="background-color: #F87102;"
          @mouseenter="handleButtonHover(true, $event)"
          @mouseleave="handleButtonHover(false, $event)"
        >
          <i class="fas fa-plus mr-2"></i>
          {{ t('providers.addProvider') }}
        </button>
        <button 
          v-if="providers.length > 0"
          @click="showClearDialog = true"
          class="px-4 py-2 border border-red-300 text-red-600 rounded-md hover:bg-red-50 transition-colors"
          :title="t('providers.clearAll')"
        >
          <i class="fas fa-trash-alt"></i>
        </button>
        <button 
          @click="handleQuit"
          class="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
          :title="t('common.quit')"
        >
          <i class="fas fa-power-off"></i>
        </button>
      </div>
    </header>

    <!-- 供应商列表 -->
    <main class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="flex items-center justify-center h-64">
        <div class="text-gray-500">{{ t('common.loading') }}</div>
      </div>

      <div v-else-if="error" class="flex items-center justify-center h-64">
        <div class="text-red-500">{{ error }}</div>
      </div>

      <div v-else-if="providers.length === 0" class="flex flex-col items-center justify-center h-64">
        <div class="text-gray-400 mb-4">
          <i class="fas fa-box-open text-6xl"></i>
        </div>
        <p class="text-gray-500 mb-4">{{ t('providers.noProviders') }}</p>
        <button
          @click="handleAdd"
          class="px-6 py-2 text-white rounded-lg transition-colors"
          style="background-color: #F87102;"
          @mouseenter="handleButtonHover(true, $event)"
          @mouseleave="handleButtonHover(false, $event)"
        >
          {{ t('providers.addFirstProvider') }}
        </button>
      </div>

      <div v-else class="space-y-4">
        <div 
          v-for="provider in providers" 
          :key="provider.id"
          @click="handleSwitch(provider.id)"
          class="bg-white rounded-2xl shadow-lg cursor-pointer transition-all duration-300 hover:shadow-xl"
          :style="provider.isActive ? {
            backgroundColor: '#F87102',
            boxShadow: '0 10px 25px -5px rgba(248, 113, 2, 0.4), 0 8px 10px -6px rgba(248, 113, 2, 0.3)'
          } : {
            boxShadow: '0 4px 15px -3px rgba(0, 0, 0, 0.1), 0 2px 6px -2px rgba(0, 0, 0, 0.05)'
          }"
        >
          <div class="p-6 flex items-center justify-between">
            <div class="flex items-center">
              <div 
                class="w-16 h-16 rounded-2xl flex items-center justify-center mr-4 overflow-hidden p-2"
                :style="provider.isActive ? {
                  backgroundColor: 'rgba(255, 255, 255, 0.2)'
                } : {
                  backgroundColor: 'rgba(248, 113, 2, 0.1)'
                }"
              >
                <img 
                  v-if="provider.icon && provider.icon.startsWith('/')" 
                  :src="provider.icon" 
                  :alt="provider.name"
                  class="w-full h-full object-contain"
                />
                <span v-else class="text-3xl">{{ provider.icon || '🔧' }}</span>
              </div>
              <div>
                <h3 
                  class="font-semibold text-lg"
                  :class="provider.isActive ? 'text-white' : 'text-gray-900'"
                >
                  {{ provider.name }}
                </h3>
                <p 
                  class="text-sm mt-1"
                  :class="provider.isActive ? 'text-white/80' : 'text-gray-500'"
                >
                  {{ provider.baseUrl }}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-3">
              <span v-if="provider.isActive" class="text-white">
                <i class="fas fa-check-circle text-2xl"></i>
              </span>
              <button 
                @click.stop="handleEdit(provider.id)"
                class="w-10 h-10 rounded-full flex items-center justify-center transition-colors"
                :class="provider.isActive ? 'text-white/80 hover:bg-white/20' : 'text-gray-400 hover:bg-gray-100'"
                :title="t('common.edit')"
              >
                <i class="fas fa-ellipsis-v"></i>
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 清空配置确认对话框 -->
    <div 
      v-if="showClearDialog"
      class="fixed inset-0 bg-black/30 backdrop-blur-lg flex items-center justify-center z-50"
      @click.self="showClearDialog = false"
    >
      <div class="glass-effect-lg rounded-2xl max-w-md w-full mx-4 overflow-hidden shadow-2xl">
        <div class="p-6">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-12 h-12 rounded-full bg-red-100/80 flex items-center justify-center flex-shrink-0">
              <i class="fas fa-exclamation-triangle text-red-700 text-xl"></i>
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900">{{ t('providers.clearAllTitle') }}</h3>
              <p class="text-sm text-gray-600 mt-1">{{ t('providers.clearAllWarning') }}</p>
            </div>
          </div>
          
          <div class="bg-red-50/60 border border-red-200/60 rounded-lg p-4 mb-4">
            <p class="text-sm text-red-900 font-medium">
              <i class="fas fa-info-circle mr-2"></i>
              {{ t('providers.clearAllDescription') }}
            </p>
          </div>

          <div class="flex gap-3">
            <button
              @click="showClearDialog = false"
              class="flex-1 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50/50 transition-all duration-300 text-gray-700 font-medium"
            >
              {{ t('common.cancel') }}
            </button>
            <button
              @click="handleClearAll"
              class="flex-1 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg hover:shadow-lg transition-all duration-300 font-medium"
            >
              {{ t('providers.confirmClear') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useProviderStore } from '../../stores/provider'
import { storeToRefs } from 'pinia'
import { exit } from '@tauri-apps/plugin-process'
import { useToast } from '../../composables/useToast'
import LanguageSwitcher from '../../components/LanguageSwitcher.vue'

const { t } = useI18n()
const router = useRouter()
const providerStore = useProviderStore()
const { providers, loading, error } = storeToRefs(providerStore)
const toast = useToast()
const showClearDialog = ref(false)

onMounted(async () => {
  await providerStore.loadProviders()
})

const handleAdd = () => {
  router.push('/providers/add')
}

const handleEdit = (id: string) => {
  router.push(`/providers/${id}/edit`)
}

const handleSwitch = async (id: string) => {
  try {
    await providerStore.switchProvider(id)
    toast.success(t('providers.switchSuccess'))
  } catch (e) {
    toast.error(t('providers.switchError') + '：' + e)
  }
}

const handleClearAll = async () => {
  try {
    await providerStore.clearAllProviders()
    showClearDialog.value = false
    toast.success(t('providers.clearSuccess'))
  } catch (e) {
    toast.error(t('providers.clearError') + '：' + e)
  }
}

const handleQuit = async () => {
  await exit(0)
}

const handleButtonHover = (isEntering: boolean, event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (target) {
    target.style.backgroundColor = isEntering ? '#E86600' : '#F87102'
  }
}
</script>

