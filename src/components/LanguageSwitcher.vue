<template>
  <div class="relative" ref="dropdownRef">
    <button
      @click="toggleDropdown"
      class="px-3 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors flex items-center gap-2"
      :title="currentLocale === 'zh-CN' ? '切换语言' : 'Switch Language'"
    >
      <i class="fas fa-globe"></i>
      <span class="text-sm">{{ currentLocale === 'zh-CN' ? '中文' : 'EN' }}</span>
      <i class="fas fa-chevron-down text-xs"></i>
    </button>

    <!-- Dropdown Menu -->
    <div
      v-if="showDropdown"
      class="absolute right-0 mt-2 w-40 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50"
    >
      <button
        @click="changeLocale('zh-CN')"
        class="w-full px-4 py-2 text-left hover:bg-gray-50 transition-colors flex items-center gap-2"
        :class="currentLocale === 'zh-CN' ? 'text-orange-600 font-medium' : 'text-gray-700'"
      >
        <i class="fas fa-check" :class="currentLocale === 'zh-CN' ? 'opacity-100' : 'opacity-0'"></i>
        <span>简体中文</span>
      </button>
      <button
        @click="changeLocale('en-US')"
        class="w-full px-4 py-2 text-left hover:bg-gray-50 transition-colors flex items-center gap-2"
        :class="currentLocale === 'en-US' ? 'text-orange-600 font-medium' : 'text-gray-700'"
      >
        <i class="fas fa-check" :class="currentLocale === 'en-US' ? 'opacity-100' : 'opacity-0'"></i>
        <span>English</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { setLocale } from '../plugins/i18n'

const { locale } = useI18n()
const showDropdown = ref(false)
const dropdownRef = ref<HTMLElement>()

const currentLocale = computed(() => locale.value)

const toggleDropdown = () => {
  showDropdown.value = !showDropdown.value
}

const changeLocale = (newLocale: string) => {
  setLocale(newLocale)
  showDropdown.value = false
}

// 点击外部关闭下拉菜单
const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    showDropdown.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

