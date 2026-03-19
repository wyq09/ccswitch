<template>
  <div class="h-full flex items-center justify-center bg-black/30 backdrop-blur-lg">
    <div class="glass-effect-lg rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col shadow-2xl">
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
          <h1 class="text-xl font-semibold text-gray-900">{{ t('projects.editTitle') }}</h1>
        </div>
        <div class="flex gap-2">
          <button
            @click="handleSave"
            :disabled="!isValid || saving"
            class="px-6 py-2.5 bg-orange-500 hover:bg-orange-600 text-white rounded-xl font-medium transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ saving ? t('common.saving') : t('common.save') }}
          </button>
          <button
            @click="showDeleteDialog = true"
            class="px-4 py-2.5 bg-red-600 hover:bg-red-700 text-white rounded-xl transition-all duration-300"
            :title="t('common.delete')"
          >
            <i class="fas fa-trash-alt"></i>
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-auto p-8">
        <div v-if="loading" class="flex items-center justify-center h-64">
          <div class="text-gray-500">{{ t('common.loading') }}</div>
        </div>

        <div v-else-if="!project" class="flex items-center justify-center h-64">
          <div class="text-red-500">{{ t('projects.projectNotFound') }}</div>
        </div>

        <div v-else class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ t('projects.projectName') }}
            </label>
            <input
              v-model="form.name"
              type="text"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              {{ t('projects.projectPath') }}
            </label>
            <div class="flex gap-2">
              <input
                v-model="form.path"
                type="text"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              />
              <button
                @click="pickDirectory"
                type="button"
                class="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
              >
                {{ t('projects.browse') }}
              </button>
            </div>
          </div>

          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-7 7-7-7" />
              </svg>
              {{ t('projects.provider') }}
            </label>
            <div class="relative">
              <select
                v-model="form.providerId"
                class="w-full appearance-none bg-white/90 px-4 py-2.5 pr-11 border border-gray-300 rounded-xl shadow-sm transition-all focus:ring-2 focus:ring-primary/20 focus:border-primary focus:bg-white"
              >
                <option value="" disabled>{{ t('projects.selectProvider') }}</option>
                <option v-for="providerItem in providers" :key="providerItem.id" :value="providerItem.id">
                  {{ providerItem.name }}
                </option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-4 text-gray-400">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </div>
            </div>
          </div>

          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              {{ t('projects.model') }}
            </label>
            <div class="relative">
              <select
                v-model="form.model"
                :disabled="modelOptions.length === 0"
                class="w-full appearance-none bg-white/90 px-4 py-2.5 pr-11 border border-gray-300 rounded-xl shadow-sm transition-all focus:ring-2 focus:ring-primary/20 focus:border-primary focus:bg-white disabled:bg-gray-100 disabled:text-gray-400 disabled:cursor-not-allowed"
              >
                <option value="" disabled>{{ t('projects.selectModel') }}</option>
                <option v-for="option in modelOptions" :key="option.value" :value="option.value">
                  {{ option.label }}
                </option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-4 text-gray-400">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </div>
            </div>
          </div>

          <div>
            <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-2">
              <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 3v2.25M14.25 3v2.25M4.5 8.25h15M6.75 21h10.5A2.25 2.25 0 0019.5 18.75V8.25A2.25 2.25 0 0017.25 6H6.75A2.25 2.25 0 004.5 8.25v10.5A2.25 2.25 0 006.75 21z" />
              </svg>
              {{ t('projects.terminalTool') }}
            </label>
            <div class="relative">
              <select
                v-model="form.terminalTool"
                class="w-full appearance-none bg-white/90 px-4 py-2.5 pr-11 border border-gray-300 rounded-xl shadow-sm transition-all focus:ring-2 focus:ring-primary/20 focus:border-primary focus:bg-white"
              >
                <option value="" disabled>{{ t('projects.selectTerminalTool') }}</option>
                <option v-for="tool in terminalToolPresets" :key="tool.id" :value="tool.id">
                  {{ tool.name }} - {{ tool.description }}
                </option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-4 text-gray-400">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </div>
            </div>
          </div>

          <div v-if="requiresOpenCommandTemplate">
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-medium text-gray-700">
                {{ t('projects.terminalOpenCommand') }}
              </label>
              <button
                @click="resetOpenCommandTemplate"
                type="button"
                class="text-sm text-primary hover:underline"
              >
                {{ t('projects.resetTerminalOpenCommand') }}
              </button>
            </div>
            <textarea
              v-model="form.terminalOpenCommandTemplate"
              rows="3"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent font-mono text-sm"
            />
            <p class="text-xs text-gray-500 mt-2">
              {{ t('projects.terminalOpenCommandHint') }}
            </p>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-medium text-gray-700">
                {{ t('projects.launchCommand') }}
              </label>
              <button
                @click="resetCommandTemplate"
                type="button"
                class="text-sm text-primary hover:underline"
              >
                {{ t('projects.resetCommand') }}
              </button>
            </div>
            <textarea
              v-model="form.launchCommandTemplate"
              rows="4"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent font-mono text-sm"
            />
            <p class="text-xs text-gray-500 mt-2">
              {{ t('projects.commandHint') }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <div
      v-if="showDeleteDialog"
      class="fixed inset-0 bg-black/30 backdrop-blur-lg flex items-center justify-center z-50"
      @click="showDeleteDialog = false"
    >
      <div class="glass-effect-lg rounded-xl p-6 max-w-md w-full mx-4" @click.stop>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">{{ t('common.delete') }}</h3>
        <p class="text-gray-700 mb-6">
          {{ t('projects.deleteConfirm', { name: project?.name || '' }) }}
        </p>
        <div class="flex justify-end gap-3">
          <button
            @click="showDeleteDialog = false"
            class="px-4 py-2 text-gray-700 hover:bg-gray-100/50 rounded-lg transition-all duration-300 font-medium border border-gray-300"
          >
            {{ t('common.cancel') }}
          </button>
          <button
            @click="confirmDelete"
            class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-all duration-300 font-medium"
          >
            {{ t('common.delete') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '../../stores/project'
import { useProviderStore } from '../../stores/provider'
import { useToast } from '../../composables/useToast'
import {
  createProjectFormFromProject,
  findTerminalToolPreset,
  getProviderModelOptions,
} from '../../config/projectDefaults'
import type { ProjectFormState } from '../../types'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()
const providerStore = useProviderStore()
const toast = useToast()

const projectId = route.params.id as string
const loading = ref(true)
const saving = ref(false)
const showDeleteDialog = ref(false)
const lastAutoCommandTemplate = ref('')
const lastAutoOpenCommandTemplate = ref('')

const { projects, terminalToolPresets } = storeToRefs(projectStore)
const { providers } = storeToRefs(providerStore)

const project = computed(() =>
  projects.value.find(projectItem => projectItem.id === projectId),
)

const form = ref<ProjectFormState>({
  name: '',
  path: '',
  providerId: '',
  model: '',
  terminalTool: '',
  launchCommandTemplate: '',
  terminalOpenCommandTemplate: '',
})

const selectedProvider = computed(() =>
  providers.value.find(provider => provider.id === form.value.providerId),
)

const selectedTerminalTool = computed(() =>
  findTerminalToolPreset(terminalToolPresets.value, form.value.terminalTool),
)

const modelOptions = computed(() => getProviderModelOptions(selectedProvider.value))
const requiresOpenCommandTemplate = computed(() =>
  Boolean(selectedTerminalTool.value?.requiresOpenCommandTemplate),
)

const isValid = computed(() => {
  return [
    form.value.name.trim(),
    form.value.path.trim(),
    form.value.providerId.trim(),
    form.value.model.trim(),
    form.value.terminalTool.trim(),
    form.value.launchCommandTemplate.trim(),
    requiresOpenCommandTemplate.value ? form.value.terminalOpenCommandTemplate.trim() : 'ok',
  ].every(Boolean)
})

onMounted(async () => {
  await Promise.all([
    providerStore.loadProviders(),
    projectStore.loadProjects(),
    projectStore.loadTerminalToolPresets(),
  ])

  if (project.value) {
    form.value = createProjectFormFromProject(project.value)
    lastAutoCommandTemplate.value = form.value.launchCommandTemplate
    lastAutoOpenCommandTemplate.value = form.value.terminalOpenCommandTemplate
  }

  loading.value = false
})

watch(
  () => form.value.providerId,
  () => {
    const provider = selectedProvider.value
    if (!provider) {
      form.value.model = ''
      return
    }

    const optionValues = getProviderModelOptions(provider).map(option => option.value)
    if (!optionValues.includes(form.value.model)) {
      form.value.model = provider.models.default
    }
  },
)

watch(
  () => form.value.terminalTool,
  (nextTool, previousTool) => {
    const previousPreset = previousTool
      ? findTerminalToolPreset(terminalToolPresets.value, previousTool)
      : undefined
    const nextPreset = findTerminalToolPreset(terminalToolPresets.value, nextTool)
    if (!nextPreset) {
      return
    }

    const previousDefault = previousPreset?.defaultCommandTemplate || lastAutoCommandTemplate.value
    if (
      !form.value.launchCommandTemplate.trim() ||
      form.value.launchCommandTemplate === previousDefault ||
      form.value.launchCommandTemplate === lastAutoCommandTemplate.value
    ) {
      form.value.launchCommandTemplate = nextPreset.defaultCommandTemplate
      lastAutoCommandTemplate.value = nextPreset.defaultCommandTemplate
    }

    const previousOpenDefault =
      previousPreset?.defaultOpenCommandTemplate || lastAutoOpenCommandTemplate.value
    if (
      !form.value.terminalOpenCommandTemplate.trim() ||
      form.value.terminalOpenCommandTemplate === previousOpenDefault ||
      form.value.terminalOpenCommandTemplate === lastAutoOpenCommandTemplate.value
    ) {
      form.value.terminalOpenCommandTemplate = nextPreset.defaultOpenCommandTemplate
      lastAutoOpenCommandTemplate.value = nextPreset.defaultOpenCommandTemplate
    }
  },
)

const pickDirectory = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
  })

  if (selected && !Array.isArray(selected)) {
    form.value.path = selected
  }
}

const resetCommandTemplate = () => {
  const preset = selectedTerminalTool.value
  if (!preset) {
    return
  }

  form.value.launchCommandTemplate = preset.defaultCommandTemplate
  lastAutoCommandTemplate.value = preset.defaultCommandTemplate
}

const resetOpenCommandTemplate = () => {
  const preset = selectedTerminalTool.value
  if (!preset) {
    return
  }

  form.value.terminalOpenCommandTemplate = preset.defaultOpenCommandTemplate
  lastAutoOpenCommandTemplate.value = preset.defaultOpenCommandTemplate
}

const handleBack = () => {
  router.back()
}

const handleSave = async () => {
  if (!project.value || !isValid.value) {
    return
  }

  try {
    saving.value = true
    await projectStore.saveProject({
      ...project.value,
      name: form.value.name,
      path: form.value.path,
      providerId: form.value.providerId,
      model: form.value.model,
      terminalTool: form.value.terminalTool,
      launchCommandTemplate: form.value.launchCommandTemplate,
      terminalOpenCommandTemplate: form.value.terminalOpenCommandTemplate,
      updatedAt: Date.now(),
    })
    toast.success(t('projects.saveSuccess'))
    router.push('/projects')
  } catch (e) {
    toast.error(t('projects.saveError') + '：' + e)
  } finally {
    saving.value = false
  }
}

const confirmDelete = async () => {
  try {
    await projectStore.deleteProject(projectId)
    toast.success(t('projects.deleteSuccess'))
    router.push('/projects')
  } catch (e) {
    toast.error(t('projects.deleteError') + '：' + e)
  }
}
</script>
