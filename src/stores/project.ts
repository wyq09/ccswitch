import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  LaunchProjectResult,
  Project,
  TerminalToolPreset,
} from '../types'

export const useProjectStore = defineStore('project', () => {
  const projects = ref<Project[]>([])
  const terminalToolPresets = ref<TerminalToolPreset[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const sortedProjects = computed(() =>
    [...projects.value].sort((a, b) => b.updatedAt - a.updatedAt),
  )

  async function loadProjects() {
    try {
      loading.value = true
      error.value = null
      projects.value = await invoke<Project[]>('load_projects')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load projects:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveProject(project: Project) {
    try {
      loading.value = true
      error.value = null
      await invoke('save_project', { project })
      await loadProjects()
    } catch (e) {
      error.value = e as string
      console.error('Failed to save project:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteProject(id: string) {
    try {
      loading.value = true
      error.value = null
      await invoke('delete_project', { id })
      await loadProjects()
    } catch (e) {
      error.value = e as string
      console.error('Failed to delete project:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function loadTerminalToolPresets() {
    try {
      terminalToolPresets.value = await invoke<TerminalToolPreset[]>(
        'get_terminal_tool_presets',
      )
    } catch (e) {
      console.error('Failed to load terminal tool presets:', e)
      throw e
    }
  }

  async function launchProject(id: string) {
    try {
      loading.value = true
      error.value = null
      return await invoke<LaunchProjectResult>('launch_project', { id })
    } catch (e) {
      error.value = e as string
      console.error('Failed to launch project:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    projects,
    terminalToolPresets,
    loading,
    error,
    sortedProjects,
    loadProjects,
    saveProject,
    deleteProject,
    loadTerminalToolPresets,
    launchProject,
  }
})
