export type ProviderModels = {
  default: string
  smallFast: string
  opus: string
  sonnet: string
  haiku: string
}

export type ProviderCustomEnv = Record<string, string | number>

export type Provider = {
  id: string
  name: string
  icon?: string
  baseUrl: string
  apiKey: string
  tags: string[]
  models: ProviderModels
  anthropicModel?: string
  anthropicSmallFastModel?: string
  customEnv?: ProviderCustomEnv
  isActive: boolean
  createdAt: number
  updatedAt: number
}

export type ProviderFormState = {
  name: string
  apiKey: string
  baseUrl: string
  icon: string
  tags: string[]
  models: ProviderModels
  anthropicModel: string
  anthropicSmallFastModel: string
  customEnv: ProviderCustomEnv
}

export type Project = {
  id: string
  name: string
  path: string
  providerId: string
  model: string
  terminalTool: string
  launchCommandTemplate: string
  terminalOpenCommandTemplate: string
  createdAt: number
  updatedAt: number
}

export type ProjectFormState = {
  name: string
  path: string
  providerId: string
  model: string
  terminalTool: string
  launchCommandTemplate: string
  terminalOpenCommandTemplate: string
}

export type TerminalToolPreset = {
  id: string
  name: string
  description: string
  defaultCommandTemplate: string
  defaultOpenCommandTemplate: string
  requiresOpenCommandTemplate: boolean
}

export type LaunchProjectResult = {
  command: string
  terminalTool: string
  providerName: string
  projectName: string
}

export type Template = {
  id: string
  name: string
  icon: string
  baseUrl: string
  defaultModels: {
    opus: string
    sonnet: string
    haiku: string
  }
}

export type ClaudeSettings = {
  env: {
    ANTHROPIC_BASE_URL?: string
    ANTHROPIC_AUTH_TOKEN?: string
    API_TIMEOUT_MS?: string
    AUTOPILOT?: string
    CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC?: string | number
    CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS?: string
    ANTHROPIC_MODEL?: string
    ANTHROPIC_SMALL_FAST_MODEL?: string
    ANTHROPIC_DEFAULT_SONNET_MODEL?: string
    ANTHROPIC_DEFAULT_OPUS_MODEL?: string
    ANTHROPIC_DEFAULT_HAIKU_MODEL?: string
    [key: string]: any
  }
  [key: string]: any
}
