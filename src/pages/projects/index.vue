<template>
  <div class="h-full flex flex-col" style="background-color: #273B3E;">
    <header class="bg-white shadow-sm px-6 py-4 flex justify-between items-center">
      <h2 class="text-xl font-semibold text-gray-800">{{ t('projects.title') }}</h2>
      <div class="flex items-center gap-2">
        <button
          @click="handleAdd"
          class="px-4 py-2 text-white rounded-md transition-colors flex items-center"
          style="background-color: #F87102;"
          @mouseenter="handleButtonHover(true, $event)"
          @mouseleave="handleButtonHover(false, $event)"
        >
          <i class="fas fa-plus mr-2"></i>
          {{ t('projects.addProject') }}
        </button>
      </div>
    </header>

    <main class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="flex items-center justify-center h-64">
        <div class="text-gray-500">{{ t('common.loading') }}</div>
      </div>

      <div v-else-if="error" class="flex items-center justify-center h-64">
        <div class="text-red-500">{{ error }}</div>
      </div>

      <div
        v-else-if="sortedProjects.length === 0"
        class="flex flex-col items-center justify-center h-64"
      >
        <div class="text-gray-400 mb-4">
          <i class="fas fa-folder-open text-6xl"></i>
        </div>
        <p class="text-gray-500 mb-4">{{ t('projects.noProjects') }}</p>
        <button
          @click="handleAdd"
          class="px-6 py-2 text-white rounded-lg transition-colors"
          style="background-color: #F87102;"
          @mouseenter="handleButtonHover(true, $event)"
          @mouseleave="handleButtonHover(false, $event)"
        >
          {{ t('projects.addFirstProject') }}
        </button>
      </div>

      <div v-else class="space-y-4">
        <div
          v-for="project in sortedProjects"
          :key="project.id"
          class="bg-white rounded-2xl shadow-lg transition-all duration-300 hover:shadow-xl"
          :style="{
            boxShadow: '0 4px 15px -3px rgba(0, 0, 0, 0.1), 0 2px 6px -2px rgba(0, 0, 0, 0.05)',
          }"
        >
          <div class="p-6 flex items-center justify-between gap-6">
            <div class="min-w-0">
              <div class="flex items-center gap-3 mb-2">
                <h3 class="font-semibold text-lg text-gray-900 truncate">
                  {{ project.name }}
                </h3>
                <span class="px-2.5 py-1 rounded-full text-xs bg-orange-100 text-primary">
                  {{ getProviderName(project.providerId) }}
                </span>
              </div>
              <p class="text-sm text-gray-500 truncate">{{ project.path }}</p>
              <div class="flex flex-wrap gap-2 mt-3">
                <span class="px-2.5 py-1 rounded-full text-xs bg-gray-100 text-gray-700">
                  {{ getProjectModelLabel(project) }}
                </span>
                <span class="px-2.5 py-1 rounded-full text-xs bg-gray-100 text-gray-700">
                  {{ getToolName(project.terminalTool) }}
                </span>
              </div>
            </div>

            <div class="flex items-center gap-3">
              <button
                @click="handleLaunch(project.id)"
                class="w-10 h-10 rounded-full flex items-center justify-center text-primary hover:bg-orange-50 transition-colors"
                :title="t('projects.launch')"
              >
                <i class="fas fa-play"></i>
              </button>
              <button
                @click="handleEdit(project.id)"
                class="w-10 h-10 rounded-full flex items-center justify-center text-gray-400 hover:bg-gray-100 transition-colors"
                :title="t('common.edit')"
              >
                <i class="fas fa-ellipsis-v"></i>
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useProjectStore } from '../../stores/project'
import { useProviderStore } from '../../stores/provider'
import { useToast } from '../../composables/useToast'
import { getProviderModelOptions } from '../../config/projectDefaults'
import type { Project } from '../../types'

const { t } = useI18n()
const router = useRouter()
const projectStore = useProjectStore()
const providerStore = useProviderStore()
const toast = useToast()

const { sortedProjects, loading, error, terminalToolPresets } = storeToRefs(projectStore)
const { providers } = storeToRefs(providerStore)

onMounted(async () => {
  await Promise.all([
    providerStore.loadProviders(),
    projectStore.loadProjects(),
    projectStore.loadTerminalToolPresets(),
  ])
})

const providersById = computed(() => {
  return new Map(providers.value.map(provider => [provider.id, provider]))
})

const terminalToolsById = computed(() => {
  return new Map(terminalToolPresets.value.map(tool => [tool.id, tool]))
})

const handleAdd = () => {
  router.push('/projects/add')
}

const handleEdit = (id: string) => {
  router.push(`/projects/${id}/edit`)
}

const handleLaunch = async (id: string) => {
  try {
    const result = await projectStore.launchProject(id)
    toast.success(
      t('projects.launchSuccess', {
        name: result.projectName,
        tool: getToolName(result.terminalTool),
      }),
    )
  } catch (e) {
    toast.error(t('projects.launchError') + '：' + e)
  }
}

const getProviderName = (providerId: string) => {
  return providersById.value.get(providerId)?.name || t('projects.missingProvider')
}

const getProjectModelLabel = (project: Project) => {
  const provider = providersById.value.get(project.providerId)
  const matched = getProviderModelOptions(provider).find(option => option.value === project.model)
  return matched?.label || project.model
}

const getToolName = (toolId: string) => {
  return terminalToolsById.value.get(toolId)?.name || toolId
}

const handleButtonHover = (isEntering: boolean, event: MouseEvent) => {
  const target = event.currentTarget as HTMLElement | null
  if (target) {
    target.style.backgroundColor = isEntering ? '#E86600' : '#F87102'
  }
}
</script>
