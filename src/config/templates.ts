import type { Template } from '../types'

export const BUILTIN_TEMPLATES: Template[] = [
  {
    id: 'claude-default',
    name: 'Claude 默认',
    icon: '/src/assets/logo/Claude.svg',
    baseUrl: 'https://api.anthropic.com',
    defaultModels: {
      opus: 'claude-opus-4',
      sonnet: 'claude-sonnet-4.5',
      haiku: 'claude-haiku-4',
    },
  },
  {
    id: 'zhipu-ai',
    name: '智谱 AI',
    icon: '/src/assets/logo/Zhipu.svg',
    baseUrl: 'https://open.bigmodel.cn/api/anthropic',
    defaultModels: {
      opus: 'glm-4.6',
      sonnet: 'glm-4.6',
      haiku: 'glm-4.5-air',
    },
  },
  {
    id: 'moonshot',
    name: '月之暗面',
    icon: '/src/assets/logo/Kimi.svg',
    baseUrl: 'https://api.moonshot.cn/anthropic',
    defaultModels: {
      opus: 'kimi-k2-0905-preview',
      sonnet: 'kimi-k2-0905-preview',
      haiku: 'kimi-k2-0905-preview',
    },
  },
  {
    id: 'deepseek',
    name: 'DeepSeek',
    icon: '/src/assets/logo/deepseek.svg',
    baseUrl: 'https://api.deepseek.com/anthropic',
    defaultModels: {
      opus: 'deepseek-chat',
      sonnet: 'deepseek-chat',
      haiku: 'deepseek-chat',
    },
  },
  {
    id: 'qiniu',
    name: '七牛云',
    icon: '/src/assets/logo/qiniu.svg',
    baseUrl: 'https://openai.qiniu.com',
    defaultModels: {
      opus: 'MiniMax-M2',
      sonnet: 'MiniMax-M2',
      haiku: 'MiniMax-M2',
    },
  },
  {
    id: 'minimax',
    name: 'MiniMax',
    icon: '/src/assets/logo/minimax.svg',
    baseUrl: 'https://api.minimax.chat/anthropic',
    defaultModels: {
      opus: 'MiniMax-M2',
      sonnet: 'MiniMax-M2',
      haiku: 'MiniMax-M2',
    },
  },
  {
    id: 'qwen',
    name: '通义千问',
    icon: '/src/assets/logo/qwen.svg',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    defaultModels: {
      opus: 'qwen-max',
      sonnet: 'qwen-plus',
      haiku: 'qwen-turbo',
    },
  },
  {
    id: 'openrouter',
    name: 'OpenRouter',
    icon: '/src/assets/logo/OpenRouter.svg',
    baseUrl: 'https://openrouter.ai/api/v1',
    defaultModels: {
      opus: 'anthropic/claude-opus-4',
      sonnet: 'anthropic/claude-sonnet-4',
      haiku: 'anthropic/claude-haiku-4',
    },
  }
]

