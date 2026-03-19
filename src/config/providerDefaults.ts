import type { Provider, ProviderCustomEnv, ProviderFormState, Template } from '../types'

export const DEFAULT_CUSTOM_ENV: ProviderCustomEnv = {
  API_TIMEOUT_MS: '3000000',
  AUTOPILOT: 'true',
  CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC: '1',
  CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS: '1',
}

export function createDefaultProviderForm(): ProviderFormState {
  return {
    name: '',
    apiKey: '',
    baseUrl: '',
    icon: '',
    tags: [],
    models: {
      default: '',
      smallFast: '',
      opus: '',
      sonnet: '',
      haiku: '',
    },
    anthropicModel: '',
    anthropicSmallFastModel: '',
    customEnv: { ...DEFAULT_CUSTOM_ENV },
  }
}

export function applyTemplateToForm(form: ProviderFormState, template: Template): ProviderFormState {
  return {
    ...form,
    name: template.name,
    baseUrl: template.baseUrl,
    icon: template.icon,
    models: {
      default: template.defaultModels.sonnet,
      smallFast: template.defaultModels.haiku,
      opus: template.defaultModels.opus,
      sonnet: template.defaultModels.sonnet,
      haiku: template.defaultModels.haiku,
    },
  }
}

export function createFormFromProvider(provider: Provider, options?: { duplicate?: boolean }): ProviderFormState {
  const duplicate = options?.duplicate ?? false
  const suffix = duplicate ? ' - 副本' : ''

  return {
    name: `${provider.name}${suffix}`,
    apiKey: provider.apiKey,
    baseUrl: provider.baseUrl,
    icon: provider.icon || '',
    tags: [...provider.tags],
    models: { ...provider.models },
    anthropicModel: provider.anthropicModel || '',
    anthropicSmallFastModel: provider.anthropicSmallFastModel || '',
    customEnv: {
      ...DEFAULT_CUSTOM_ENV,
      ...(provider.customEnv || {}),
    },
  }
}
