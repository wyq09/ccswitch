import type {
  Project,
  ProjectFormState,
  Provider,
  TerminalToolPreset,
} from '../types'

const MODEL_ROLE_LABELS: Record<string, string> = {
  default: '默认',
  smallFast: '小快',
  opus: 'Opus',
  sonnet: 'Sonnet',
  haiku: 'Haiku',
}

export function createDefaultProjectForm(
  presets: TerminalToolPreset[] = [],
): ProjectFormState {
  const defaultPreset = presets[0]

  return {
    name: '',
    path: '',
    providerId: '',
    model: '',
    terminalTool: defaultPreset?.id || '',
    launchCommandTemplate: defaultPreset?.defaultCommandTemplate || '',
    terminalOpenCommandTemplate: defaultPreset?.defaultOpenCommandTemplate || '',
  }
}

export function createProjectFormFromProject(project: Project): ProjectFormState {
  return {
    name: project.name,
    path: project.path,
    providerId: project.providerId,
    model: project.model,
    terminalTool: project.terminalTool,
    launchCommandTemplate: project.launchCommandTemplate,
    terminalOpenCommandTemplate: project.terminalOpenCommandTemplate || '',
  }
}

export function getProviderModelOptions(provider?: Provider | null) {
  if (!provider) {
    return []
  }

  const rolesByModel = new Map<string, string[]>()
  const orderedRoles: Array<keyof Provider['models']> = [
    'default',
    'smallFast',
    'opus',
    'sonnet',
    'haiku',
  ]

  for (const role of orderedRoles) {
    const modelName = provider.models[role]
    if (!modelName) {
      continue
    }

    const roles = rolesByModel.get(modelName) || []
    roles.push(MODEL_ROLE_LABELS[role])
    rolesByModel.set(modelName, roles)
  }

  return Array.from(rolesByModel.entries()).map(([value, roles]) => ({
    value,
    label: `${value} (${roles.join(' / ')})`,
  }))
}

export function findTerminalToolPreset(
  presets: TerminalToolPreset[],
  toolId: string,
) {
  return presets.find((preset) => preset.id === toolId)
}
