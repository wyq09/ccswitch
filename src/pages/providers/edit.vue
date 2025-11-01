<template>
  <div class="h-full flex items-center justify-center bg-black/30 backdrop-blur-lg">
    <!-- Modal Card with Glass Effect -->
    <div class="glass-effect-lg rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col shadow-2xl">
      <!-- Header -->
      <div class="flex items-center justify-between px-8 py-6 border-b border-gray-200/50 bg-gradient-to-r from-white/50 to-white/30">
        <div class="flex items-center gap-4">
          <button
            @click="handleBack"
            class="p-2 hover:bg-gray-200/30 rounded-lg transition-all duration-300"
          >
            <svg class="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h1 class="text-xl font-semibold text-gray-900">编辑 API 服务商</h1>
        </div>
        <div class="flex gap-2">
          <button
            @click="handleSave"
            :disabled="!isValid || saving"
            class="px-6 py-2.5 bg-orange-500 hover:bg-orange-600 text-white rounded-xl font-medium transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ saving ? '保存中...' : '保存' }}
          </button>
          <button
            @click="showDeleteDialog = true"
            class="px-4 py-2.5 bg-red-600 hover:bg-red-700 text-white rounded-xl transition-all duration-300"
            title="删除此供应商"
          >
            <i class="fas fa-trash-alt"></i>
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-auto">
        <div v-if="loading" class="flex items-center justify-center h-64">
          <div class="text-gray-500">加载中...</div>
        </div>

        <div v-else-if="!provider" class="flex items-center justify-center h-64">
          <div class="text-red-500">供应商不存在</div>
        </div>

        <div v-else>
          <!-- Active Banner -->
          <div v-if="provider.isActive" class="bg-primary text-white px-8 py-4 flex items-center gap-3">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
            </svg>
            <span class="font-medium">当前激活的供应商</span>
          </div>

          <!-- Form -->
          <div class="p-8 space-y-5">
          <!-- Provider Name -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              供应商名称
            </label>
            <input
              v-model="form.name"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- API Endpoint -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
              </svg>
              API 端点
            </label>
            <input
              v-model="form.baseUrl"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Tags -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
              </svg>
              标签
            </label>
            <div class="flex gap-2">
              <input
                v-model="tagInput"
                @keyup.enter="addTag"
                type="text"
                placeholder="输入标签后按回车添加"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              />
              <button
                @click="addTag"
                type="button"
                class="px-4 py-2 bg-primary hover:bg-orange-600 text-white rounded-lg transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
              </button>
            </div>
            <div v-if="form.tags.length > 0" class="mt-3 flex flex-wrap gap-2">
              <span
                v-for="(tag, index) in form.tags"
                :key="index"
                class="inline-flex items-center gap-1 px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded-md"
              >
                {{ tag }}
                <button
                  @click="removeTag(index)"
                  type="button"
                  class="hover:text-red-600"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </span>
            </div>
          </div>

          <!-- API Key -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
              </svg>
              API 密钥
              <span class="text-red-500">*</span>
            </label>
            <div class="relative">
              <input
                v-model="form.apiKey"
                :type="showApiKey ? 'text' : 'password'"
                class="w-full px-4 py-2 pr-12 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              />
              <button
                @click="showApiKey = !showApiKey"
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
              >
                <svg v-if="showApiKey" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
                <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Opus Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <span class="w-6 h-6 flex items-center justify-center bg-gray-100 rounded text-xs font-semibold">O</span>
              Opus 模型
            </label>
            <input
              v-model="form.models.opus"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Sonnet Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <span class="w-6 h-6 flex items-center justify-center bg-gray-100 rounded text-xs font-semibold">S</span>
              Sonnet 模型
            </label>
            <input
              v-model="form.models.sonnet"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Haiku Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <span class="w-6 h-6 flex items-center justify-center bg-gray-100 rounded text-xs font-semibold">H</span>
              Haiku 模型
            </label>
            <input
              v-model="form.models.haiku"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Anthropic Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <span class="w-6 h-6 flex items-center justify-center bg-gray-100 rounded text-xs font-semibold">A</span>
              Anthropic 模型
            </label>
            <input
              v-model="form.anthropicModel"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Anthropic Small Fast Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <span class="w-6 h-6 flex items-center justify-center bg-gray-100 rounded text-xs font-semibold">ASF</span>
              Anthropic Small Fast 模型
            </label>
            <input
              v-model="form.anthropicSmallFastModel"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Custom Environment Variables -->
          <div>
            <div class="flex items-center justify-between mb-3">
              <label class="flex items-center gap-2 text-sm font-medium text-gray-700">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                自定义环境变量
              </label>
              <span class="text-xs text-gray-500">可选</span>
            </div>
            
            <!-- 新增环境变量输入 -->
            <div class="flex gap-2 mb-3">
              <input
                v-model="envName"
                type="text"
                placeholder="环境变量名"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
              />
              <input
                v-model="envValue"
                type="text"
                placeholder="值"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
              />
              <button
                @click="addCustomEnv"
                type="button"
                class="px-4 py-2 bg-primary hover:bg-orange-600 text-white rounded-lg transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
              </button>
            </div>
            
            <div class="space-y-3 mb-3">
              <div v-for="(value, key, index) in form.customEnv" :key="index" class="flex gap-2 items-end">
                <input
                  :value="key"
                  @input="updateCustomEnvKey(index, ($event.target as HTMLInputElement).value)"
                  type="text"
                  placeholder="环境变量名"
                  class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
                />
                <input
                  :value="value"
                  @input="updateCustomEnvValue(index, ($event.target as HTMLInputElement).value)"
                  type="text"
                  placeholder="值"
                  class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
                />
                <button
                  @click="removeCustomEnv(index)"
                  type="button"
                  class="px-3 py-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
            
            <div v-if="Object.keys(form.customEnv).length === 0" class="text-sm text-gray-400 text-center py-4">
              暂无环境变量，点击上方按钮添加
            </div>
          </div>
        </div>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div
      v-if="showDeleteDialog"
      class="fixed inset-0 bg-black/30 backdrop-blur-lg flex items-center justify-center z-50"
      @click="showDeleteDialog = false"
    >
      <div
        class="glass-effect-lg rounded-xl p-6 max-w-md w-full mx-4"
        @click.stop
      >
        <h3 class="text-lg font-semibold text-gray-900 mb-2">确认删除</h3>
        <p class="text-gray-700 mb-6">确定要删除供应商 "{{ provider?.name }}" 吗？此操作无法撤销。</p>
        <div class="flex justify-end gap-3">
          <button
            @click="showDeleteDialog = false"
            class="px-4 py-2 text-gray-700 hover:bg-gray-100/50 rounded-lg transition-all duration-300 font-medium border border-gray-300"
          >
            取消
          </button>
          <button
            @click="confirmDelete"
            class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-all duration-300 font-medium"
          >
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useProviderStore } from '../../stores/provider'
import { storeToRefs } from 'pinia'
import { useToast } from '../../composables/useToast'
import type { Provider } from '../../types'

const router = useRouter()
const route = useRoute()
const providerStore = useProviderStore()
const { providers } = storeToRefs(providerStore)
const toast = useToast()

const providerId = route.params.id as string
const loading = ref(true)
const saving = ref(false)
const showApiKey = ref(false)
const showDeleteDialog = ref(false)
const tagInput = ref('')
const envName = ref('')
const envValue = ref('')

const provider = computed(() => 
  providers.value.find(p => p.id === providerId)
)

const form = ref({
  name: '',
  apiKey: '',
  baseUrl: '',
  icon: '',
  tags: [] as string[],
  models: {
    default: '',
    smallFast: '',
    opus: '',
    sonnet: '',
    haiku: '',
  },
  anthropicModel: '',
  anthropicSmallFastModel: '',
  customEnv: {} as Record<string, string | number>,
})

const isValid = computed(() => {
  return form.value.name.trim() !== '' && form.value.apiKey.trim() !== ''
})

onMounted(async () => {
  await providerStore.loadProviders()
  if (provider.value) {
    form.value = {
      name: provider.value.name,
      apiKey: provider.value.apiKey,
      baseUrl: provider.value.baseUrl,
      icon: provider.value.icon || '',
      tags: [...provider.value.tags],
      models: { ...provider.value.models },
      anthropicModel: provider.value.anthropicModel || '',
      anthropicSmallFastModel: provider.value.anthropicSmallFastModel || '',
      customEnv: { ...(provider.value.customEnv || {}) },
    }
  }
  loading.value = false
})

const addTag = () => {
  const tag = tagInput.value.trim()
  if (tag && !form.value.tags.includes(tag)) {
    form.value.tags.push(tag)
    tagInput.value = ''
  }
}

const removeTag = (index: number) => {
  form.value.tags.splice(index, 1)
}

const addCustomEnv = () => {
  const key = envName.value.trim()
  const value = envValue.value.trim()
  if (key && value) {
    form.value.customEnv[key] = value
    envName.value = ''
    envValue.value = ''
  }
}

const updateCustomEnvKey = (index: number, key: string) => {
  const newKey = key.trim()
  if (newKey) {
    const [oldKey] = Object.keys(form.value.customEnv).splice(index, 1)
    form.value.customEnv[newKey] = form.value.customEnv[oldKey]
    delete form.value.customEnv[oldKey]
  }
}

const updateCustomEnvValue = (index: number, value: string | number) => {
  const newKey = Object.keys(form.value.customEnv)[index]
  if (newKey) {
    form.value.customEnv[newKey] = String(value).trim()
  }
}

const removeCustomEnv = (index: number) => {
  const [key] = Object.keys(form.value.customEnv).splice(index, 1)
  delete form.value.customEnv[key]
}

const handleBack = () => {
  router.back()
}

const handleSave = async () => {
  if (!isValid.value || !provider.value) return

  try {
    saving.value = true
    const updatedProvider: Provider = {
      ...provider.value,
      name: form.value.name,
      apiKey: form.value.apiKey,
      baseUrl: form.value.baseUrl,
      icon: form.value.icon,
      tags: form.value.tags,
      models: form.value.models,
      anthropicModel: form.value.anthropicModel,
      anthropicSmallFastModel: form.value.anthropicSmallFastModel,
      customEnv: form.value.customEnv,
      updatedAt: Date.now(),
    }
    await providerStore.saveProvider(updatedProvider)
    toast.success('供应商更新成功')
    router.push('/providers')
  } catch (e) {
    console.error('Failed to save provider:', e)
    toast.error('保存失败：' + e)
  } finally {
    saving.value = false
  }
}

const confirmDelete = async () => {
  try {
    await providerStore.deleteProvider(providerId)
    toast.success('供应商删除成功')
    router.push('/providers')
  } catch (e) {
    console.error('Failed to delete provider:', e)
    toast.error('删除失败：' + e)
  }
}
</script>

