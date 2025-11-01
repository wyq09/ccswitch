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
          <h1 class="text-xl font-semibold text-gray-900">{{ t('providerForm.addTitle') }}</h1>
        </div>
        <button
          @click="handleSave"
          :disabled="!isValid || saving"
          class="px-6 py-2.5 bg-orange-500 hover:bg-orange-600 text-white rounded-xl font-medium transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ saving ? t('common.saving') : t('common.save') }}
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-auto p-8">
        <div class="space-y-6">
        <!-- Templates -->
        <div>
          <div class="flex items-center justify-between mb-3">
            <h2 class="text-base font-semibold text-gray-900">{{ t('providerForm.templates') }}</h2>
            <button class="flex items-center gap-1.5 px-3 py-1.5 bg-orange-50 text-primary hover:bg-orange-100 rounded-lg text-sm font-medium transition-colors">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
              </svg>
              {{ t('providerForm.viewDocs') }}
            </button>
          </div>
          <p class="text-sm text-gray-500 mb-5">{{ t('providerForm.templatesDesc') }}</p>
          
          <div class="grid grid-cols-5 gap-3">
            <button
              v-for="template in allTemplates"
              :key="template.id"
              @click="applyTemplate(template)"
              class="flex flex-col items-center gap-2 p-3 rounded-xl border-2 border-gray-200 hover:border-primary hover:bg-orange-50 transition-all"
            >
              <div class="w-12 h-12 flex items-center justify-center">
                <img 
                  v-if="template.icon.startsWith('/')" 
                  :src="template.icon" 
                  :alt="template.name"
                  class="w-full h-full object-contain"
                />
                <span v-else class="text-2xl">{{ template.icon }}</span>
              </div>
              <span class="text-xs text-gray-700 text-center leading-tight">{{ template.name }}</span>
            </button>
          </div>
        </div>

        <!-- Divider -->
        <div class="border-t border-gray-200"></div>

        <!-- Form -->
        <div class="space-y-5">
          <!-- Provider Name -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
              </svg>
              {{ t('providerForm.providerName') }}
            </label>
            <input
              v-model="form.name"
              type="text"
              :placeholder="t('providerForm.providerNamePlaceholder')"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- API Key -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
              </svg>
              {{ t('providerForm.apiKey') }}
              <span class="text-red-500">*</span>
            </label>
            <div class="relative">
              <input
                v-model="form.apiKey"
                :type="showApiKey ? 'text' : 'password'"
                :placeholder="t('providerForm.apiKeyPlaceholder')"
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

          <!-- API Endpoint -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
              </svg>
              {{ t('providerForm.apiEndpoint') }}
            </label>
            <input
              v-model="form.baseUrl"
              type="text"
              :placeholder="t('providerForm.apiEndpointPlaceholder')"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Haiku Model -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              Haiku 模型
            </label>
            <input
              v-model="form.models.haiku"
              type="text"
              placeholder="例如：claude-haiku-4"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- ANTHROPIC_MODEL -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              ANTHROPIC_MODEL
            </label>
            <input
              v-model="form.anthropicModel"
              type="text"
              placeholder="Claude 默认模型"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- ANTHROPIC_SMALL_FAST_MODEL -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              ANTHROPIC_SMALL_FAST_MODEL
            </label>
            <input
              v-model="form.anthropicSmallFastModel"
              type="text"
              placeholder="快速模型"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
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
              placeholder="glm-4.6"
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
              placeholder="glm-4.6"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <!-- Tags -->
          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
              </svg>
              {{ t('providerForm.tags') }}
            </label>
            <div class="flex gap-2 mb-3">
              <input
                v-model="tagInput"
                @keyup.enter="addTag"
                type="text"
                :placeholder="t('providerForm.tagsPlaceholder')"
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
            <div v-if="form.tags.length > 0" class="flex flex-wrap gap-2">
              <span
                v-for="(tag, index) in form.tags"
                :key="index"
                class="inline-flex items-center gap-2 px-3 py-1 bg-orange-100 text-primary rounded-full text-sm"
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useProviderStore } from '../../stores/provider'
import { storeToRefs } from 'pinia'
import { useToast } from '../../composables/useToast'
import type { Provider, Template } from '../../types'

const { t } = useI18n()
const router = useRouter()
const providerStore = useProviderStore()
const { allTemplates } = storeToRefs(providerStore)
const toast = useToast()

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

const tagInput = ref('')
const showApiKey = ref(false)
const saving = ref(false)
const envName = ref('')
const envValue = ref('')

const isValid = computed(() => {
  return form.value.name.trim() !== '' && form.value.apiKey.trim() !== ''
})

const applyTemplate = (template: Template) => {
  form.value.name = template.name
  form.value.baseUrl = template.baseUrl
  form.value.icon = template.icon
  form.value.models.default = template.defaultModels.sonnet
  form.value.models.smallFast = template.defaultModels.haiku
  form.value.models.opus = template.defaultModels.opus
  form.value.models.sonnet = template.defaultModels.sonnet
  form.value.models.haiku = template.defaultModels.haiku
}

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
  if (!isValid.value) return

  try {
    saving.value = true
    const provider: Provider = {
      id: Date.now().toString(),
      name: form.value.name,
      icon: form.value.icon,
      baseUrl: form.value.baseUrl,
      apiKey: form.value.apiKey,
      tags: form.value.tags,
      models: form.value.models,
      anthropicModel: form.value.anthropicModel,
      anthropicSmallFastModel: form.value.anthropicSmallFastModel,
      customEnv: form.value.customEnv,
      isActive: false,
      createdAt: Date.now(),
      updatedAt: Date.now(),
    }
    await providerStore.saveProvider(provider)
    toast.success(t('providerForm.saveSuccess'))
    router.push('/providers')
  } catch (e) {
    console.error('Failed to save provider:', e)
    toast.error(t('providerForm.saveError') + '：' + e)
  } finally {
    saving.value = false
  }
}
</script>

