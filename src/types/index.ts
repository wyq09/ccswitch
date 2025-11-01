export type Provider = {
  id: string
  name: string
  icon?: string
  baseUrl: string
  apiKey: string
  tags: string[]
  models: {
    default: string
    smallFast: string
    opus: string
    sonnet: string
    haiku: string
  }
  anthropicModel?: string
  anthropicSmallFastModel?: string
  customEnv?: Record<string, string | number>  // 自定义环境变量
  isActive: boolean
  createdAt: number
  updatedAt: number
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
    CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC?: number
    ANTHROPIC_MODEL?: string
    ANTHROPIC_SMALL_FAST_MODEL?: string
    ANTHROPIC_DEFAULT_SONNET_MODEL?: string
    ANTHROPIC_DEFAULT_OPUS_MODEL?: string
    ANTHROPIC_DEFAULT_HAIKU_MODEL?: string
    [key: string]: any
  }
  [key: string]: any
}

