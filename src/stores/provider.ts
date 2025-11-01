import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Provider, Template } from '../types'
import { BUILTIN_TEMPLATES } from '../config/templates'

export const useProviderStore = defineStore('provider', () => {
  const providers = ref<Provider[]>([])
  const customTemplates = ref<Template[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const activeProvider = computed(() => 
    providers.value.find(p => p.isActive)
  )

  const allTemplates = computed(() => [
    ...BUILTIN_TEMPLATES,
    ...customTemplates.value,
  ])

  async function loadProviders() {
    try {
      loading.value = true
      error.value = null
      const data = await invoke<Provider[]>('load_providers')
      providers.value = data
    } catch (e) {
      error.value = e as string
      console.error('Failed to load providers:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveProvider(provider: Provider) {
    try {
      loading.value = true
      error.value = null
      await invoke('save_provider', { provider })
      await loadProviders()
    } catch (e) {
      error.value = e as string
      console.error('Failed to save provider:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteProvider(id: string) {
    try {
      loading.value = true
      error.value = null
      await invoke('delete_provider', { id })
      await loadProviders()
    } catch (e) {
      error.value = e as string
      console.error('Failed to delete provider:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function switchProvider(id: string) {
    try {
      loading.value = true
      error.value = null
      await invoke('switch_provider', { id })
      await loadProviders()
    } catch (e) {
      error.value = e as string
      console.error('Failed to switch provider:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function loadTemplates() {
    try {
      const data = await invoke<Template[]>('load_templates')
      customTemplates.value = data
    } catch (e) {
      console.error('Failed to load templates:', e)
    }
  }

  async function saveTemplate(template: Template) {
    try {
      await invoke('save_template', { template })
      await loadTemplates()
    } catch (e) {
      console.error('Failed to save template:', e)
      throw e
    }
  }

  async function deleteTemplate(id: string) {
    try {
      await invoke('delete_template', { id })
      await loadTemplates()
    } catch (e) {
      console.error('Failed to delete template:', e)
      throw e
    }
  }

  async function clearAllProviders() {
    try {
      loading.value = true
      error.value = null
      await invoke('clear_all_providers')
      await loadProviders()
    } catch (e) {
      error.value = e as string
      console.error('Failed to clear all providers:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    providers,
    customTemplates,
    allTemplates,
    loading,
    error,
    activeProvider,
    loadProviders,
    saveProvider,
    deleteProvider,
    switchProvider,
    clearAllProviders,
    loadTemplates,
    saveTemplate,
    deleteTemplate,
  }
})

